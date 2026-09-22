//! Resolve and build immutable commit baselines independently of worktree overlays.
use super::*;

/// A completed immutable baseline, plus segment metadata needed by the overlay.
pub(super) struct Baseline {
    pub(super) git_dir: PathBuf,
    pub(super) base: Snapshot,
    pub(super) metas: HashMap<u64, SegmentMeta>,
    pub(super) base_ids: HashSet<u64>,
    pub(super) changed: bool,
    pub(super) indexed: usize,
    pub(super) durable: bool,
}

/// Reuse cached commit versions and extract only missing baseline content.
/// The returned baseline is complete before it is used to select worktree versions.
pub(super) fn build_baseline(
    directory: &Path,
    budget: MemoryBudget,
    manifest: &Manifest,
    state: &State,
    worktree: &Worktree<'_>,
    validated: &[segment::Segment],
) -> Result<Baseline> {
    let Worktree {
        current,
        ids,
        old_documents,
        working_oids,
        previous_oids,
        contents,
        ..
    } = worktree;
    let initial = worktree.initial;
    let registry = manifest
        .registry
        .as_ref()
        .context("missing document registry")?;
    let current_by_id: HashMap<_, _> = current.iter().map(|s| (ids[&s.path] as usize, s)).collect();
    let tree_id = manifest.git_tree.clone().context("missing Git tree")?;
    let cached_base = snapshot_for(directory, state, &tree_id, manifest, validated);
    let base_missing = cached_base.is_none();
    let mut base = cached_base.unwrap_or(Snapshot {
        version: CACHE_VERSION,
        root: manifest.root.clone(),
        epoch: state.epoch,
        tree: tree_id.clone(),
        registry: registry.clone(),
        documents: Vec::new(),
        segments: Vec::new(),
    });
    let mut prior_base = state
        .trees
        .last()
        .filter(|tree| base_missing && **tree != tree_id)
        .and_then(|tree| snapshot_for(directory, state, tree, manifest, validated));
    let repo = Repository::discover(&manifest.root)?;
    let tree = repo.find_tree(Oid::from_str(&tree_id)?)?;
    let workdir = repo
        .workdir()
        .context("Git repository has no worktree")?
        .canonicalize()?;
    let prefix = manifest
        .root
        .strip_prefix(&workdir)
        .context("search root is outside Git worktree")?;
    let mut metas: HashMap<_, _> = manifest
        .segments
        .iter()
        .chain(&base.segments)
        .chain(prior_base.iter().flat_map(|base| &base.segments))
        .map(|meta| (meta.id, meta.clone()))
        .collect();
    // Diff paths are relative to the searched subtree, so they map directly
    // to document IDs without repository-prefix allocations per document.
    let target_tree = scoped_tree(&repo, prefix, &tree)?;
    let mut changes = if initial {
        initial_tree_oids(&repo, target_tree.as_ref(), &manifest.documents)?
    } else {
        HashMap::new()
    };
    let mut displaced = HashMap::new();
    let mut base_changed = false;
    if base_missing
        && let Some(prior) = &mut prior_base
        && let Ok(prior_tree) = repo.find_tree(Oid::from_str(&prior.tree)?)
    {
        let prior_tree = scoped_tree(&repo, prefix, &prior_tree)?;
        changes.extend(tree_changes(
            &repo,
            prior_tree.as_ref(),
            target_tree.as_ref(),
            ids,
        )?);
        // The prior snapshot remains on disk. Transfer its validated mapping
        // in memory; unchanged entries require neither cloning nor hashing.
        base.documents = std::mem::take(&mut prior.documents);
        for &id in changes.keys() {
            if let Some(slot) = base.documents.get_mut(id)
                && let Some(cached) = slot.take()
            {
                displaced.insert(id, cached);
            }
        }
        base_changed = true;
    }
    base_changed |= base.registry != *registry;
    base.registry = registry.clone();
    base.documents.resize(manifest.documents.len(), None);
    let mut pending_base = Vec::new();
    for (id, doc) in manifest.documents.iter().enumerate() {
        if base.documents[id].is_some() {
            continue;
        }
        // Only unresolved entries need a path lookup. Older manifests can
        // contain duplicate IDs; retain the latest registry entry.
        if ids[&doc.path] as usize != id {
            continue;
        }
        let git_oid = match changes.get(&id) {
            Some(Some(oid)) => *oid,
            Some(None) => continue,
            None => {
                let Some(tree) = &target_tree else { continue };
                match tree.get_path(&doc.path) {
                    Ok(entry) if matches!(entry.filemode(), 0o100644 | 0o100755) => entry.id(),
                    Ok(_) => continue,
                    Err(e) if e.code() == git2::ErrorCode::NotFound => continue,
                    Err(e) => return Err(e.into()),
                }
            }
        };
        let oid = blob_id(git_oid);
        let reusable = old_documents
            .get(id)
            .filter(|doc| doc.active && previous_oids[id] == Some(oid))
            .cloned()
            .or_else(|| {
                displaced
                    .get(&id)
                    .or_else(|| {
                        prior_base
                            .as_ref()
                            .and_then(|base| base.documents.get(id))
                            .and_then(Option::as_ref)
                    })
                    .filter(|cached| cached.oid == oid)
                    .map(|cached| cached.document.clone())
            });
        if let Some(document) = reusable {
            base.documents[id] = Some(CachedDocument { oid, document });
        } else {
            let source = if working_oids[id] == Some(oid) {
                Source {
                    id: id as u32,
                    state: (*current_by_id[&id]).clone(),
                    blob: None,
                }
            } else {
                let blob = repo.find_blob(git_oid)?;
                Source {
                    id: id as u32,
                    state: FileState {
                        path: doc.path.clone(),
                        len: blob.size() as u64,
                        modified_nanos: 0,
                    },
                    blob: Some(oid),
                }
            };
            pending_base.push(source);
            // Filled with the new segment reference after extraction.
            base.documents[id] = Some(CachedDocument {
                oid,
                document: doc.clone(),
            });
        }
        base_changed = true;
    }
    let durable = initial || !compaction::small_base(directory, manifest)?;
    if let Some((files, meta)) = extract(
        &manifest.root,
        repo.path(),
        directory,
        budget,
        &pending_base,
        durable,
        contents,
    )? {
        for file in files {
            let id = file.id as usize;
            base.documents[id].as_mut().unwrap().document = document(file, meta.id);
        }
        metas.insert(meta.id, meta);
    }
    let base_ids: HashSet<_> = base
        .documents
        .iter()
        .flatten()
        .filter(|d| d.document.searchable)
        .map(|d| d.document.segment_id)
        .collect();
    base.segments = metas
        .values()
        .filter(|s| base_ids.contains(&s.id))
        .cloned()
        .collect();
    base.segments.sort_unstable_by_key(|s| s.id);

    Ok(Baseline {
        git_dir: repo.path().to_owned(),
        base,
        metas,
        base_ids,
        changed: base_changed,
        indexed: pending_base.len(),
        durable,
    })
}

