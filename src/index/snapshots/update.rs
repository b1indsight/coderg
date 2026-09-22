//! Staged snapshot refresh: identify worktree versions, construct the commit
//! baseline, install the overlay, then publish under the caller's writer lock.
use super::*;
use std::borrow::Cow;

#[path = "baseline.rs"]
mod baseline;
use baseline::{Baseline, build_baseline};

/// Versions of the scanned worktree and their relation to the previous view.
/// Buffered contents share the same checked bytes used to calculate OIDs.
struct Worktree<'a> {
    current: Vec<FileState>,
    ids: HashMap<PathBuf, u32>,
    old_documents: Cow<'a, [Document]>,
    working_oids: Vec<Option<BlobId>>,
    previous_oids: Vec<Option<BlobId>>,
    changed: Vec<bool>,
    contents: InitialContents,
    initial: bool,
}

struct OverlaySummary {
    indexed: usize,
    reused: usize,
    compaction: compaction::Summary,
}

/// The caller holds write.lock throughout preparation and publication. All
/// content extraction finishes before sidecars and the active manifest are
/// published; GC runs only after the new active manifest has been installed.
pub(super) fn update(
    directory: &Path,
    budget: MemoryBudget,
    mut manifest: Manifest,
    current: Vec<FileState>,
    previous: Option<&Manifest>,
    validated: &[segment::Segment],
) -> Result<(Manifest, usize, usize, compaction::Summary)> {
    let initial = previous.is_none();
    // refresh already checked this manifest under the writer lock.
    let cached = previous
        .map(|m| state_for(directory, m))
        .transpose()?
        .flatten();
    let mut state = cached.unwrap_or(State {
        version: CACHE_VERSION,
        epoch: next_segment_id(directory),
        manifest: [0; 20],
        oids: vec![None; manifest.documents.len()],
        trees: Vec::new(),
        timeline: Vec::new(),
        roots: HashMap::new(),
        registries: HashMap::new(),
        registry_counts: HashMap::new(),
        history_watermark: 0,
        segment_bytes: HashMap::new(),
    });
    let old_documents = previous.map_or_else(
        || Cow::Owned(manifest.documents.clone()),
        |m| Cow::Borrowed(m.documents.as_slice()),
    );
    let ids = extend_registry(&mut manifest, &current, &mut state)?;
    let worktree = Worktree::read(
        &manifest,
        current,
        ids,
        std::mem::take(&mut state.oids),
        old_documents,
        budget,
        initial,
    )?;
    let mut baseline = build_baseline(directory, budget, &manifest, &state, &worktree, validated)?;
    let overlay = install_overlay(directory, budget, &mut manifest, &worktree, &mut baseline)?;
    manifest.source_state = worktree.current;
    if !initial {
        manifest.generation += 1;
    }
    state.oids = worktree.working_oids;
    publish(
        directory,
        &mut manifest,
        &mut state,
        &baseline.base,
        baseline.changed,
        baseline.durable,
    )?;
    Ok((
        manifest,
        overlay.indexed,
        overlay.reused,
        overlay.compaction,
    ))
}

/// Append new paths without changing any existing document ID or registry prefix.
fn extend_registry(
    manifest: &mut Manifest,
    current: &[FileState],
    state: &mut State,
) -> Result<HashMap<PathBuf, u32>> {
    let mut registry = manifest
        .registry
        .clone()
        .filter(|r| r.epoch == state.epoch && state.registries.get(&r.count) == Some(&r.digest))
        .unwrap_or(crate::manifest::RegistryIdentity {
            epoch: state.epoch,
            count: 0,
            digest: [0; 20],
        });
    let mut ids: HashMap<_, _> = manifest
        .documents
        .iter()
        .enumerate()
        .map(|(id, doc)| (doc.path.clone(), id as u32))
        .collect();
    for source in current {
        if !ids.contains_key(&source.path) {
            let id = u32::try_from(manifest.documents.len()).context("too many documents")?;
            ids.insert(source.path.clone(), id);
            manifest.documents.push(Document {
                path: source.path.clone(),
                len: source.len,
                modified_nanos: source.modified_nanos,
                active: false,
                searchable: false,
                segment_id: 0,
            });
        }
    }
    for document in &manifest.documents[registry.count..] {
        let mut bytes = registry.digest.to_vec();
        bytes.extend(bincode::serde::encode_to_vec(&document.path, config())?);
        registry.digest = blob_id(Oid::hash_object(ObjectType::Blob, &bytes)?);
        registry.count += 1;
    }
    state.registries.insert(registry.count, registry.digest);
    manifest.registry = Some(registry.clone());
    Ok(ids)
}

