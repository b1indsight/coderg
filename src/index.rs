use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use ignore::{DirEntry, WalkBuilder, WalkState};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{git_state, ngram, segment};

const VERSION: u32 = 4;
const MAX_SEGMENTS: usize = 8;
const MIN_LARGE_CHANGE: usize = 64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    version: u32,
    pub root: PathBuf,
    generation: u64,
    git_repository: bool,
    git_head: Option<String>,
    pub git_tree: Option<String>,
    source_state: Vec<FileState>,
    pub documents: Vec<Document>,
    pub segments: Vec<segment::SegmentMeta>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct FileState {
    path: PathBuf,
    len: u64,
    modified_nanos: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub path: PathBuf,
    pub len: u64,
    modified_nanos: u128,
    pub active: bool,
    pub searchable: bool,
    pub segment_id: u64,
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
}

pub enum RefreshOutcome {
    Unchanged,
    CommitAdvanced,
    Incremental { changed: usize },
    SwitchedToCachedTree,
    Rebuilt,
}

pub struct DiskIndex {
    pub manifest: Manifest,
    segments: Vec<segment::Segment>,
}

struct IndexedFile {
    id: u32,
    state: FileState,
    searchable: bool,
    hashes: Vec<ngram::GramHash>,
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

pub fn build(path: &Path, requested_index_dir: Option<&Path>) -> Result<BuildSummary> {
    let root = resolve_root(path)?;
    let index_dir = resolve_index_dir(&root, requested_index_dir);
    prepare_index_dir(&root, &index_dir)?;
    let source_state = collect_files(&root, &index_dir)?;
    let repository_state = git_state::inspect(&root, &index_dir)?;
    let segment_id = next_segment_id(&index_dir);
    let mut indexed = index_files(
        &root,
        source_state
            .iter()
            .cloned()
            .enumerate()
            .map(|(id, state)| (id as u32, state)),
    )?;
    let postings = postings_from_files(&mut indexed);
    let segment = segment::write(&index_dir, segment_id, postings)?;
    let ngrams = segment.ngrams as usize;
    let documents = indexed
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
    let manifest = Manifest {
        version: VERSION,
        root: root.clone(),
        generation: 1,
        git_repository: repository_state.is_some(),
        git_head: repository_state
            .as_ref()
            .and_then(|state| state.head.clone()),
        git_tree: repository_state
            .as_ref()
            .and_then(|state| state.tree.clone()),
        source_state,
        documents,
        segments: vec![segment],
    };
    write_manifest(&index_dir, &manifest)?;
    cache_manifest_if_clean(
        &index_dir,
        &manifest,
        repository_state.is_some_and(|state| state.clean),
    )?;
    Ok(BuildSummary {
        files: manifest
            .documents
            .iter()
            .filter(|document| document.searchable)
            .count(),
        bytes: searchable_bytes(&manifest),
        ngrams,
        index_dir,
    })
}

pub fn refresh(index: &DiskIndex, requested_index_dir: Option<&Path>) -> Result<RefreshOutcome> {
    let root = &index.manifest.root;
    let index_dir = resolve_index_dir(root, requested_index_dir);

    // Most searches stay on the same commit. In that case the persisted file
    // snapshot is enough to detect worktree changes, avoiding a full Git status
    // walk. A changed commit still uses status below because clean tree-cache
    // switching requires an authoritative Git answer.
    if index.manifest.git_repository
        && let Some(identity) = git_state::identity(root)?
        && identity.head == index.manifest.git_head
        && identity.tree == index.manifest.git_tree
    {
        let current_state = collect_files(root, &index_dir)?;
        if current_state == index.manifest.source_state {
            return Ok(RefreshOutcome::Unchanged);
        }
        let changed = changed_file_count(&index.manifest.source_state, &current_state);
        let large_change =
            changed >= MIN_LARGE_CHANGE && changed.saturating_mul(5) > current_state.len().max(1);
        if index.manifest.segments.len() >= MAX_SEGMENTS || large_change {
            build(root, requested_index_dir)?;
            return Ok(RefreshOutcome::Rebuilt);
        }
        write_incremental(
            index,
            &index_dir,
            current_state,
            identity.head,
            identity.tree,
            None,
            false,
        )?;
        return Ok(RefreshOutcome::Incremental { changed });
    }

    let repository_state = if index.manifest.git_repository {
        git_state::inspect(root, &index_dir)?
    } else {
        None
    };
    let current_head = repository_state
        .as_ref()
        .and_then(|state| state.head.clone());
    let current_tree = repository_state
        .as_ref()
        .and_then(|state| state.tree.clone());

    if repository_state.as_ref().is_some_and(|state| state.clean) {
        if current_head == index.manifest.git_head && current_tree == index.manifest.git_tree {
            return Ok(RefreshOutcome::Unchanged);
        }
        if current_tree != index.manifest.git_tree
            && current_tree.is_some()
            && restore_cached_tree(
                index,
                &index_dir,
                current_head.as_deref(),
                current_tree.as_deref(),
            )?
        {
            return Ok(RefreshOutcome::SwitchedToCachedTree);
        }
        if current_tree == index.manifest.git_tree {
            let mut manifest = index.manifest.clone();
            manifest.generation += 1;
            manifest.git_head = current_head;
            write_manifest(&index_dir, &manifest)?;
            cache_manifest_if_clean(&index_dir, &manifest, true)?;
            return Ok(RefreshOutcome::CommitAdvanced);
        }
    }

    let current_state = collect_files(root, &index_dir)?;
    if current_state == index.manifest.source_state {
        if current_head == index.manifest.git_head && current_tree == index.manifest.git_tree {
            return Ok(RefreshOutcome::Unchanged);
        }
        let mut manifest = index.manifest.clone();
        manifest.generation += 1;
        manifest.git_head = current_head;
        manifest.git_tree = current_tree;
        write_manifest(&index_dir, &manifest)?;
        cache_manifest_if_clean(
            &index_dir,
            &manifest,
            repository_state.as_ref().is_some_and(|state| state.clean),
        )?;
        return Ok(RefreshOutcome::CommitAdvanced);
    }

    let known_changed = repository_state
        .as_ref()
        .filter(|state| !state.clean)
        .and_then(|state| state.changed_paths.as_ref());
    let changed = known_changed.map_or_else(
        || changed_file_count(&index.manifest.source_state, &current_state),
        HashSet::len,
    );
    let large_change =
        changed >= MIN_LARGE_CHANGE && changed.saturating_mul(5) > current_state.len().max(1);
    if index.manifest.segments.len() >= MAX_SEGMENTS || large_change {
        build(root, requested_index_dir)?;
        return Ok(RefreshOutcome::Rebuilt);
    }
    write_incremental(
        index,
        &index_dir,
        current_state,
        current_head,
        current_tree,
        known_changed,
        repository_state.as_ref().is_some_and(|state| state.clean),
    )?;
    Ok(RefreshOutcome::Incremental { changed })
}

fn write_incremental(
    index: &DiskIndex,
    index_dir: &Path,
    current_state: Vec<FileState>,
    current_head: Option<String>,
    current_tree: Option<String>,
    known_changed: Option<&HashSet<PathBuf>>,
    cache_clean: bool,
) -> Result<()> {
    let mut manifest = index.manifest.clone();
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
        let unchanged = match known_changed {
            Some(changed) => {
                old_states.contains_key(state.path.as_path()) && !changed.contains(&state.path)
            }
            None => old_states
                .get(state.path.as_path())
                .is_some_and(|old| *old == state),
        };
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

    if !pending.is_empty() {
        let segment_id = next_segment_id(index_dir);
        let mut indexed = index_files(&manifest.root, pending)?;
        let postings = postings_from_files(&mut indexed);
        let segment = segment::write(index_dir, segment_id, postings)?;
        for file in indexed {
            let document = &mut manifest.documents[file.id as usize];
            document.path = file.state.path;
            document.len = file.state.len;
            document.modified_nanos = file.state.modified_nanos;
            document.active = true;
            document.searchable = file.searchable;
            document.segment_id = segment_id;
        }
        manifest.segments.push(segment);
    }
    manifest.generation += 1;
    manifest.git_head = current_head;
    manifest.git_tree = current_tree;
    manifest.source_state = current_state;
    write_manifest(index_dir, &manifest)?;
    cache_manifest_if_clean(index_dir, &manifest, cache_clean)?;
    Ok(())
}

fn index_files<I>(root: &Path, files: I) -> Result<Vec<IndexedFile>>
where
    I: IntoIterator<Item = (u32, FileState)>,
{
    let files: Vec<_> = files.into_iter().collect();
    files
        .into_par_iter()
        .map(|(id, state)| {
            let full_path = root.join(&state.path);
            let bytes = fs::read(&full_path)
                .with_context(|| format!("cannot read {}", full_path.display()))?;
            let searchable = !is_binary(&bytes);
            let hashes = if searchable {
                // Keep only the unique keys, not the hash table's spare buckets.
                ngram::hashes_for_document(&bytes).into_iter().collect()
            } else {
                Vec::new()
            };
            Ok(IndexedFile {
                id,
                state,
                searchable,
                hashes,
            })
        })
        .collect()
}

fn postings_from_files(files: &mut [IndexedFile]) -> Vec<(ngram::GramHash, u32)> {
    let count = files.iter().map(|file| file.hashes.len()).sum();
    let mut postings = Vec::with_capacity(count);
    for file in files {
        // Release each file's keys as we assemble the contiguous posting records.
        postings.extend(
            std::mem::take(&mut file.hashes)
                .into_iter()
                .map(|hash| (hash, file.id)),
        );
    }
    postings
}

pub fn load(path: &Path, requested_index_dir: Option<&Path>) -> Result<DiskIndex> {
    let root = resolve_root(path)?;
    let index_dir = resolve_index_dir(&root, requested_index_dir);
    let manifest = read_manifest(&index_dir.join("manifest.json"))
        .with_context(|| "index not found; run `coderg index` or omit --no-refresh")?;
    if manifest.version != VERSION || manifest.root != root {
        bail!("index format or root mismatch; rebuild the index");
    }
    let segments = manifest
        .segments
        .iter()
        .map(|meta| segment::load(&index_dir, meta))
        .collect::<Result<_>>()?;
    Ok(DiskIndex { manifest, segments })
}

impl DiskIndex {
    pub fn postings(&self, hash: ngram::GramHash) -> Result<Vec<u32>> {
        let mut current = BTreeSet::new();
        for segment in &self.segments {
            for id in segment.postings(hash)? {
                let Some(document) = self.manifest.documents.get(id as usize) else {
                    bail!("segment contains an invalid document ID; rebuild the index");
                };
                if document.active && document.searchable && document.segment_id == segment.meta.id
                {
                    current.insert(id);
                }
            }
        }
        Ok(current.into_iter().collect())
    }

