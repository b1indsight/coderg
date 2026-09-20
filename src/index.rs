use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::{self, File},
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use crate::manifest::{self, FileState};
pub use crate::manifest::{Document, Manifest};
use anyhow::{Context, Result, bail};
use ignore::{DirEntry, WalkBuilder, WalkState};
use rayon::prelude::*;

use crate::{
    build::{CHUNK_BYTES, MemoryBudget, PostingsBuilder},
    compaction, git_state, ngram, segment,
};

// v5 changes sparse-gram selection to the fixed letter-frequency prior.
const VERSION: u32 = 5;
// Small snapshots do not amortize the parallel walker's worker startup and
// shutdown costs. This hint only selects how to walk; every file is checked.
const MAX_SERIAL_WALK_FILES: usize = 512;

// Each walker owns its batch and publishes it once, when traversal finishes.
// File metadata checks must not contend on a shared lock for every file.
struct FileCollector<'a> {
    root: &'a Path,
    batches: &'a Mutex<Vec<Result<Vec<FileState>>>>,
    files: Result<Vec<FileState>>,
}

impl FileCollector<'_> {
    fn visit(&mut self, entry: std::result::Result<DirEntry, ignore::Error>) -> WalkState {
        let state = entry
            .map_err(anyhow::Error::from)
            .and_then(|entry| file_state(self.root, &entry));
        match state {
            Ok(Some(state)) => {
                if let Ok(files) = &mut self.files {
                    files.push(state);
                }
            }
            Ok(None) => {}
            Err(error) => {
                self.files = Err(error);
                return WalkState::Quit;
            }
        }
        WalkState::Continue
    }
}

impl Drop for FileCollector<'_> {
    fn drop(&mut self) {
        self.batches
            .lock()
            .unwrap()
            .push(std::mem::replace(&mut self.files, Ok(Vec::new())));
    }
}

pub struct BuildSummary {
    pub files: usize,
    pub bytes: u64,
    pub ngrams: usize,
    pub index_dir: PathBuf,
}

pub struct Stats {
    pub root: PathBuf,
    pub files: usize,
    pub source_bytes: u64,
    pub ngrams: u64,
    pub index_bytes: u64,
    pub segments: usize,
    pub git_tree: Option<String>,
    pub middle_segments: usize,
    pub middle_bytes: u64,
    pub base_bytes: u64,
    pub generational: bool,
    pub full_compaction_threshold_bytes: u64,
    pub maintenance_due: bool,
}

pub enum RefreshOutcome {
    Unchanged,
    CommitAdvanced,
    Rebuilt,
    Incremental {
        changed: usize,
        compaction: compaction::Summary,
    },
}

pub struct DiskIndex {
    pub manifest: Manifest,
    segments: Vec<segment::Segment>,
    mapped: Option<manifest::view::View>,
}

struct IndexedFile {
    id: u32,
    state: FileState,
    searchable: bool,
}

pub fn resolve_root(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("cannot access search root {}", path.display()))
}

pub fn resolve_index_dir(root: &Path, requested: Option<&Path>) -> PathBuf {
    match requested {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => root.join(path),
        None => root.join(".coderg-index"),
    }
}

pub fn build(
    path: &Path,
    requested_index_dir: Option<&Path>,
    budget: MemoryBudget,
) -> Result<BuildSummary> {
    let root = resolve_root(path)?;
    let index_dir = resolve_index_dir(&root, requested_index_dir);
    prepare_index_dir(&root, &index_dir)?;
    let _writer = writer_lock(&index_dir)?;
    let source_state = collect_files(&root, &index_dir, None)?;
    let repository_state = git_state::identity(&root)?;
    let mut manifest = Manifest {
        registry: None,
        publication: None,
        version: VERSION,
        root,
        generation: 1,
        git_repository: false,
        git_head: None,
        git_tree: None,
        source_state,
        documents: Vec::new(),
        segments: Vec::new(),
    };
    update_identity(&mut manifest, repository_state);
    rebuild(&index_dir, budget, manifest, true)
}

