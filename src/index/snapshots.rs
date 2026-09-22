//! Persistent Git content snapshots. Ordinary unchanged refresh never enters
//! this module. Document IDs are append-only within a cache epoch; compaction
//! preserves them, and an explicit build starts a new epoch.
use super::*;

#[path = "snapshots/update.rs"]
mod update;
use crate::manifest::SegmentMeta;
use git2::{ObjectType, Oid, Repository};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use update::update;
#[path = "snapshots/oldest.rs"]
mod oldest;
pub use oldest::HistorySummary;
pub(super) use oldest::compact_oldest;

type BlobId = [u8; 20];
const CACHE_VERSION: u32 = 5;
const MAX_OVERLAY_SEGMENTS: usize = 8;

pub struct Stats {
    pub retained_trees: usize,
    pub retained_bytes: u64,
    pub base_segments: usize,
    pub overlay_segments: usize,
    pub base_bytes: u64,
    pub overlay_bytes: u64,
}

pub(super) fn stats(directory: &Path, manifest: &Manifest) -> Result<Option<Stats>> {
    let Some(state) = state_for(directory, manifest)? else {
        return Ok(None);
    };
    let Some(tree) = state.trees.last() else {
        return Ok(None);
    };
    let Some(base) = snapshot_for(directory, &state, tree, manifest, &[]) else {
        return Ok(None);
    };
    let base_ids: HashSet<_> = base.segments.iter().map(|s| s.id).collect();
    let overlay: Vec<_> = manifest
        .segments
        .iter()
        .filter(|s| !base_ids.contains(&s.id))
        .collect();
    let mut retained_bytes = 0;
    for name in ["snapshots", "segments"] {
        for entry in fs::read_dir(directory.join(name))? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                retained_bytes += entry.metadata()?.len();
            }
        }
    }
    Ok(Some(Stats {
        retained_trees: state.trees.len(),
        retained_bytes,
        base_segments: base.segments.len(),
        overlay_segments: overlay.len(),
        base_bytes: base
            .segments
            .iter()
            .map(|s| compaction::bytes(directory, s))
            .sum::<Result<u64>>()?,
        overlay_bytes: overlay
            .iter()
            .map(|s| compaction::bytes(directory, s))
            .sum::<Result<u64>>()?,
    }))
}

#[derive(Serialize, Deserialize)]
struct State {
    version: u32,
    epoch: u64,
    manifest: BlobId,
    oids: Vec<Option<BlobId>>,
    // Oldest first; the last entry is the active commit baseline.
    trees: Vec<String>,
    // First indexed order, independent of checkout/recency.
    timeline: Vec<String>,
    // Checksummed GC roots, published atomically with the active state.
    roots: HashMap<String, Vec<PathBuf>>,
    registries: HashMap<usize, BlobId>,
    registry_counts: HashMap<String, usize>,
    history_watermark: u64,
    segment_bytes: HashMap<PathBuf, u64>,
}

#[derive(Clone, Serialize, Deserialize)]
struct CachedDocument {
    oid: BlobId,
    document: Document,
}

#[derive(Serialize, Deserialize)]
struct Snapshot {
    version: u32,
    root: PathBuf,
    epoch: u64,
    tree: String,
    registry: crate::manifest::RegistryIdentity,
    documents: Vec<Option<CachedDocument>>,
    segments: Vec<SegmentMeta>,
}

fn blob_id(oid: Oid) -> BlobId {
    oid.as_bytes().try_into().expect("SHA-1 Git OID")
}

fn config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_fixed_int_encoding()
        .with_limit::<268_435_456>()
}

fn read<T: DeserializeOwned>(path: &Path) -> Option<T> {
    // A missing, incompatible or torn cache is a miss, never authority over the
    // published manifest. Bound allocation even for a damaged local cache.
    if fs::metadata(path).ok()?.len() > 268_435_456 {
        return None;
    }
    let bytes = fs::read(path).ok()?;
    let payload_len = bytes.len().checked_sub(20)?;
    let (payload, checksum) = bytes.split_at(payload_len);
    if Oid::hash_object(ObjectType::Blob, payload).ok()?.as_bytes() != checksum {
        return None;
    }
    let (value, used) = bincode::serde::decode_from_slice(payload, config()).ok()?;
    (used == payload.len()).then_some(value)
}