    pub fn all_document_ids(&self) -> Vec<u32> {
        self.manifest
            .documents
            .iter()
            .enumerate()
            .filter(|(_, document)| document.active && document.searchable)
            .map(|(id, _)| id as u32)
            .collect()
    }
}

fn restore_cached_tree(
    current: &DiskIndex,
    index_dir: &Path,
    head: Option<&str>,
    tree: Option<&str>,
) -> Result<bool> {
    let Some(tree) = tree else {
        return Ok(false);
    };
    let cache_path = index_dir.join("manifests").join(format!("{tree}.json"));
    if !cache_path.is_file() {
        return Ok(false);
    }
    let mut cached = read_manifest(&cache_path)?;
    if cached.version != VERSION || cached.root != current.manifest.root {
        return Ok(false);
    }
    cached.generation = current.manifest.generation + 1;
    cached.git_head = head.map(str::to_owned);
    cached.git_tree = Some(tree.to_owned());
    // Checkout/reset commonly changes mtimes even when the cached Git tree is
    // byte-for-byte identical. Rebase the metadata snapshot so the next search
    // does not create a redundant overlay segment.
    let source_state = collect_files(&cached.root, index_dir)?;
    let state_by_path: HashMap<&Path, &FileState> = source_state
        .iter()
        .map(|state| (state.path.as_path(), state))
        .collect();
    for document in &mut cached.documents {
        if let Some(state) = state_by_path.get(document.path.as_path()) {
            document.len = state.len;
            document.modified_nanos = state.modified_nanos;
        }
    }
    cached.source_state = source_state;
    write_manifest(index_dir, &cached)?;
    Ok(true)
}

fn cache_manifest_if_clean(index_dir: &Path, manifest: &Manifest, clean: bool) -> Result<()> {
    let Some(tree) = &manifest.git_tree else {
        return Ok(());
    };
    if !clean {
        return Ok(());
    }
    let manifests_dir = index_dir.join("manifests");
    fs::create_dir_all(&manifests_dir)?;
    write_manifest_file(&manifests_dir.join(format!("{tree}.json")), manifest)
}

fn read_manifest(path: &Path) -> Result<Manifest> {
    Ok(serde_json::from_reader(BufReader::new(File::open(path)?))?)
}

fn write_manifest(index_dir: &Path, manifest: &Manifest) -> Result<()> {
    write_manifest_file(&index_dir.join("manifest.json"), manifest)
}

fn write_manifest_file(path: &Path, manifest: &Manifest) -> Result<()> {
    let tmp = path.with_extension("json.tmp");
    let mut file = BufWriter::new(File::create(&tmp)?);
    serde_json::to_writer_pretty(&mut file, manifest)?;
    file.flush()?;
    replace(tmp, path.to_path_buf())
}

fn replace(from: PathBuf, to: PathBuf) -> Result<()> {
    if cfg!(windows) && to.exists() {
        fs::remove_file(&to)?;
    }
    fs::rename(&from, &to).with_context(|| format!("cannot replace {}", to.display()))
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

fn collect_files(root: &Path, index_dir: &Path) -> Result<Vec<FileState>> {
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .follow_links(false)
        .threads(rayon::current_num_threads())
        .filter_entry({
            let index_dir = index_dir.to_path_buf();
            move |entry| should_visit(entry, &index_dir)
        });
    let files = Mutex::new(Vec::new());
    let error = Mutex::new(None);
    builder.build_parallel().run(|| {
        let files = &files;
        let error = &error;
        Box::new(move |result| {
            let state = result
                .map_err(anyhow::Error::from)
                .and_then(|entry| file_state(root, &entry));
            match state {
                Ok(Some(state)) => files.lock().unwrap().push(state),
                Ok(None) => {}
                Err(cause) => {
                    *error.lock().unwrap() = Some(cause);
                    return WalkState::Quit;
                }
            }
            WalkState::Continue
        })
    });
    if let Some(error) = error.into_inner().unwrap() {
        return Err(error);
    }
    let mut files = files.into_inner().unwrap();
    files.sort_by(|left, right| left.path.cmp(&right.path));
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
    let mut index_bytes = fs::metadata(index_dir.join("manifest.json"))?.len();
    for segment in &index.manifest.segments {
        index_bytes += fs::metadata(index_dir.join(&segment.lookup))?.len();
        index_bytes += fs::metadata(index_dir.join(&segment.postings))?.len();
    }
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