// The caller holds the writer lock and supplies the current source snapshot.
fn rebuild(
    index_dir: &Path,
    budget: MemoryBudget,
    mut manifest: Manifest,
    durable: bool,
) -> Result<BuildSummary> {
    let segment_id = next_segment_id(index_dir);
    let (indexed, postings) = extract_files(
        &manifest.root,
        index_dir,
        budget,
        manifest
            .source_state
            .iter()
            .cloned()
            .enumerate()
            .map(|(id, state)| (id as u32, state)),
    )?;
    let segment = postings.write_with_durability(segment_id, durable)?;
    let sync_on_growth =
        !durable && compaction::bytes(index_dir, &segment)? >= compaction::MIN_GENERATIONAL_BYTES;
    if sync_on_growth {
        segment::sync_files(index_dir, &segment)?;
    }
    let ngrams = segment.ngrams as usize;
    manifest.documents = indexed
        .iter()
        .map(|file| Document {
            path: file.state.path.clone(),
            len: file.state.len,
            modified_nanos: file.state.modified_nanos,
            active: true,
            searchable: file.searchable,
            segment_id,
        })
        .collect();
    manifest.segments = vec![segment];
    write_manifest_file(
        &index_dir.join(manifest::FILE_NAME),
        &manifest,
        durable || sync_on_growth,
    )?;
    Ok(BuildSummary {
        files: manifest
            .documents
            .iter()
            .filter(|document| document.searchable)
            .count(),
        bytes: searchable_bytes(&manifest),
        ngrams,
        index_dir: index_dir.to_owned(),
    })
}

pub fn refresh(
    index: &mut DiskIndex,
    requested_index_dir: Option<&Path>,
    budget: MemoryBudget,
) -> Result<RefreshOutcome> {
    let root = index.manifest.root.clone();
    let index_dir = resolve_index_dir(&root, requested_index_dir);
    let mut identity = git_state::identity(&root)?;
    let mut current_state = collect_files(&root, &index_dir, Some(index.source_len()))?;
    if index.same_source(&current_state)? && same_identity(&index.manifest, &identity) {
        return Ok(RefreshOutcome::Unchanged);
    }

    index.materialize()?;

    // Only writers lock. If the published snapshot has not changed, the first
    // scan is still relative to the correct indexed state. Like any worktree
    // scan, it does not freeze source edits made after that scan.
    let _writer = writer_lock(&index_dir)?;
    if !same_publication(&index_dir, &index.manifest)? {
        *index = load(&root, requested_index_dir)?;
        identity = git_state::identity(&root)?;
        current_state = collect_files(&root, &index_dir, Some(index.source_len()))?;
    }
    if current_state == index.manifest.source_state {
        if same_identity(&index.manifest, &identity) {
            return Ok(RefreshOutcome::Unchanged);
        }
        let mut manifest = index.manifest.clone();
        manifest.generation += 1;
        update_identity(&mut manifest, identity);
        write_manifest_file(
            &index_dir.join(manifest::FILE_NAME),
            &manifest,
            !compaction::small_base(&index_dir, &manifest)?,
        )?;
        return Ok(RefreshOutcome::CommitAdvanced);
    }
    // Always compare with the complete indexed worktree, including dirty
    // content. Git status against the new HEAD cannot describe that diff.
    write_incremental(index, &index_dir, current_state, identity, budget)
}

fn same_publication(directory: &Path, loaded: &Manifest) -> Result<bool> {
    let path = directory.join(manifest::FILE_NAME);
    if let Some(identity) = loaded.publication {
        // The loaded payload was checksum-verified. Writers replace manifests
        // atomically, never in place: the same content identity can reuse that
        // validated in-memory object. A changed identity triggers a full load.
        let mut header = [0; 28];
        File::open(path)?.read_exact(&mut header)?;
        return Ok(manifest::publication(&header) == Some(identity));
    }
    Ok(read_manifest(&path)? == *loaded)
}