/// Scope both sides of a diff to the indexed subtree. Missing or non-tree
/// entries represent an absent subtree, including file/directory replacements.
fn scoped_tree<'repo>(
    repo: &'repo Repository,
    prefix: &Path,
    tree: &git2::Tree<'repo>,
) -> Result<Option<git2::Tree<'repo>>> {
    if prefix.as_os_str().is_empty() {
        return Ok(Some(repo.find_tree(tree.id())?));
    }
    match tree.get_path(prefix) {
        Ok(entry) if entry.kind() == Some(ObjectType::Tree) => {
            Ok(Some(repo.find_tree(entry.id())?))
        }
        Ok(_) => Ok(None),
        Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Map tree changes to stable document IDs, explicitly invalidating deleted
/// paths and transitions from regular files to symlinks or other entry types.
fn tree_changes(
    repo: &Repository,
    prior: Option<&git2::Tree<'_>>,
    target: Option<&git2::Tree<'_>>,
    ids: &HashMap<PathBuf, u32>,
) -> Result<HashMap<usize, Option<Oid>>> {
    let mut changes = HashMap::new();
    if prior.is_none() && target.is_none() {
        return Ok(changes);
    }
    let diff = repo.diff_tree_to_tree(prior, target, None)?;
    for delta in diff.deltas() {
        if let Some(path) = delta.old_file().path()
            && let Some(&id) = ids.get(path)
        {
            changes.insert(id as usize, None);
        }
        let file = delta.new_file();
        if let Some(path) = file.path()
            && let Some(&id) = ids.get(path)
        {
            let oid = match file.mode() {
                git2::FileMode::Blob | git2::FileMode::BlobExecutable if !file.id().is_zero() => {
                    Some(file.id())
                }
                _ => None,
            };
            changes.insert(id as usize, oid);
        }
    }
    Ok(changes)
}

/// Resolve initial regular-file OIDs with one Git tree lookup per directory.
fn initial_tree_oids(
    repo: &Repository,
    tree: Option<&git2::Tree<'_>>,
    documents: &[Document],
) -> Result<HashMap<usize, Option<Oid>>> {
    fn directory(
        repo: &Repository,
        path: &Path,
        cache: &mut HashMap<PathBuf, Option<Oid>>,
    ) -> Result<Option<Oid>> {
        if let Some(oid) = cache.get(path) {
            return Ok(*oid);
        }
        let parent = directory(repo, path.parent().unwrap_or(Path::new("")), cache)?;
        let oid = if let Some(parent) = parent {
            let tree = repo.find_tree(parent)?;
            match tree.get_path(Path::new(path.file_name().unwrap())) {
                Ok(entry) if entry.kind() == Some(ObjectType::Tree) => Some(entry.id()),
                Ok(_) => None,
                Err(e) if e.code() == git2::ErrorCode::NotFound => None,
                Err(e) => return Err(e.into()),
            }
        } else {
            None
        };
        cache.insert(path.to_owned(), oid);
        Ok(oid)
    }
    let mut directories = HashMap::from([(PathBuf::new(), tree.map(|t| t.id()))]);
    let mut grouped: HashMap<&Path, Vec<usize>> = HashMap::new();
    for (id, doc) in documents.iter().enumerate() {
        grouped
            .entry(doc.path.parent().unwrap_or(Path::new("")))
            .or_default()
            .push(id);
    }
    let mut result = HashMap::with_capacity(documents.len());
    for (parent, ids) in grouped {
        let tree = directory(repo, parent, &mut directories)?
            .map(|oid| repo.find_tree(oid))
            .transpose()?;
        for id in ids {
            let oid = if let Some(tree) = &tree {
                match tree.get_path(Path::new(documents[id].path.file_name().unwrap())) {
                    Ok(entry) if matches!(entry.filemode(), 0o100644 | 0o100755) => {
                        Some(entry.id())
                    }
                    Ok(_) => None,
                    Err(e) if e.code() == git2::ErrorCode::NotFound => None,
                    Err(e) => return Err(e.into()),
                }
            } else {
                None
            };
            result.insert(id, oid);
        }
    }
    Ok(result)
}
