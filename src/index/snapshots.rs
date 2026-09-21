//! Persistent Git content snapshots. Ordinary unchanged refresh never enters
//! this module. Document IDs are append-only within a cache epoch; compaction
//! preserves them, and an explicit build starts a new epoch.
use super::*;
use crate::manifest::SegmentMeta;
use git2::{ObjectType, Oid, Repository};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

type BlobId = [u8; 20];
const CACHE_VERSION: u32 = 3;
const MAX_SNAPSHOTS: usize = 8;
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
    // Checksummed GC roots, published atomically with the active state.
    roots: HashMap<String, Vec<PathBuf>>,
    registries: HashMap<usize, BlobId>,
    registry_counts: HashMap<String, usize>,
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
    let mut searchable = vec![false; sources.len()];
    let next = AtomicUsize::new(0);
    std::thread::scope(|scope| -> Result<()> {
        let (sender, receiver) = mpsc::sync_channel(workers);
        for _ in 0..workers {
            let sender = sender.clone();
            let next = &next;
            scope.spawn(move || {
                let mut repository = None;
                loop {
                    let position = next.fetch_add(1, Ordering::Relaxed);
                    let Some(source) = sources.get(position) else {
                        break;
                    };
                    let result = (|| -> Result<()> {
                        let emit = |grams| {
                            sender
                                .send(Ok((position, grams)))
                                .map_err(|_| anyhow::anyhow!("snapshot build cancelled"))
                        };
                        if let Some(oid) = source.blob {
                            if repository.is_none() {
                                repository = Some(Repository::open(git_dir)?);
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
                    })();
                    if let Err(error) = result {
                        let _ = sender.send(Err(error));
                        break;
                    }
                }
            });
        }
        drop(sender);
        for message in receiver {
            let (position, grams) = message?;
            searchable[position] = true;
            postings.extend(sources[position].id, &grams)?;
        }
        Ok(())
    })?;
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
    if current == index.manifest.source_state && same_identity(&index.manifest, &identity) {
        return Ok(RefreshOutcome::Unchanged);
    }
    let changed = changed_file_count(&index.manifest.source_state, &current);
    let mut manifest = index.manifest.clone();
    update_identity(&mut manifest, identity);
    let (manifest, indexed, reused, compaction) = update(
        directory,
        budget,
        manifest,
        current,
        Some(&index.manifest),
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
    index.manifest = manifest;
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

fn update(
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
        roots: HashMap::new(),
        registries: HashMap::new(),
        registry_counts: HashMap::new(),
    });
    let mut registry = manifest
        .registry
        .clone()
        .filter(|r| r.epoch == state.epoch && state.registries.get(&r.count) == Some(&r.digest))
        .unwrap_or(crate::manifest::RegistryIdentity {
            epoch: state.epoch,
            count: 0,
            digest: [0; 20],
        });
    let old_oids = std::mem::take(&mut state.oids);
    let old_documents = previous.map_or_else(
        || std::borrow::Cow::Owned(manifest.documents.clone()),
        |m| std::borrow::Cow::Borrowed(m.documents.as_slice()),
    );
    let old_states: HashMap<_, _> = manifest
        .source_state
        .iter()
        .map(|s| (s.path.as_path(), s))
        .collect();
    let mut ids: HashMap<_, _> = manifest
        .documents
        .iter()
        .enumerate()
        .map(|(id, doc)| (doc.path.clone(), id as u32))
        .collect();
    for source in &current {
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
    state.oids.resize(manifest.documents.len(), None);
    let current_by_id: HashMap<_, _> = current.iter().map(|s| (ids[&s.path] as usize, s)).collect();
    let mut contents = InitialContents::default();
    let mut buffered = HashSet::new();
    if initial {
        let limit = (budget.record_bytes(budget.workers(current.len())) / 4).min(32 * 1024 * 1024);
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
    for (id, oid, unchanged, bytes) in hashes {
        if let Some(slot) = contents.files.get_mut(id) {
            *slot = bytes;
        }
        working_oids[id] = Some(oid);
        if unchanged && old_documents.get(id).is_some_and(|d| d.active) {
            previous_oids[id] = Some(oid);
        }
    }
    let tree_id = manifest.git_tree.clone().context("missing Git tree")?;
    let cached_base = snapshot_for(directory, &state, &tree_id, &manifest, validated);
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
        .and_then(|tree| snapshot_for(directory, &state, tree, &manifest, validated));
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
    let scoped = |tree: &git2::Tree<'_>| -> Result<Option<git2::Tree<'_>>> {
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
    };
    let target_tree = scoped(&tree)?;
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
        let prior_tree = scoped(&prior_tree)?;
        if prior_tree.is_some() || target_tree.is_some() {
            let diff = repo.diff_tree_to_tree(prior_tree.as_ref(), target_tree.as_ref(), None)?;
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
                        git2::FileMode::Blob | git2::FileMode::BlobExecutable
                            if !file.id().is_zero() =>
                        {
                            Some(file.id())
                        }
                        _ => None,
                    };
                    changes.insert(id as usize, oid);
                }
            }
        }
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
    base_changed |= base.registry != registry;
    base.registry = registry;
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
    let durable = initial || !compaction::small_base(directory, &manifest)?;
    if let Some((files, meta)) = extract(
        &manifest.root,
        repo.path(),
        directory,
        budget,
        &pending_base,
        durable,
        &contents,
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

    for document in &mut manifest.documents {
        document.active = false;
    }
    let mut pending_work = Vec::new();
    let mut reused = 0;
    for source in &current {
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
            if !old_states
                .get(source.path.as_path())
                .is_some_and(|old| *old == source)
            {
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
        repo.path(),
        directory,
        budget,
        &pending_work,
        durable,
        &contents,
    )? {
        for file in files {
            let id = file.id as usize;
            manifest.documents[id] = document(file, meta.id);
        }
        metas.insert(meta.id, meta);
    }
    let indexed = pending_base.len() + pending_work.len();
    let active_ids: HashSet<_> = manifest
        .documents
        .iter()
        .filter(|d| d.active && d.searchable)
        .map(|d| d.segment_id)
        .collect();
    manifest.segments = metas
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
        compaction::merge(directory, &mut manifest, inputs, next_segment_id(directory))?
    } else {
        compaction::Summary::default()
    };
    // Publishing already indexed versions (promotion, rollback or revisit)
    // only changes references. Automatic maintenance belongs to the layer
    // receiving newly extracted content, not to version identity changes.
    let base_inputs = if pending_base.is_empty() {
        Vec::new()
    } else {
        compaction::snapshot_plan(directory, &base.segments)?
    };
    if !base_inputs.is_empty() {
        let mut clean = manifest.clone();
        clean.documents = manifest
            .documents
            .iter()
            .enumerate()
            .map(|(id, doc)| {
                base.documents[id]
                    .as_ref()
                    .map(|c| c.document.clone())
                    .unwrap_or_else(|| {
                        let mut doc = doc.clone();
                        doc.active = false;
                        doc
                    })
            })
            .collect();
        clean.segments = base.segments.clone();
        if base_inputs.len() > compaction::MAX_MERGE_INPUTS {
            // The plan puts the largest baseline first. Keep it out of all
            // intermediate batches so it is rewritten only in the final one.
            let largest = clean
                .segments
                .iter()
                .position(|meta| meta.id == base_inputs[0])
                .context("snapshot baseline is missing from merge inputs")?;
            let baseline = clean.segments.remove(largest);
            clean.segments.push(baseline);
            compaction::merge_to_base(directory, &mut clean, || next_segment_id(directory))?;
        } else {
            compaction::merge(
                directory,
                &mut clean,
                &base_inputs,
                next_segment_id(directory),
            )?;
        }
        for (id, cached) in base.documents.iter_mut().enumerate() {
            if let Some(cached) = cached {
                cached.document = clean.documents[id].clone();
            }
        }
        base.segments = clean.segments;
        for (id, cached) in base.documents.iter().enumerate() {
            if let Some(cached) = cached
                && manifest.documents[id].active
                && working_oids[id] == Some(cached.oid)
            {
                manifest.documents[id].segment_id = cached.document.segment_id;
            }
        }
        manifest.segments.extend(base.segments.iter().cloned());
        let active: HashSet<_> = manifest
            .documents
            .iter()
            .filter(|d| d.active && d.searchable)
            .map(|d| d.segment_id)
            .collect();
        manifest.segments.retain(|s| active.contains(&s.id));
        manifest.segments.sort_unstable_by_key(|s| s.id);
        manifest.segments.dedup_by_key(|s| s.id);
        base_changed = true;
    }
    manifest.source_state = current;
    if !initial {
        manifest.generation += 1;
    }
    state.oids = working_oids;
    let encoded_manifest = manifest::encode(&manifest)?;
    manifest.publication = manifest::publication(&encoded_manifest);
    state.manifest = manifest
        .publication
        .context("missing publication identity")?;
    state.trees.retain(|t| *t != tree_id);
    state.trees.push(tree_id.clone());
    if state.trees.len() > MAX_SNAPSHOTS {
        state.trees.remove(0);
    }
    if base_changed || !snapshot_path(directory, state.epoch, &tree_id).exists() {
        write(&snapshot_path(directory, state.epoch, &tree_id), &base)?;
    }
    state
        .registry_counts
        .retain(|tree, _| state.trees.contains(tree));
    state
        .registry_counts
        .insert(tree_id.clone(), base.registry.count);
    let retained_counts: HashSet<_> = state.registry_counts.values().copied().collect();
    state
        .registries
        .retain(|count, _| retained_counts.contains(count));
    state.roots.retain(|tree, _| state.trees.contains(tree));
    state.roots.insert(
        tree_id.clone(),
        base.segments
            .iter()
            .flat_map(|s| [s.lookup.clone(), s.postings.clone()])
            .collect(),
    );
    write(&state_path(directory), &state)?;
    write_manifest_bytes(
        &directory.join(manifest::FILE_NAME),
        &encoded_manifest,
        durable,
    )?;
    collect_garbage_with_state(directory, &manifest, &state)?;
    Ok((manifest, indexed, reused, summary))
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

    fn cached_index() -> (tempfile::TempDir, PathBuf, DiskIndex) {
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
        let mut query =
            super::super::load_for_search(&index.manifest.root, Some(&directory)).unwrap();
        assert!(query.manifest.documents.is_empty());
        assert!(query.manifest.source_state.is_empty());
        assert!(matches!(
            super::super::refresh(&mut query, Some(&directory), "32".parse().unwrap()).unwrap(),
            RefreshOutcome::Unchanged
        ));
        assert!(query.mapped.is_some());
        assert!(query.manifest.publication.is_none());
        assert_eq!(query.document_path(0).unwrap(), Path::new("x"));
        fs::write(query.manifest.root.join("z"), "new query content\n").unwrap();
        super::super::refresh(&mut query, Some(&directory), "32".parse().unwrap()).unwrap();
        assert!(query.mapped.is_none());
        assert!(query.manifest.publication.is_some());
        assert_eq!(query.all_document_ids(), vec![0, 1]);
    }

    #[test]
    fn refresh_installs_the_published_view_for_immediate_queries() {
        let (_temp, directory, mut index) = cached_index();
        fs::write(index.manifest.root.join("z"), "distinctive quasar token\n").unwrap();
        super::super::refresh(&mut index, Some(&directory), "32".parse().unwrap()).unwrap();
        assert_eq!(
            index.manifest,
            read_manifest(&directory.join(manifest::FILE_NAME)).unwrap()
        );
        assert_eq!(index.all_document_ids(), vec![0, 1]);
        let gram = ngram::covering_hashes(b"distinctive quasar token", 1)[0];
        assert!(index.postings(gram).unwrap().contains(&1));
    }

    #[test]
    fn publication_mismatch_and_interrupted_publish_disable_cache() {
        let (_temp, directory, index) = cached_index();
        let mut state = state_for(&directory, &index.manifest).unwrap().unwrap();
        let mut next = index.manifest.clone();
        next.generation += 1;
        next.publication = manifest::publication(&manifest::encode(&next).unwrap());
        assert!(state_for(&directory, &next).unwrap().is_none());
        // Model state published before the new active manifest.
        state.manifest = next.publication.unwrap();
        write(&state_path(&directory), &state).unwrap();
        assert!(state_for(&directory, &index.manifest).unwrap().is_none());
        assert!(state_for(&directory, &next).unwrap().is_some());
    }

    #[test]
    fn registry_mismatch_and_same_id_different_segment_metadata_are_rejected() {
        let (_temp, directory, index) = cached_index();
        let state = state_for(&directory, &index.manifest).unwrap().unwrap();
        let tree = state.trees.last().unwrap();
        let path = snapshot_path(&directory, state.epoch, tree);
        let mut snapshot =
            snapshot_for(&directory, &state, tree, &index.manifest, &index.segments).unwrap();
        snapshot.registry.digest[0] ^= 1;
        write(&path, &snapshot).unwrap();
        assert!(snapshot_for(&directory, &state, tree, &index.manifest, &index.segments).is_none());
        snapshot.registry.digest[0] ^= 1;
        // Matching ID alone must never bypass opening a distinct segment.
        snapshot.segments[0].lookup = "segments/missing.lookup".into();
        write(&path, &snapshot).unwrap();
        assert!(snapshot_for(&directory, &state, tree, &index.manifest, &index.segments).is_none());
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
        let old_path = directory.join(&original.manifest.segments[0].lookup);
        for version in 0..12 {
            fs::write(root.join("x"), format!("changed version {version}\n")).unwrap();
            commit();
            let mut current = super::super::load(&root, Some(&directory)).unwrap();
            super::super::refresh(&mut current, Some(&directory), budget).unwrap();
        }
        assert!(
            !old_path.exists(),
            "evicted snapshots must release obsolete files"
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
