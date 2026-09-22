//! Experimental in-memory snapshots. Not part of the production index format.
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
};

use git2::{ObjectType, Oid};
use serde::Serialize;

use crate::ngram;

#[derive(Clone)]
pub struct Blob {
    pub oid: Oid,
    pub bytes: Arc<[u8]>,
}

impl Blob {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            oid: Oid::hash_object(ObjectType::Blob, &bytes).unwrap(),
            bytes: bytes.into(),
        }
    }
}

pub type Files = BTreeMap<String, Blob>;
type View = BTreeMap<String, Document>;

struct Segment {
    documents: Vec<(String, Blob)>,
    postings: HashMap<u32, Vec<usize>>,
}

#[derive(Clone)]
struct Document {
    segment: Arc<Segment>,
    local_id: usize,
}

impl Document {
    fn blob(&self) -> &Blob {
        &self.segment.documents[self.local_id].1
    }
}

#[derive(Default, Debug, Serialize)]
pub struct Work {
    pub extracted_files: usize,
    pub extracted_bytes: usize,
    pub new_segments: usize,
    pub snapshot_hit: bool,
    pub overlay_entries: usize,
}

fn extend(view: &mut View, pending: Files, work: &mut Work) {
    if pending.is_empty() {
        return;
    }
    let mut segment = Segment {
        documents: pending.into_iter().collect(),
        postings: HashMap::new(),
    };
    for (id, (_, blob)) in segment.documents.iter().enumerate() {
        // The prototype uses the production sparse-gram extractor, but keeps
        // uncompressed postings in RAM. It intentionally has no disk writer.
        let mut grams = ngram::hashes_for_chunk(&blob.bytes);
        grams.sort_unstable();
        grams.dedup();
        for gram in grams {
            segment.postings.entry(gram).or_default().push(id);
        }
        work.extracted_files += 1;
        work.extracted_bytes += blob.bytes.len();
    }
    work.new_segments += 1;
    let segment = Arc::new(segment);
    for (local_id, (path, _)) in segment.documents.iter().enumerate() {
        view.insert(
            path.clone(),
            Document {
                segment: segment.clone(),
                local_id,
            },
        );
    }
}

fn materialize(files: &Files, sources: &[&View], work: &mut Work) -> View {
    let mut view = View::new();
    let mut pending = Files::new();
    for (path, blob) in files {
        let existing = sources
            .iter()
            .find_map(|source| source.get(path).filter(|doc| doc.blob().oid == blob.oid));
        if let Some(doc) = existing {
            view.insert(path.clone(), doc.clone());
        } else {
            pending.insert(path.clone(), blob.clone());
        }
    }
    extend(&mut view, pending, work);
    view
}

#[derive(Default)]
pub struct SnapshotIndex {
    snapshots: HashMap<Oid, Arc<View>>,
    base: Arc<View>,
    // None is a tombstone: deleting a file must not expose its base version.
    overlay: BTreeMap<String, Option<Document>>,
    current: View,
}

impl SnapshotIndex {
    pub fn refresh(&mut self, tree: Oid, committed: &Files, working: &Files) -> Work {
        let mut work = Work::default();
        let base = if let Some(snapshot) = self.snapshots.get(&tree) {
            work.snapshot_hit = true;
            snapshot.clone()
        } else {
            // Promotion is per path AND content identity, including partial commits.
            let snapshot = Arc::new(materialize(
                committed,
                &[&self.current, &self.base],
                &mut work,
            ));
            self.snapshots.insert(tree, snapshot.clone());
            snapshot
        };
        let current = materialize(working, &[&base, &self.current], &mut work);
        let mut overlay = BTreeMap::new();
        for (path, doc) in &current {
            if base
                .get(path)
                .is_none_or(|base_doc| base_doc.blob().oid != doc.blob().oid)
            {
                overlay.insert(path.clone(), Some(doc.clone()));
            }
        }
        for path in base.keys() {
            if !current.contains_key(path) {
                overlay.insert(path.clone(), None);
            }
        }
        work.overlay_entries = overlay.len();
        self.base = base;
        self.overlay = overlay;
        self.current = current;
        work
    }

    pub fn search(&self, needle: &[u8]) -> Vec<String> {
        search(&self.current, needle)
    }

    pub fn retained(&self) -> Retained {
        let views = self
            .snapshots
            .values()
            .map(|view| view.as_ref())
            .chain([&self.current]);
        retained(views)
    }
}

/// Same in-memory representation, but reuse only the immediately preceding
/// working view. Models the current refresh policy with stronger content-based
/// detection than production's size/mtime check; not the production benchmark.
#[derive(Default)]
pub struct IncrementalIndex {
    current: View,
}

impl IncrementalIndex {
    pub fn refresh(&mut self, working: &Files) -> Work {
        let mut work = Work::default();
        self.current = materialize(working, &[&self.current], &mut work);
        work
    }