fn write<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let parent = path.parent().context("cache has no parent")?;
    fs::create_dir_all(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    let bytes = bincode::serde::encode_to_vec(value, config())?;
    temporary.write_all(&bytes)?;
    temporary.write_all(Oid::hash_object(ObjectType::Blob, &bytes)?.as_bytes())?;
    temporary
        .persist(path)
        .with_context(|| format!("cannot publish {}", path.display()))?;
    // Sidecars are optional, checksummed caches. Active manifest and segment
    // durability is unchanged; no per-sidecar fsync is needed for correctness.
    Ok(())
}

fn state_path(directory: &Path) -> PathBuf {
    directory.join("snapshots/state.bin")
}

fn snapshot_path(directory: &Path, epoch: u64, tree: &str) -> PathBuf {
    directory
        .join("snapshots")
        .join(format!("{epoch:020}-{tree}.bin"))
}

fn state_for(directory: &Path, manifest: &Manifest) -> Result<Option<State>> {
    Ok(read::<State>(&state_path(directory)).filter(|state| {
        state.version == CACHE_VERSION
            && state.oids.len() == manifest.documents.len()
            && manifest.publication == Some(state.manifest)
            && manifest.registry.as_ref().is_some_and(|registry| {
                registry.epoch == state.epoch
                    && registry.count == manifest.documents.len()
                    && state.registries.get(&registry.count) == Some(&registry.digest)
            })
    }))
}

fn snapshot_for(
    directory: &Path,
    state: &State,
    tree: &str,
    manifest: &Manifest,
    validated: &[segment::Segment],
) -> Option<Snapshot> {
    let snapshot: Snapshot = read(&snapshot_path(directory, state.epoch, tree))?;
    if snapshot.version != CACHE_VERSION
        || snapshot.epoch != state.epoch
        || snapshot.root != manifest.root
        || snapshot.tree != tree
        || snapshot.documents.len() > manifest.documents.len()
        || snapshot.registry.epoch != state.epoch
        || snapshot.registry.count != snapshot.documents.len()
        || state.registry_counts.get(tree) != Some(&snapshot.registry.count)
        || state.registries.get(&snapshot.registry.count) != Some(&snapshot.registry.digest)
    {
        return None;
    }
    let segment_ids: HashSet<_> = snapshot.segments.iter().map(|s| s.id).collect();
    for cached in snapshot.documents.iter().flatten() {
        if cached.document.searchable && !segment_ids.contains(&cached.document.segment_id) {
            return None;
        }
    }
    // Validate cached mappings before selecting them; bad/missing caches fall
    // back to normal extraction. The writer lock excludes segment collection.
    let loaded: HashMap<_, _> = validated.iter().map(|s| (s.meta.id, &s.meta)).collect();
    for meta in &snapshot.segments {
        if loaded.get(&meta.id).is_some_and(|known| *known == meta) {
            continue;
        }
        if segment::load(directory, meta).is_err() {
            return None;
        }
    }
    Some(snapshot)
}

pub(super) fn reader_lock(directory: &Path) -> Result<File> {
    let path = directory.join("read.lock");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?,
        Err(error) => return Err(error.into()),
    };
    file.lock_shared()?;
    Ok(file)
}

/// Metadata equality remains the normal refresh contract. Validate changed
/// files around hashing/extraction so ordinary concurrent edits abort publish.
fn check_source(root: &Path, state: &FileState) -> Result<()> {
    let metadata = fs::metadata(root.join(&state.path))?;
    let modified = metadata
        .modified()?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    if metadata.len() != state.len || modified != state.modified_nanos {
        bail!(
            "source changed while indexing {}; retry the search",
            state.path.display()
        );
    }
    Ok(())
}

// Initial builds retain a bounded subset of small-file contents between
// identity calculation and extraction. Reserve their space from postings.
#[derive(Default)]
struct InitialContents {
    files: Vec<Option<Box<[u8]>>>,
    reserved: usize,
}

#[derive(Clone)]
struct Source {
    id: u32,
    state: FileState,
    // None reads actual worktree bytes, including any checkout transformations.
    blob: Option<BlobId>,
}