impl<'a> Worktree<'a> {
    /// Hash changed sources with metadata checks around each read. During legacy
    /// migration, old versions are reusable only when their metadata still matches.
    fn read(
        manifest: &Manifest,
        current: Vec<FileState>,
        ids: HashMap<PathBuf, u32>,
        old_oids: Vec<Option<BlobId>>,
        old_documents: Cow<'a, [Document]>,
        budget: MemoryBudget,
        initial: bool,
    ) -> Result<Self> {
        let old_states: HashMap<_, _> = manifest
            .source_state
            .iter()
            .map(|s| (s.path.as_path(), s))
            .collect();
        let mut contents = InitialContents::default();
        let mut buffered = HashSet::new();
        if initial {
            let limit =
                (budget.record_bytes(budget.workers(current.len())) / 4).min(32 * 1024 * 1024);
            // Favor small files to eliminate more opens and metadata operations.
            let mut order: Vec<_> = current.iter().collect();
            order.sort_unstable_by_key(|source| source.len);
            for source in order {
                if source.len > 1024 * 1024 {
                    break;
                }
                let bytes = source.len as usize + 1;
                if contents.reserved + bytes > limit {
                    break;
                }
                contents.reserved += bytes;
                buffered.insert(ids[&source.path] as usize);
            }
            contents.files = (0..manifest.documents.len()).map(|_| None).collect();
        }
        let hashes: Vec<_> = current
            .par_iter()
            .map(|source| -> Result<_> {
                let id = ids[&source.path] as usize;
                let unchanged = old_states
                    .get(source.path.as_path())
                    .is_some_and(|old| *old == source);
                let mut bytes = None;
                let oid = if unchanged && let Some(oid) = old_oids.get(id).copied().flatten() {
                    oid
                } else {
                    check_source(&manifest.root, source)?;
                    if buffered.contains(&id) {
                        let mut data = Vec::with_capacity(source.len as usize + 1);
                        File::open(manifest.root.join(&source.path))?
                            .take(source.len + 1)
                            .read_to_end(&mut data)?;
                        check_source(&manifest.root, source)?;
                        anyhow::ensure!(
                            data.len() as u64 == source.len,
                            "file changed while reading: {}",
                            source.path.display()
                        );
                        let oid = blob_id(Oid::hash_object(ObjectType::Blob, &data)?);
                        bytes = Some(data.into_boxed_slice());
                        oid
                    } else {
                        let oid = blob_id(Oid::hash_file(
                            ObjectType::Blob,
                            manifest.root.join(&source.path),
                        )?);
                        check_source(&manifest.root, source)?;
                        oid
                    }
                };
                Ok((id, oid, unchanged, bytes))
            })
            .collect::<Result<_>>()?;
        let mut working_oids = vec![None; manifest.documents.len()];
        // Legacy migration can reuse a document only if its old metadata still
        // describes the bytes just hashed. Never hash new bytes as an old version.
        let mut previous_oids = old_oids;
        previous_oids.resize(manifest.documents.len(), None);
        let mut changed = vec![false; manifest.documents.len()];
        for (id, oid, unchanged, bytes) in hashes {
            changed[id] = !unchanged;
            if let Some(slot) = contents.files.get_mut(id) {
                *slot = bytes;
            }
            working_oids[id] = Some(oid);
            if unchanged && old_documents.get(id).is_some_and(|d| d.active) {
                previous_oids[id] = Some(oid);
            }
        }
        Ok(Self {
            current,
            ids,
            old_documents,
            working_oids,
            previous_oids,
            changed,
            contents,
            initial,
        })
    }
}