fn same_identity(manifest: &Manifest, identity: &Option<git_state::GitIdentity>) -> bool {
    manifest.git_repository == identity.is_some()
        && manifest.git_head.as_ref() == identity.as_ref().and_then(|state| state.head.as_ref())
        && manifest.git_tree.as_ref() == identity.as_ref().and_then(|state| state.tree.as_ref())
}

fn update_identity(manifest: &mut Manifest, identity: Option<git_state::GitIdentity>) {
    manifest.git_repository = identity.is_some();
    manifest.git_head = identity.as_ref().and_then(|state| state.head.clone());
    manifest.git_tree = identity.and_then(|state| state.tree);
}

/// Explicit maintenance operates on indexed content, without scanning source.
/// The writer lock also protects document ownership and segment ID allocation.
pub fn compact(
    path: &Path,
    requested_index_dir: Option<&Path>,
    dry_run: bool,
) -> Result<compaction::Summary> {
    let root = resolve_root(path)?;
    let index_dir = resolve_index_dir(&root, requested_index_dir);
    let _writer = if dry_run {
        None
    } else {
        Some(writer_lock(&index_dir)?)
    };
    let mut manifest = load(&root, requested_index_dir)?.manifest;
    let had_multiple_segments = manifest.segments.len() > 1;
    compaction::prune(&mut manifest);
    let single_segment = compaction::small_base(&index_dir, &manifest)?;
    let inputs: Vec<_> = manifest
        .segments
        .iter()
        .skip(usize::from(!single_segment))
        .take(if single_segment {
            usize::MAX
        } else {
            compaction::MAX_MERGE_INPUTS
        })
        .map(|meta| meta.id)
        .collect();
    if inputs.len() < 2 && !(single_segment && had_multiple_segments) {
        return Ok(compaction::Summary::default());
    }
    if dry_run {
        return compaction::describe(&index_dir, &manifest, &inputs);
    }
    let summary = if single_segment {
        compaction::merge_to_base(&index_dir, &mut manifest, || next_segment_id(&index_dir))?
    } else {
        let id = next_segment_id(&index_dir);
        compaction::merge(&index_dir, &mut manifest, &inputs, id)?
    };
    manifest.generation += 1;
    write_manifest_file(&index_dir.join(manifest::FILE_NAME), &manifest, true)?;
    Ok(summary)
}

fn write_incremental(
    index: &DiskIndex,
    index_dir: &Path,
    current_state: Vec<FileState>,
    identity: Option<git_state::GitIdentity>,
    budget: MemoryBudget,
) -> Result<RefreshOutcome> {
    let mut manifest = index.manifest.clone();
    let changed = changed_file_count(&manifest.source_state, &current_state);
    let small = compaction::small_base(index_dir, &manifest)?;
    let old_states: HashMap<&Path, &FileState> = manifest
        .source_state
        .iter()
        .map(|state| (state.path.as_path(), state))
        .collect();
    let current_paths: HashSet<&Path> = current_state
        .iter()
        .map(|state| state.path.as_path())
        .collect();
    let mut active_by_path: HashMap<PathBuf, u32> = manifest
        .documents
        .iter()
        .enumerate()
        .filter(|(_, document)| document.active)
        .map(|(id, document)| (document.path.clone(), id as u32))
        .collect();

    for document in &mut manifest.documents {
        if document.active && !current_paths.contains(document.path.as_path()) {
            document.active = false;
        }
    }
    let mut pending = Vec::new();
    for state in &current_state {
        let unchanged = old_states
            .get(state.path.as_path())
            .is_some_and(|old| *old == state);
        if unchanged {
            continue;
        }
        let id = match active_by_path.get(&state.path) {
            Some(&id) => id,
            None => {
                let id = u32::try_from(manifest.documents.len())
                    .context("too many documents for a u32 document ID")?;
                manifest.documents.push(Document {
                    path: state.path.clone(),
                    len: state.len,
                    modified_nanos: state.modified_nanos,
                    active: true,
                    searchable: false,
                    segment_id: 0,
                });
                active_by_path.insert(state.path.clone(), id);
                id
            }
        };
        pending.push((id, state.clone()));
    }

    let segment_id = next_segment_id(index_dir);
    let (indexed, changes) = extract_files(&manifest.root, index_dir, budget, pending)?;
    let has_changes = !indexed.is_empty();
    for file in indexed {
        let document = &mut manifest.documents[file.id as usize];
        document.path = file.state.path;
        document.len = file.state.len;
        document.modified_nanos = file.state.modified_nanos;
        document.active = true;
        document.searchable = file.searchable;
        document.segment_id = segment_id;
    }
    manifest.generation += 1;
    update_identity(&mut manifest, identity);
    manifest.source_state = current_state;
    let summary = if small {
        if has_changes {
            manifest
                .segments
                .push(changes.write_with_durability(segment_id, false)?);
        }
        // Count every appended segment, including obsolete versions, just as
        // the old update path retained them until a rebuild. B is excluded.
        if compaction::small_rebuild_due(index_dir, &manifest)? {
            rebuild(index_dir, budget, manifest, false)?;
            return Ok(RefreshOutcome::Rebuilt);
        }
        compaction::Summary::default()
    } else {
        if has_changes {
            manifest.segments.push(changes.write(segment_id)?);
        }
        compaction::prune(&mut manifest);
        let inputs = compaction::automatic_plan(index_dir, &manifest)?;
        if inputs.is_empty() {
            compaction::Summary::default()
        } else {
            let id = next_segment_id(index_dir);
            compaction::merge(index_dir, &mut manifest, &inputs, id)?
        }
    };
    write_manifest_file(&index_dir.join(manifest::FILE_NAME), &manifest, !small)?;
    Ok(RefreshOutcome::Incremental {
        changed,
        compaction: summary,
    })
}

