use super::*;
use std::time::Instant;

#[derive(Serialize)]
pub struct HistorySummary {
    pub published: bool,
    pub retired_tree: String,
    pub baseline_tree: String,
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub prepare_ms: f64,
    pub merge_ms: f64,
    pub remap_ms: f64,
    pub publish_wait_ms: f64,
    pub publish_ms: f64,
    pub total_ms: f64,
}

/// One explicit maintenance job. Input hard links survive concurrent GC;
/// expensive merging and snapshot encoding run without the writer lock.
pub(in crate::index) fn compact_oldest(
    path: &Path,
    requested_index_dir: Option<&Path>,
) -> Result<HistorySummary> {
    compact_with_hook(path, requested_index_dir, || Ok(()))
}

fn compact_with_hook(
    path: &Path,
    requested_index_dir: Option<&Path>,
    before_publish: impl FnOnce() -> Result<()>,
) -> Result<HistorySummary> {
    let started = Instant::now();
    let root = resolve_root(path)?;
    let directory = resolve_index_dir(&root, requested_index_dir);
    let maintenance = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(directory.join("history.lock"))?;
    maintenance.lock()?;
    let stage = tempfile::Builder::new()
        .prefix("history-job-")
        .tempdir_in(&directory)?;
    fs::create_dir(stage.path().join("segments"))?;
    fs::create_dir(stage.path().join("snapshots"))?;
    let writer = writer_lock(&directory)?;
    let mut manifest = load(&root, requested_index_dir)?.into_manifest()?;
    let original_publication = manifest.publication;
    let mut state = state_for(&directory, &manifest)?.context("no valid Git snapshot state")?;
    let oldest = state
        .timeline
        .first()
        .context("no cached baselines")?
        .clone();
    // Only advance along an actual parent edge on the indexed HEAD lineage.
    // A sibling branch is never mistaken for an incremental successor.
    let repo = Repository::discover(&root)?;
    let mut commit = repo.find_commit(Oid::from_str(
        manifest
            .git_head
            .as_deref()
            .context("missing indexed HEAD")?,
    )?)?;
    let mut child = None;
    while commit.tree_id().to_string() != oldest {
        child = Some(commit.tree_id().to_string());
        anyhow::ensure!(
            commit.parent_count() > 0,
            "oldest cached tree is not on the indexed HEAD first-parent lineage"
        );
        commit = commit.parent(0)?;
    }
    let target = child.context("no successor to the oldest cached baseline")?;
    anyhow::ensure!(
        state.trees.contains(&target),
        "successor tree has not been indexed"
    );
    let mut snapshots = state
        .trees
        .iter()
        .filter(|t| **t != oldest)
        .map(|tree| {
            snapshot_for(&directory, &state, tree, &manifest, &[])
                .with_context(|| format!("invalid cached tree {tree}"))
        })
        .collect::<Result<Vec<_>>>()?;
    let baseline = snapshots
        .iter()
        .find(|s| s.tree == target)
        .context("missing successor snapshot")?;
    let baseline_docs = baseline.documents.clone();
    let mut captured_sizes = HashMap::new();
    for snapshot in &snapshots {
        for meta in &snapshot.segments {
            for file in [&meta.lookup, &meta.postings] {
                if !captured_sizes.contains_key(file) {
                    captured_sizes.insert(file.clone(), fs::metadata(directory.join(file))?.len());
                }
            }
        }
    }
    let selected: HashSet<_> = baseline.segments.iter().map(|s| s.id).collect();
    let mut clean = manifest.clone();
    for (id, doc) in clean.documents.iter_mut().enumerate() {
        if let Some(Some(cached)) = baseline_docs.get(id) {
            *doc = cached.document.clone();
        } else {
            doc.active = false;
        }
    }
    clean.segments = baseline.segments.clone();
    let input_bytes = clean
        .segments
        .iter()
        .map(|s| compaction::bytes(&directory, s))
        .sum::<Result<u64>>()?;
    for meta in &clean.segments {
        for file in [&meta.lookup, &meta.postings] {
            fs::hard_link(directory.join(file), stage.path().join(file))?;
        }
    }
    let prepare_ms = started.elapsed().as_secs_f64() * 1000.0;
    drop(writer);
    let merging = Instant::now();
    if clean.segments.len() > 1 {
        compaction::merge_to_base(stage.path(), &mut clean, || next_segment_id(stage.path()))?;
    } else if let Some(meta) = clean.segments.first() {
        let id = meta.id;
        compaction::merge(
            stage.path(),
            &mut clean,
            &[id],
            next_segment_id(stage.path()),
        )?;
    }
    let merge_ms = merging.elapsed().as_secs_f64() * 1000.0;
    let remapping = Instant::now();
    let output = clean
        .segments
        .first()
        .cloned()
        .context("empty successor index")?;
    let output_bytes = compaction::bytes(stage.path(), &output)?;
    let epoch = output.id;
    for snapshot in &mut snapshots {
        for (id, cached) in snapshot.documents.iter_mut().enumerate() {
            if let Some(cached) = cached
                && let Some(Some(base)) = baseline_docs.get(id)
                && cached.oid == base.oid
                && cached.document.searchable
                && selected.contains(&cached.document.segment_id)
            {
                cached.document.segment_id = clean.documents[id].segment_id;
            }
        }
        let live: HashSet<_> = snapshot
            .documents
            .iter()
            .flatten()
            .filter(|c| c.document.searchable)
            .map(|c| c.document.segment_id)
            .collect();
        snapshot.segments.retain(|s| live.contains(&s.id));
        if live.contains(&output.id) {
            snapshot.segments.push(output.clone());
        }
        snapshot.epoch = epoch;
        snapshot.registry.epoch = epoch;
        write(
            &snapshot_path(stage.path(), epoch, &snapshot.tree),
            snapshot,
        )?;
    }
    for (id, doc) in manifest.documents.iter_mut().enumerate() {
        if doc.active
            && doc.searchable
            && selected.contains(&doc.segment_id)
            && let Some(Some(base)) = baseline_docs.get(id)
            && state.oids[id] == Some(base.oid)
        {
            doc.segment_id = clean.documents[id].segment_id;
        }
    }
    let live: HashSet<_> = manifest
        .documents
        .iter()
        .filter(|d| d.active && d.searchable)
        .map(|d| d.segment_id)
        .collect();
    manifest.segments.retain(|s| live.contains(&s.id));
    if live.contains(&output.id) {
        manifest.segments.push(output.clone());
    }
    manifest
        .registry
        .as_mut()
        .context("missing registry")?
        .epoch = epoch;
    manifest.generation += 1;
    state.epoch = epoch;
    state.trees.retain(|t| *t != oldest);
    state.timeline.retain(|t| *t != oldest);
    state.roots.clear();
    state.registry_counts.remove(&oldest);
    state.segment_bytes.clear();
    for snapshot in &snapshots {
        let roots: Vec<_> = snapshot
            .segments
            .iter()
            .flat_map(|s| [s.lookup.clone(), s.postings.clone()])
            .collect();
        for file in &roots {
            if !state.segment_bytes.contains_key(file) {
                let bytes = if *file == output.lookup || *file == output.postings {
                    fs::metadata(stage.path().join(file))?.len()
                } else {
                    *captured_sizes
                        .get(file)
                        .context("uncaptured history segment")?
                };
                state.segment_bytes.insert(file.clone(), bytes);
            }
        }
        state.roots.insert(snapshot.tree.clone(), roots);
    }
    state.history_watermark = output_bytes.max(1);
    let encoded = manifest::encode(&manifest)?;
    manifest.publication = manifest::publication(&encoded);
    state.manifest = manifest.publication.context("missing publication")?;
    write(&state_path(stage.path()), &state)?;
    let remap_ms = remapping.elapsed().as_secs_f64() * 1000.0;
    before_publish()?;
    let waiting = Instant::now();
    let _writer = writer_lock(&directory)?;
    let publish_wait_ms = waiting.elapsed().as_secs_f64() * 1000.0;
    let publishing = Instant::now();
    let mut header = [0; 28];
    File::open(directory.join(manifest::FILE_NAME))?.read_exact(&mut header)?;
    let published = manifest::publication(&header) == original_publication;
    if published {
        anyhow::ensure!(
            !directory.join(&output.lookup).exists(),
            "history output ID collision"
        );
        for file in [&output.lookup, &output.postings] {
            fs::rename(stage.path().join(file), directory.join(file))?;
        }
        for snapshot in &snapshots {
            fs::rename(
                snapshot_path(stage.path(), epoch, &snapshot.tree),
                snapshot_path(&directory, epoch, &snapshot.tree),
            )?;
        }
        fs::rename(state_path(stage.path()), state_path(&directory))?;
        write_manifest_bytes(&directory.join(manifest::FILE_NAME), &encoded, true)?;
        collect_garbage_with_state(&directory, &manifest, &state)?;
    }
    Ok(HistorySummary {
        published,
        retired_tree: oldest,
        baseline_tree: target,
        input_bytes,
        output_bytes,
        prepare_ms,
        merge_ms,
        remap_ms,
        publish_wait_ms,
        publish_ms: publishing.elapsed().as_secs_f64() * 1000.0,
        total_ms: started.elapsed().as_secs_f64() * 1000.0,
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn intervening_writer_discards_prepared_compaction() {
        let (_temp, directory, mut index) = super::super::tests::cached_index();
        let root = index.root().to_path_buf();
        fs::write(root.join("x"), "committed successor\n").unwrap();
        let repo = Repository::open(&root).unwrap();
        let mut git_index = repo.index().unwrap();
        git_index.add_path(Path::new("x")).unwrap();
        git_index.write().unwrap();
        let tree = repo.find_tree(git_index.write_tree().unwrap()).unwrap();
        let parent = repo.head().unwrap().peel_to_commit().unwrap();
        let signature = git2::Signature::now("Test", "test@example.invalid").unwrap();
        repo.commit(Some("HEAD"), &signature, &signature, "B", &tree, &[&parent])
            .unwrap();
        super::super::super::refresh(&mut index, Some(&directory), "32".parse().unwrap()).unwrap();
        let mut new_publication = None;
        let summary = compact_with_hook(&root, Some(&directory), || {
            fs::write(root.join("x"), "newer workspace content\n")?;
            let mut current = super::super::super::load(&root, Some(&directory))?;
            super::super::super::refresh(&mut current, Some(&directory), "32".parse().unwrap())?;
            new_publication = current.full_manifest().unwrap().publication;
            Ok(())
        })
        .unwrap();
        assert!(!summary.published);
        let current = super::super::super::load(&root, Some(&directory)).unwrap();
        assert_eq!(
            current.full_manifest().unwrap().publication,
            new_publication
        );
        assert_eq!(
            state_for(&directory, current.full_manifest().unwrap().as_ref())
                .unwrap()
                .unwrap()
                .trees
                .len(),
            2
        );
    }
}