/// Select live worktree versions and compact only newly extracted overlays.
fn install_overlay(
    directory: &Path,
    budget: MemoryBudget,
    manifest: &mut Manifest,
    worktree: &Worktree<'_>,
    baseline: &mut Baseline,
) -> Result<OverlaySummary> {
    let Worktree {
        current,
        ids,
        old_documents,
        working_oids,
        previous_oids,
        contents,
        ..
    } = worktree;
    let Baseline {
        git_dir,
        base,
        metas,
        base_ids,
        indexed: base_indexed,
        durable,
        ..
    } = baseline;
    for document in &mut manifest.documents {
        document.active = false;
    }
    let mut pending_work = Vec::new();
    let mut reused = 0;
    for source in current {
        let id = ids[&source.path] as usize;
        let oid = working_oids[id].unwrap();
        let reusable = base.documents[id]
            .as_ref()
            .filter(|cached| cached.oid == oid)
            .map(|cached| &cached.document)
            .or_else(|| {
                old_documents
                    .get(id)
                    .filter(|doc| doc.active && previous_oids[id] == Some(oid))
            });
        if let Some(document) = reusable {
            let active = &mut manifest.documents[id];
            active.active = true;
            active.len = source.len;
            active.modified_nanos = source.modified_nanos;
            active.searchable = document.searchable;
            active.segment_id = document.segment_id;
            if worktree.changed[id] {
                reused += 1;
            }
        } else {
            pending_work.push(Source {
                id: id as u32,
                state: source.clone(),
                blob: None,
            });
        }
    }
    if let Some((files, meta)) = extract(
        &manifest.root,
        git_dir,
        directory,
        budget,
        &pending_work,
        *durable,
        contents,
    )? {
        for file in files {
            let id = file.id as usize;
            manifest.documents[id] = document(file, meta.id);
        }
        metas.insert(meta.id, meta);
    }
    let indexed = *base_indexed + pending_work.len();
    let active_ids: HashSet<_> = manifest
        .documents
        .iter()
        .filter(|d| d.active && d.searchable)
        .map(|d| d.segment_id)
        .collect();
    manifest.segments = std::mem::take(metas)
        .into_values()
        .filter(|s| active_ids.contains(&s.id))
        .collect();
    manifest.segments.sort_unstable_by_key(|s| s.id);
    // Keep the commit baseline separate. Compact only the workspace overlay,
    // so a large abandoned edit never rewrites its immutable baseline.
    let overlay: Vec<_> = manifest
        .segments
        .iter()
        .filter(|s| !base_ids.contains(&s.id))
        .map(|s| s.id)
        .collect();
    let summary = if !pending_work.is_empty() && overlay.len() > MAX_OVERLAY_SEGMENTS {
        let inputs = &overlay[..overlay.len().min(compaction::MAX_MERGE_INPUTS)];
        compaction::merge(directory, manifest, inputs, next_segment_id(directory))?
    } else {
        compaction::Summary::default()
    };
    Ok(OverlaySummary {
        indexed,
        reused,
        compaction: summary,
    })
}

/// Publish sidecars before the active manifest, then reclaim against the new
/// root set. An interrupted sidecar publication is rejected by publication ID.
fn publish(
    directory: &Path,
    manifest: &mut Manifest,
    state: &mut State,
    base: &Snapshot,
    base_changed: bool,
    durable: bool,
) -> Result<()> {
    let tree_id = base.tree.clone();
    // Historical consolidation is driven by retained bytes, never by commit
    // count. Promotion and cached checkout remain reference-only operations.
    // Foreground refresh only records immutable history sizes. Consolidation
    // runs in an explicit independent compact-history maintenance process.
    for meta in &base.segments {
        for file in [&meta.lookup, &meta.postings] {
            if !state.segment_bytes.contains_key(file) {
                state
                    .segment_bytes
                    .insert(file.clone(), fs::metadata(directory.join(file))?.len());
            }
        }
    }
    if state.history_watermark == 0 {
        state.history_watermark = state.segment_bytes.values().sum::<u64>().max(1);
    }
    let encoded_manifest = manifest::encode(manifest)?;
    manifest.publication = manifest::publication(&encoded_manifest);
    state.manifest = manifest
        .publication
        .context("missing publication identity")?;
    state.trees.retain(|t| *t != tree_id);
    state.trees.push(tree_id.clone());
    if !state.timeline.contains(&tree_id) {
        state.timeline.push(tree_id.clone());
    }
    if base_changed || !snapshot_path(directory, state.epoch, &tree_id).exists() {
        write(&snapshot_path(directory, state.epoch, &tree_id), base)?;
    }
    state
        .registry_counts
        .insert(tree_id.clone(), base.registry.count);
    state.roots.insert(
        tree_id.clone(),
        base.segments
            .iter()
            .flat_map(|s| [s.lookup.clone(), s.postings.clone()])
            .collect(),
    );
    write(&state_path(directory), state)?;
    write_manifest_bytes(
        &directory.join(manifest::FILE_NAME),
        &encoded_manifest,
        durable,
    )?;
    collect_garbage_with_state(directory, manifest, state)?;
    Ok(())
}