fn extract_files<I>(
    root: &Path,
    index_dir: &Path,
    budget: MemoryBudget,
    files: I,
) -> Result<(Vec<IndexedFile>, PostingsBuilder)>
where
    I: IntoIterator<Item = (u32, FileState)>,
{
    let files: Vec<_> = files.into_iter().collect();
    let workers = budget.workers(files.len());
    let mut postings = PostingsBuilder::new(index_dir, budget.record_bytes(workers));
    if files.is_empty() {
        return Ok((Vec::new(), postings));
    }
    let mut searchable = vec![false; files.len()];
    let next_file = AtomicUsize::new(0);
    std::thread::scope(|scope| -> Result<()> {
        let (sender, receiver) = mpsc::sync_channel(workers);
        for _ in 0..workers {
            let sender = sender.clone();
            let files = &files;
            let next_file = &next_file;
            // Separate producers let the collector use Rayon's parallel sort
            // without blocking Rayon workers on a full channel or a mutex.
            scope.spawn(move || {
                loop {
                    let position = next_file.fetch_add(1, Ordering::Relaxed);
                    let Some((_, state)) = files.get(position) else {
                        break;
                    };
                    let path = root.join(&state.path);
                    let result = hash_file_chunks(&path, |hashes| {
                        sender
                            .send(Ok((position, hashes)))
                            .map_err(|_| anyhow::anyhow!("index build cancelled"))
                    });
                    if let Err(error) = result {
                        let _ = sender.send(Err(error));
                        break;
                    }
                }
            });
        }
        drop(sender);
        for batch in receiver {
            let (position, hashes) = batch?;
            searchable[position] = true;
            postings.extend(files[position].0, &hashes)?;
        }
        Ok(())
    })?;
    let indexed = files
        .into_iter()
        .zip(searchable)
        .map(|((id, state), searchable)| IndexedFile {
            id,
            state,
            searchable,
        })
        .collect();
    Ok((indexed, postings))
}

fn hash_file_chunks(
    path: &Path,
    mut emit: impl FnMut(Vec<ngram::GramHash>) -> Result<()>,
) -> Result<()> {
    let file = File::open(path).with_context(|| format!("cannot read {}", path.display()))?;
    hash_reader_chunks(file, &mut emit).with_context(|| format!("cannot read {}", path.display()))
}