fn extract(
    root: &Path,
    git_dir: &Path,
    directory: &Path,
    budget: MemoryBudget,
    sources: &[Source],
    durable: bool,
    contents: &InitialContents,
) -> Result<Option<(Vec<IndexedFile>, SegmentMeta)>> {
    if sources.is_empty() {
        return Ok(None);
    }
    let workers = budget.workers(sources.len());
    let mut postings =
        PostingsBuilder::new(directory, budget.record_bytes(workers) - contents.reserved);
    let searchable = extraction::collect(
        sources,
        workers,
        &mut postings,
        |source| source.id,
        || None::<Repository>,
        |repository, source, emit| {
            if let Some(oid) = source.blob {
                if repository.is_none() {
                    *repository = Some(Repository::open(git_dir)?);
                }
                let blob = repository
                    .as_ref()
                    .unwrap()
                    .find_blob(Oid::from_bytes(&oid)?)?;
                hash_reader_chunks(blob.content(), emit)?;
            } else if let Some(bytes) = contents
                .files
                .get(source.id as usize)
                .and_then(Option::as_deref)
            {
                // OID and grams come from the same checked byte snapshot.
                hash_reader_chunks(bytes, emit)?;
            } else {
                check_source(root, &source.state)?;
                hash_file_chunks(&root.join(&source.state.path), emit)?;
                check_source(root, &source.state)?;
            }
            Ok(())
        },
    )?;
    let files = sources
        .iter()
        .zip(searchable)
        .map(|(source, searchable)| IndexedFile {
            id: source.id,
            state: source.state.clone(),
            searchable,
        })
        .collect();
    let segment = postings.write_with_durability(next_segment_id(directory), durable)?;
    Ok(Some((files, segment)))
}

fn document(file: IndexedFile, segment_id: u64) -> Document {
    Document {
        path: file.state.path,
        len: file.state.len,
        modified_nanos: file.state.modified_nanos,
        active: true,
        searchable: file.searchable,
        segment_id,
    }
}

pub(super) fn build(
    directory: &Path,
    budget: MemoryBudget,
    mut manifest: Manifest,
) -> Result<BuildSummary> {
    fs::create_dir_all(directory.join("segments"))?;
    let current = std::mem::take(&mut manifest.source_state);
    let (manifest, _, _, _) = update(directory, budget, manifest, current, None, &[])?;
    Ok(BuildSummary {
        files: manifest
            .documents
            .iter()
            .filter(|d| d.active && d.searchable)
            .count(),
        bytes: searchable_bytes(&manifest),
        ngrams: manifest.segments.iter().map(|s| s.ngrams as usize).sum(),
        index_dir: directory.to_owned(),
    })
}

pub(super) fn refresh(
    index: &mut DiskIndex,
    directory: &Path,
    current: Vec<FileState>,
    identity: Option<git_state::GitIdentity>,
    budget: MemoryBudget,
) -> Result<RefreshOutcome> {
    if current == index.full_manifest()?.source_state && index.same_identity(&identity) {
        return Ok(RefreshOutcome::Unchanged);
    }
    let changed = changed_file_count(&index.full_manifest()?.source_state, &current);
    let mut manifest = index.full_manifest()?.into_owned();
    update_identity(&mut manifest, identity);
    let (manifest, indexed, reused, compaction) = update(
        directory,
        budget,
        manifest,
        current,
        Some(index.full_manifest()?.as_ref()),
        &index.segments,
    )?;
    // Keep the verified publication and unchanged mmaps in memory. The writer
    // lock still excludes collection while we open newly referenced segments.
    let loaded: HashMap<_, _> = index
        .segments
        .iter()
        .map(|s| (s.meta.id, &s.meta))
        .collect();
    let mut fresh = HashMap::new();
    for meta in &manifest.segments {
        if !loaded.get(&meta.id).is_some_and(|known| *known == meta) {
            fresh.insert(meta.id, segment::load(directory, meta)?);
        }
    }
    let mut retained: HashMap<_, _> = std::mem::take(&mut index.segments)
        .into_iter()
        .map(|s| (s.meta.id, s))
        .collect();
    index.segments = manifest
        .segments
        .iter()
        .map(|meta| {
            fresh
                .remove(&meta.id)
                .or_else(|| retained.remove(&meta.id))
                .expect("every published segment was loaded")
        })
        .collect();
    index.storage = ManifestStorage::Owned(manifest);
    if changed == 0 && indexed == 0 {
        return Ok(RefreshOutcome::CommitAdvanced { loaded: true });
    }
    Ok(RefreshOutcome::Snapshot {
        changed,
        indexed,
        reused,
        compaction,
    })
}

pub(super) fn compacted(directory: &Path, manifest: &mut Manifest) -> Result<Vec<u8>> {
    let old = read_manifest(&directory.join(manifest::FILE_NAME))?;
    let encoded = manifest::encode(manifest)?;
    manifest.publication = manifest::publication(&encoded);
    if let Some(mut state) = state_for(directory, &old)?
        && let Some(publication) = manifest.publication
    {
        state.manifest = publication;
        write(&state_path(directory), &state)?;
    }
    Ok(encoded)
}