    pub fn search(&self, needle: &[u8]) -> Vec<String> {
        search(&self.current, needle)
    }

    pub fn retained(&self) -> Retained {
        retained([&self.current])
    }
}

fn search(view: &View, needle: &[u8]) -> Vec<String> {
    let Some(gram) = ngram::covering_hashes(needle, 1).first().copied() else {
        return view
            .iter()
            .filter(|(_, doc)| contains(&doc.blob().bytes, needle))
            .map(|(path, _)| path.clone())
            .collect();
    };
    let mut seen = HashSet::new();
    let mut matches = Vec::new();
    for doc in view.values() {
        if !seen.insert(Arc::as_ptr(&doc.segment)) {
            continue;
        }
        if let Some(ids) = doc.segment.postings.get(&gram) {
            for &id in ids {
                let (path, blob) = &doc.segment.documents[id];
                if view.get(path).is_some_and(|active| {
                    Arc::ptr_eq(&active.segment, &doc.segment) && active.local_id == id
                }) && contains(&blob.bytes, needle)
                {
                    matches.push(path.clone());
                }
            }
        }
    }
    matches.sort();
    matches
}

pub fn contains(bytes: &[u8], needle: &[u8]) -> bool {
    needle.is_empty() || memchr::memmem::find(bytes, needle).is_some()
}

#[derive(Debug, Serialize)]
pub struct Retained {
    pub segments: usize,
    pub source_bytes: usize,
    pub posting_records: usize,
    // Logical payload only: excludes maps, allocator overhead and snapshot maps.
    pub posting_payload_bytes: usize,
}

fn retained<'a>(views: impl IntoIterator<Item = &'a View>) -> Retained {
    let mut seen = HashSet::new();
    let mut result = Retained {
        segments: 0,
        source_bytes: 0,
        posting_records: 0,
        posting_payload_bytes: 0,
    };
    for view in views {
        for doc in view.values() {
            if seen.insert(Arc::as_ptr(&doc.segment)) {
                result.segments += 1;
                result.source_bytes += doc
                    .segment
                    .documents
                    .iter()
                    .map(|(_, blob)| blob.bytes.len())
                    .sum::<usize>();
                let records = doc.segment.postings.values().map(Vec::len).sum::<usize>();
                result.posting_records += records;
                result.posting_payload_bytes +=
                    records * size_of::<usize>() + doc.segment.postings.len() * size_of::<u32>();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(entries: &[(&str, &str)]) -> Files {
        entries
            .iter()
            .map(|(p, b)| (p.to_string(), Blob::new(b.as_bytes().to_vec())))
            .collect()
    }

    fn tree(n: u8) -> Oid {
        Oid::from_bytes(&[n; 20]).unwrap()
    }

    fn check(index: &SnapshotIndex, working: &Files) {
        for needle in ["alpha", "beta", "gamma", "delta", "absent", "", "a"] {
            let expected: Vec<_> = working
                .iter()
                .filter(|(_, b)| contains(&b.bytes, needle.as_bytes()))
                .map(|(p, _)| p.clone())
                .collect();
            assert_eq!(index.search(needle.as_bytes()), expected, "{needle}");
        }
    }

    #[test]
    fn dirty_initialization_deletion_discard_and_cached_switch() {
        let a = files(&[("x", "alpha"), ("y", "gamma")]);
        let b = files(&[("x", "beta"), ("new", "delta")]);
        let mut index = SnapshotIndex::default();
        assert_eq!(index.refresh(tree(1), &a, &b).extracted_files, 4);
        check(&index, &b);
        assert_eq!(index.overlay.len(), 3);
        assert_eq!(index.refresh(tree(2), &b, &b).extracted_files, 0);
        assert!(index.overlay.is_empty());
        check(&index, &b);
        for _ in 0..3 {
            assert_eq!(index.refresh(tree(1), &a, &a).extracted_files, 0);
            check(&index, &a);
            assert_eq!(index.refresh(tree(2), &b, &b).extracted_files, 0);
            check(&index, &b);
        }
        index.refresh(tree(2), &b, &a);
        assert_eq!(index.refresh(tree(2), &b, &b).extracted_files, 0);
        check(&index, &b);
    }

    #[test]
    fn partial_commit_does_not_promote_dirty_bytes() {
        let a = files(&[("x", "alpha"), ("y", "alpha")]);
        let staged = files(&[("x", "beta"), ("y", "alpha")]);
        let working = files(&[("x", "gamma"), ("y", "delta")]);
        let mut index = SnapshotIndex::default();
        index.refresh(tree(1), &a, &working);
        assert_eq!(index.refresh(tree(2), &staged, &working).extracted_files, 1);
        check(&index, &working);
        assert_eq!(index.refresh(tree(2), &staged, &staged).extracted_files, 0);
        check(&index, &staged);
        assert_eq!(index.refresh(tree(1), &a, &a).extracted_files, 0);
        check(&index, &a);
    }
}