fn hash_reader_chunks(
    mut file: impl Read,
    mut emit: impl FnMut(Vec<ngram::GramHash>) -> Result<()>,
) -> Result<()> {
    let mut bytes = [0; CHUNK_BYTES];
    let mut retained = 0;
    loop {
        let mut len = retained;
        while len < bytes.len() {
            match file.read(&mut bytes[len..]) {
                Ok(0) => break,
                Ok(count) => len += count,
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => {
                    return Err(error.into());
                }
            }
        }
        if retained == 0 && is_binary(&bytes[..len]) {
            return Ok(());
        }
        if retained > 0 && len == retained {
            break;
        }
        emit(ngram::hashes_for_chunk(&bytes[..len]))?;
        if len < bytes.len() {
            break;
        }
        // The predicate for any gram depends only on its own bytes. Retaining
        // MAX_GRAM - 1 bytes preserves every gram spanning a chunk boundary.
        retained = ngram::MAX_GRAM - 1;
        bytes.copy_within(len - retained..len, 0);
    }
    Ok(())
}

pub fn load(path: &Path, requested_index_dir: Option<&Path>) -> Result<DiskIndex> {
    let root = resolve_root(path)?;
    let index_dir = resolve_index_dir(&root, requested_index_dir);
    load_full(&root, &index_dir)
}

fn load_full(root: &Path, index_dir: &Path) -> Result<DiskIndex> {
    let manifest = read_manifest(&index_dir.join(manifest::FILE_NAME))
        .with_context(|| "index not found; run `coderg index` or omit --no-refresh")?;
    if manifest.version != VERSION || manifest.root != root {
        bail!("index format or root mismatch; rebuild the index");
    }
    let segments = manifest
        .segments
        .iter()
        .map(|meta| segment::load(index_dir, meta))
        .collect::<Result<_>>()?;
    Ok(DiskIndex {
        manifest,
        segments,
        mapped: None,
    })
}

/// Search-only loader: header and segment directory are owned, file records
/// remain mapped. Maintenance/statistics continue to use the full loader.
pub fn load_for_search(path: &Path, requested_index_dir: Option<&Path>) -> Result<DiskIndex> {
    let root = resolve_root(path)?;
    let directory = resolve_index_dir(&root, requested_index_dir);
    let Some((manifest, mapped)) =
        manifest::view::View::open(&directory.join(manifest::FILE_NAME))?
    else {
        return load_full(&root, &directory);
    };
    if manifest.version != VERSION || manifest.root != root {
        bail!("index format or root mismatch; rebuild the index");
    }
    let segments = manifest
        .segments
        .iter()
        .map(|meta| segment::load(&directory, meta))
        .collect::<Result<_>>()?;
    Ok(DiskIndex {
        manifest,
        segments,
        mapped: Some(mapped),
    })
}

impl DiskIndex {
    fn source_len(&self) -> usize {
        self.mapped
            .as_ref()
            .map_or(self.manifest.source_state.len(), |v| v.source_len())
    }

    fn same_source(&self, current: &[FileState]) -> Result<bool> {
        self.mapped.as_ref().map_or_else(
            || Ok(current == self.manifest.source_state),
            |v| v.same_source(current),
        )
    }

    fn materialize(&mut self) -> Result<()> {
        if let Some(mapped) = &self.mapped {
            self.manifest = mapped.materialize()?;
            self.mapped = None;
        }
        Ok(())
    }

    pub fn document_path(&self, id: u32) -> Result<&Path> {
        if let Some(mapped) = &self.mapped {
            return mapped.document_path(id);
        }
        self.manifest
            .documents
            .get(id as usize)
            .map(|d| d.path.as_path())
            .context("invalid document ID; rebuild the index")
    }

    fn is_live(&self, id: u32, segment: Option<u64>) -> Result<bool> {
        if let Some(mapped) = &self.mapped {
            return mapped.is_live(id, segment);
        }
        let doc = self
            .manifest
            .documents
            .get(id as usize)
            .context("invalid document ID; rebuild the index")?;
        Ok(doc.active && doc.searchable && segment.is_none_or(|id| id == doc.segment_id))
    }