/// Writers hold write.lock; readers hold read.lock only while opening mmaps.
/// Unix mappings survive unlink. Other platforms defer segment reclamation.
pub(super) fn collect_garbage(directory: &Path, manifest: &Manifest) -> Result<()> {
    let Some(state) = state_for(directory, manifest)? else {
        return Ok(());
    };
    collect_garbage_with_state(directory, manifest, &state)
}

fn collect_garbage_with_state(directory: &Path, manifest: &Manifest, state: &State) -> Result<()> {
    {
        let mut live: HashSet<PathBuf> = manifest
            .segments
            .iter()
            .flat_map(|s| [s.lookup.clone(), s.postings.clone()])
            .collect();
        let mut roots = HashSet::from([state_path(directory)]);
        for tree in &state.trees {
            let path = snapshot_path(directory, state.epoch, tree);
            let Some(segments) = state.roots.get(tree) else {
                // Never reclaim against an incomplete root set.
                return Ok(());
            };
            roots.insert(path);
            live.extend(segments.iter().cloned());
        }
        let guard = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(directory.join("read.lock"))?;
        guard.lock()?;
        #[cfg(unix)]
        if let Ok(entries) = fs::read_dir(directory.join("segments")) {
            for entry in entries {
                let entry = entry?;
                let path = PathBuf::from("segments").join(entry.file_name());
                if matches!(
                    path.extension().and_then(|e| e.to_str()),
                    Some("lookup" | "postings")
                ) && !live.contains(&path)
                {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        for entry in fs::read_dir(directory.join("snapshots"))? {
            let entry = entry?;
            if entry.path().extension().is_some_and(|e| e == "bin")
                && !roots.contains(&entry.path())
            {
                fs::remove_file(entry.path())?;
            }
        }
    }
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    pub(super) fn cached_index() -> (tempfile::TempDir, PathBuf, DiskIndex) {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("repo");
        let directory = temp.path().join("index");
        let repo = Repository::init(&root).unwrap();
        fs::write(root.join("x"), "original needle\n").unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("x")).unwrap();
        index.write().unwrap();
        let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
        let signature = git2::Signature::now("Test", "test@example.invalid").unwrap();
        repo.commit(Some("HEAD"), &signature, &signature, "initial", &tree, &[])
            .unwrap();
        super::super::build(&root, Some(&directory), "32".parse().unwrap()).unwrap();
        let loaded = super::super::load(&root, Some(&directory)).unwrap();
        (temp, directory, loaded)
    }

    #[test]
    fn unchanged_search_keeps_manifest_records_mapped() {
        let (_temp, directory, index) = cached_index();
        let mut query = super::super::load_for_search(index.root(), Some(&directory)).unwrap();
        assert!(matches!(query.storage, ManifestStorage::Mapped(_)));
        assert!(matches!(
            super::super::refresh(&mut query, Some(&directory), "32".parse().unwrap()).unwrap(),
            RefreshOutcome::Unchanged
        ));
        assert!(matches!(query.storage, ManifestStorage::Mapped(_)));
        assert_eq!(query.document_path(0).unwrap(), Path::new("x"));
        // Consumers requesting a full manifest must see actual records, even
        // when ordinary searches keep those records mapped.
        assert_eq!(
            query.full_manifest().unwrap(),
            index.full_manifest().unwrap()
        );
        assert!(matches!(query.storage, ManifestStorage::Mapped(_)));
        fs::write(query.root().join("z"), "new query content\n").unwrap();
        super::super::refresh(&mut query, Some(&directory), "32".parse().unwrap()).unwrap();
        assert!(matches!(query.storage, ManifestStorage::Owned(_)));
        assert!(query.full_manifest().unwrap().publication.is_some());
        assert_eq!(query.all_document_ids(), vec![0, 1]);
    }

    #[test]
    fn failed_materialization_preserves_mapped_records() {
        let (_temp, directory, index) = cached_index();
        let path = directory.join(manifest::FILE_NAME);
        let mut bytes = fs::read(&path).unwrap();
        // Search intentionally defers checksum verification. A later explicit
        // load must fail without replacing its mapped records with an empty view.
        bytes[8] ^= 1;
        fs::write(path, bytes).unwrap();
        let mut query = super::super::load_for_search(index.root(), Some(&directory)).unwrap();
        assert!(query.materialize().is_err());
        assert!(query.full_manifest().is_err());
        assert!(matches!(query.storage, ManifestStorage::Mapped(_)));
        assert_eq!(query.document_path(0).unwrap(), Path::new("x"));
    }

    #[test]
    fn refresh_installs_the_published_view_for_immediate_queries() {
        let (_temp, directory, mut index) = cached_index();
        fs::write(index.root().join("z"), "distinctive quasar token\n").unwrap();
        super::super::refresh(&mut index, Some(&directory), "32".parse().unwrap()).unwrap();
        assert_eq!(
            *index.full_manifest().unwrap(),
            read_manifest(&directory.join(manifest::FILE_NAME)).unwrap()
        );
        assert_eq!(index.all_document_ids(), vec![0, 1]);
        let gram = ngram::covering_hashes(b"distinctive quasar token", 1)[0];
        assert!(index.postings(gram).unwrap().contains(&1));
    }

    #[test]
    fn publication_mismatch_and_interrupted_publish_disable_cache() {
        let (_temp, directory, index) = cached_index();
        let mut state = state_for(&directory, index.full_manifest().unwrap().as_ref())
            .unwrap()
            .unwrap();
        let mut next = index.full_manifest().unwrap().into_owned();
        next.generation += 1;
        next.publication = manifest::publication(&manifest::encode(&next).unwrap());
        assert!(state_for(&directory, &next).unwrap().is_none());
        // Model state published before the new active manifest.
        state.manifest = next.publication.unwrap();
        write(&state_path(&directory), &state).unwrap();
        assert!(
            state_for(&directory, index.full_manifest().unwrap().as_ref())
                .unwrap()
                .is_none()
        );
        assert!(state_for(&directory, &next).unwrap().is_some());
    }

    #[test]
    fn registry_mismatch_and_same_id_different_segment_metadata_are_rejected() {
        let (_temp, directory, index) = cached_index();
        let state = state_for(&directory, index.full_manifest().unwrap().as_ref())
            .unwrap()
            .unwrap();
        let tree = state.trees.last().unwrap();
        let path = snapshot_path(&directory, state.epoch, tree);
        let mut snapshot = snapshot_for(
            &directory,
            &state,
            tree,
            index.full_manifest().unwrap().as_ref(),
            &index.segments,
        )
        .unwrap();
        snapshot.registry.digest[0] ^= 1;
        write(&path, &snapshot).unwrap();
        assert!(
            snapshot_for(
                &directory,
                &state,
                tree,
                index.full_manifest().unwrap().as_ref(),
                &index.segments
            )
            .is_none()
        );
        snapshot.registry.digest[0] ^= 1;
        // Matching ID alone must never bypass opening a distinct segment.
        snapshot.segments[0].lookup = "segments/missing.lookup".into();
        write(&path, &snapshot).unwrap();
        assert!(
            snapshot_for(
                &directory,
                &state,
                tree,
                index.full_manifest().unwrap().as_ref(),
                &index.segments
            )
            .is_none()
        );
    }

    #[test]
    fn mapped_readers_survive_eviction_and_openers_block_collection() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("repo");
        let directory = temp.path().join("index");
        let repo = Repository::init(&root).unwrap();
        let commit = || {
            let mut index = repo.index().unwrap();
            index.add_path(Path::new("x")).unwrap();
            index.write().unwrap();
            let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
            let parent = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
            let parents: Vec<_> = parent.iter().collect();
            let signature = git2::Signature::now("Test", "test@example.invalid").unwrap();
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                "version",
                &tree,
                &parents,
            )
            .unwrap();
        };
        fs::write(root.join("x"), "original needle\n").unwrap();
        commit();
        let budget = "32".parse().unwrap();
        super::super::build(&root, Some(&directory), budget).unwrap();
        let original = super::super::load(&root, Some(&directory)).unwrap();
        let old_path = directory.join(&original.full_manifest().unwrap().segments[0].lookup);
        for version in 0..12 {
            fs::write(root.join("x"), format!("changed version {version}\n")).unwrap();
            commit();
            let mut current = super::super::load(&root, Some(&directory)).unwrap();
            super::super::refresh(&mut current, Some(&directory), budget).unwrap();
        }
        assert!(
            old_path.exists(),
            "historical snapshots must retain referenced files"
        );
        let gram = ngram::covering_hashes(b"original needle", 1)[0];
        assert_eq!(original.postings(gram).unwrap(), vec![0]);
        let reader = reader_lock(&directory).unwrap();
        let writer = File::open(directory.join("read.lock")).unwrap();
        assert!(writer.try_lock().is_err());
        drop(reader);
        writer.try_lock().unwrap();
    }
}