    pub fn postings(&self, hash: ngram::GramHash) -> Result<Vec<u32>> {
        let mut current = BTreeSet::new();
        for segment in &self.segments {
            for id in segment.postings(hash)? {
                if self.is_live(id, Some(segment.meta.id))? {
                    current.insert(id);
                }
            }
        }
        Ok(current.into_iter().collect())
    }

    pub fn all_document_ids(&self) -> Vec<u32> {
        if let Some(mapped) = &self.mapped {
            return mapped.active_document_ids();
        }
        self.manifest
            .documents
            .iter()
            .enumerate()
            .filter(|(_, d)| d.active && d.searchable)
            .map(|(id, _)| id as u32)
            .collect()
    }
}

fn read_manifest(path: &Path) -> Result<Manifest> {
    manifest::read(path)
}

fn write_manifest_file(path: &Path, manifest: &Manifest, durable: bool) -> Result<()> {
    write_manifest_bytes(path, &manifest::encode(manifest)?, durable)
}

fn write_manifest_bytes(path: &Path, bytes: &[u8], durable: bool) -> Result<()> {
    // NamedTempFile::persist replaces atomically on supported platforms,
    // including Windows. An interrupted publication leaves the old root valid.
    let parent = path.parent().context("manifest has no parent directory")?;
    #[cfg(unix)]
    if durable {
        File::open(parent.join("segments"))?.sync_all()?;
    }
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    {
        let mut writer = BufWriter::new(temporary.as_file_mut());
        writer.write_all(bytes)?;
        writer.flush()?;
    }
    if durable {
        temporary.as_file().sync_all()?;
    }
    temporary
        .persist(path)
        .with_context(|| format!("cannot publish {}", path.display()))?;
    #[cfg(unix)]
    if durable {
        File::open(parent)?.sync_all()?;
    }
    match fs::remove_file(path.with_extension("json")) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn writer_lock(index_dir: &Path) -> Result<File> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(index_dir.join("write.lock"))
        .context("cannot open index writer lock")?;
    file.lock().context("cannot lock index writer")?;
    Ok(file)
}

fn prepare_index_dir(root: &Path, index_dir: &Path) -> Result<()> {
    if index_dir == root {
        bail!("the index directory cannot be the search root");
    }
    fs::create_dir_all(index_dir).with_context(|| format!("cannot create {}", index_dir.display()))
}

fn next_segment_id(index_dir: &Path) -> u64 {
    let mut id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    while index_dir
        .join("segments")
        .join(format!("{id:020}.lookup"))
        .exists()
    {
        id = id.wrapping_add(1);
    }
    id
}

fn collect_files(
    root: &Path,
    index_dir: &Path,
    previous_file_count: Option<usize>,
) -> Result<Vec<FileState>> {
    let mut builder = WalkBuilder::new(root);
    builder.hidden(false).follow_links(false).filter_entry({
        let index_dir = index_dir.to_path_buf();
        move |entry| should_visit(entry, &index_dir)
    });
    if let Some(file_count) = previous_file_count.filter(|&count| count <= MAX_SERIAL_WALK_FILES) {
        let mut files = Vec::with_capacity(file_count);
        for entry in builder.build() {
            if let Some(state) = file_state(root, &entry?)? {
                files.push(state);
            }
        }
        files.sort_unstable_by(|left, right| left.path.cmp(&right.path));
        return Ok(files);
    }
    builder.threads(rayon::current_num_threads());
    let batches = Mutex::new(Vec::new());
    builder.build_parallel().run(|| {
        let mut collector = FileCollector {
            root,
            batches: &batches,
            files: Ok(Vec::new()),
        };
        Box::new(move |entry| collector.visit(entry))
    });
    let batches = batches
        .into_inner()
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    let mut files = Vec::with_capacity(batches.iter().map(Vec::len).sum());
    for batch in batches {
        files.extend(batch);
    }
    files.par_sort_unstable_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn file_state(root: &Path, entry: &DirEntry) -> Result<Option<FileState>> {
    if !entry.file_type().is_some_and(|kind| kind.is_file()) {
        return Ok(None);
    }
    let metadata = entry.metadata()?;
    let modified_nanos = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |duration| duration.as_nanos());
    Ok(Some(FileState {
        path: entry.path().strip_prefix(root)?.to_path_buf(),
        len: metadata.len(),
        modified_nanos,
    }))
}

fn should_visit(entry: &DirEntry, index_dir: &Path) -> bool {
    entry.path() != index_dir && entry.file_name() != ".git"
}

fn changed_file_count(old: &[FileState], current: &[FileState]) -> usize {
    let old: HashMap<&Path, &FileState> = old
        .iter()
        .map(|state| (state.path.as_path(), state))
        .collect();
    let current_paths: HashSet<&Path> = current.iter().map(|state| state.path.as_path()).collect();
    let changed = current
        .iter()
        .filter(|state| {
            old.get(state.path.as_path())
                .is_none_or(|prior| *prior != *state)
        })
        .count();
    changed
        + old
            .keys()
            .filter(|path| !current_paths.contains(**path))
            .count()
}

fn is_binary(bytes: &[u8]) -> bool {
    bytes.iter().take(8192).any(|&byte| byte == 0)
}

fn searchable_bytes(manifest: &Manifest) -> u64 {
    manifest
        .documents
        .iter()
        .filter(|document| document.active && document.searchable)
        .map(|document| document.len)
        .sum()
}

pub fn stats(path: &Path, requested_index_dir: Option<&Path>) -> Result<Stats> {
    let index = load(path, requested_index_dir)?;
    let index_dir = resolve_index_dir(&index.manifest.root, requested_index_dir);
    let mut index_bytes = fs::metadata(manifest::resolve_path(
        &index_dir.join(manifest::FILE_NAME),
    )?)?
    .len();
    for segment in &index.manifest.segments {
        index_bytes += fs::metadata(index_dir.join(&segment.lookup))?.len();
        index_bytes += fs::metadata(index_dir.join(&segment.postings))?.len();
    }
    let middle_bytes = index
        .manifest
        .segments
        .iter()
        .skip(1)
        .map(|meta| compaction::bytes(&index_dir, meta))
        .sum::<Result<u64>>()?;
    let base_bytes = index
        .manifest
        .segments
        .first()
        .map(|meta| compaction::bytes(&index_dir, meta))
        .transpose()?
        .unwrap_or(0);
    let middle_segments = index.manifest.segments.len().saturating_sub(1);
    Ok(Stats {
        root: index.manifest.root.clone(),
        files: index
            .manifest
            .documents
            .iter()
            .filter(|document| document.active && document.searchable)
            .count(),
        source_bytes: searchable_bytes(&index.manifest),
        ngrams: index
            .manifest
            .segments
            .iter()
            .map(|segment| segment.ngrams)
            .sum(),
        index_bytes,
        segments: index.manifest.segments.len(),
        git_tree: index.manifest.git_tree.clone(),
        middle_segments,
        middle_bytes,
        base_bytes,
        generational: base_bytes >= compaction::MIN_GENERATIONAL_BYTES,
        full_compaction_threshold_bytes: compaction::full_compaction_threshold(base_bytes),
        maintenance_due: if base_bytes < compaction::MIN_GENERATIONAL_BYTES {
            middle_bytes >= compaction::SMALL_REBUILD_BYTES
        } else {
            middle_segments > compaction::MAX_MIDDLE_SEGMENTS
                || middle_bytes >= compaction::full_compaction_threshold(base_bytes)
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunked_hashes_match_whole_files_including_boundaries() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("source");
        let mut random = 12345_u32;
        let bytes: Vec<_> = (0..CHUNK_BYTES * 5 + 17)
            .map(|_| {
                random ^= random << 13;
                random ^= random >> 17;
                random ^= random << 5;
                (random % 255 + 1) as u8
            })
            .collect();
        for len in [
            0,
            1,
            2,
            3,
            24,
            CHUNK_BYTES - 1,
            CHUNK_BYTES,
            CHUNK_BYTES + 1,
            bytes.len(),
        ] {
            fs::write(&path, &bytes[..len]).unwrap();
            let mut actual = ngram::GramHashSet::default();
            let mut batches = 0;
            hash_file_chunks(&path, |hashes| {
                actual.extend(hashes);
                batches += 1;
                Ok(())
            })
            .unwrap();
            assert!(batches >= 1, "even an empty text file is searchable");
            assert_eq!(
                actual,
                ngram::hashes_for_document(&bytes[..len]),
                "length {len}"
            );
        }
        for nul_position in [0, 8191, 8192, CHUNK_BYTES + 1] {
            let mut bytes = bytes.clone();
            bytes[nul_position] = 0;
            fs::write(&path, &bytes).unwrap();
            let mut actual = ngram::GramHashSet::default();
            hash_file_chunks(&path, |hashes| {
                actual.extend(hashes);
                Ok(())
            })
            .unwrap();
            if nul_position < 8192 {
                assert!(actual.is_empty());
            } else {
                assert_eq!(actual, ngram::hashes_for_document(&bytes));
            }
        }
    }

    #[test]
    fn read_failure_keeps_published_index_and_cancels_workers() {
        let directory = tempfile::tempdir().unwrap();
        let manifest = directory.path().join("manifest.json");
        fs::write(&manifest, "previous manifest").unwrap();
        let result = extract_files(
            directory.path(),
            directory.path(),
            "16".parse().unwrap(),
            (0..100).map(|id| (id, state(&format!("missing-{id}"), 0))),
        );
        assert!(result.is_err());
        assert_eq!(fs::read_to_string(manifest).unwrap(), "previous manifest");
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }

    #[test]
    fn serial_and_parallel_snapshots_agree_even_with_a_stale_size_hint() {
        let root = tempfile::tempdir().unwrap();
        let index_dir = root.path().join("custom-index");
        for directory in [".git", "custom-index", ".hidden", "nested"] {
            fs::create_dir(root.path().join(directory)).unwrap();
        }
        fs::write(root.path().join(".ignore"), "*.skip\n!keep.skip\n").unwrap();
        for path in [
            ".git/config",
            "custom-index/manifest.json",
            ".hidden/source.rs",
            "nested/source.rs",
            "nested/hidden.skip",
            "nested/keep.skip",
        ] {
            fs::write(root.path().join(path), "contents\n").unwrap();
        }
        // The old snapshot can be tiny even after a large directory is added.
        for number in 0..600 {
            fs::write(root.path().join(format!("nested/{number:04}.rs")), "x\n").unwrap();
        }
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("nested", root.path().join("linked-dir")).unwrap();
            std::os::unix::fs::symlink("nested/source.rs", root.path().join("linked-file"))
                .unwrap();
        }

        let serial = collect_files(root.path(), &index_dir, Some(0)).unwrap();
        let parallel = collect_files(root.path(), &index_dir, None).unwrap();
        assert_eq!(serial, parallel);
        assert_eq!(serial.len(), 604);
        let paths: Vec<_> = serial.iter().map(|file| file.path.as_path()).collect();
        assert!(paths.contains(&Path::new(".hidden/source.rs")));
        assert!(paths.contains(&Path::new("nested/keep.skip")));
        assert!(!paths.contains(&Path::new("nested/hidden.skip")));

        let missing = root.path().join("missing");
        assert!(collect_files(&missing, &index_dir, Some(0)).is_err());
        assert!(collect_files(&missing, &index_dir, None).is_err());
    }

    #[test]
    fn counts_added_changed_and_deleted_files() {
        let old = vec![state("a", 1), state("b", 2), state("gone", 3)];
        let current = vec![state("a", 1), state("b", 4), state("new", 1)];
        assert_eq!(changed_file_count(&old, &current), 3);
    }

    fn state(path: &str, len: u64) -> FileState {
        FileState {
            path: PathBuf::from(path),
            len,
            modified_nanos: 0,
        }
    }
}
