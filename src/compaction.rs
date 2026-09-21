//! Small indexes rebuild at 8 MiB of deltas; large indexes compact B and M.
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result};
use serde::Serialize;

use crate::{
    manifest::{Manifest, SegmentMeta},
    segment::{self, Records, Segment},
};

pub const MAX_MIDDLE_SEGMENTS: usize = 8;
pub const MIN_GENERATIONAL_BYTES: u64 = 32 * 1024 * 1024;
pub const SMALL_REBUILD_BYTES: u64 = 8 * 1024 * 1024;
const MAX_AUTO_INPUTS: usize = 4;
pub const MAX_AUTO_BYTES: u64 = 32 * 1024 * 1024;
pub const MAX_MERGE_INPUTS: usize = 32;
pub const FULL_COMPACT_PERCENT: u64 = 25;
const MIN_FULL_COMPACT_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Debug, Default, Serialize)]
pub struct Summary {
    pub inputs: Vec<u64>,
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub output: Option<u64>,
    pub full: bool,
}

pub fn bytes(directory: &Path, meta: &SegmentMeta) -> Result<u64> {
    Ok(fs::metadata(directory.join(&meta.lookup))?.len()
        + fs::metadata(directory.join(&meta.postings))?.len())
}

pub fn small_base(directory: &Path, manifest: &Manifest) -> Result<bool> {
    manifest
        .segments
        .first()
        .map(|base| Ok(bytes(directory, base)? < MIN_GENERATIONAL_BYTES))
        .unwrap_or(Ok(false))
}

pub fn small_rebuild_due(directory: &Path, manifest: &Manifest) -> Result<bool> {
    let incremental_bytes = manifest
        .segments
        .iter()
        .skip(1)
        .map(|meta| bytes(directory, meta))
        .sum::<Result<u64>>()?;
    Ok(incremental_bytes >= SMALL_REBUILD_BYTES)
}

/// Discard dead M references, but keep the first segment as the stable B anchor.
/// Files remain immutable on disk so concurrent readers retain a valid snapshot.
pub fn prune(manifest: &mut Manifest) {
    let live: HashSet<_> = manifest
        .documents
        .iter()
        .filter(|doc| doc.active && doc.searchable)
        .map(|doc| doc.segment_id)
        .collect();
    let base = manifest.segments.first().map(|meta| meta.id);
    manifest
        .segments
        .retain(|meta| Some(meta.id) == base || live.contains(&meta.id));
}

pub fn automatic_plan(directory: &Path, manifest: &Manifest) -> Result<Vec<u64>> {
    let Some(base) = manifest.segments.first() else {
        return Ok(Vec::new());
    };
    let base_bytes = bytes(directory, base)?;
    if base_bytes < MIN_GENERATIONAL_BYTES {
        return Ok(Vec::new());
    }
    let sizes = manifest
        .segments
        .iter()
        .skip(1)
        .enumerate()
        .map(|(order, meta)| Ok((bytes(directory, meta)?, order, meta.id)))
        .collect::<Result<Vec<_>>>()?;
    let middle_bytes = sizes.iter().map(|(bytes, _, _)| bytes).sum::<u64>();
    if middle_bytes >= full_compaction_threshold(base_bytes) {
        // Bound fan-in for legacy indexes with many segments. Reduce M first
        // if necessary; normal two-generation updates generally have <= 9.
        return Ok(manifest
            .segments
            .iter()
            .skip(usize::from(manifest.segments.len() > MAX_MERGE_INPUTS))
            .take(MAX_MERGE_INPUTS)
            .map(|meta| meta.id)
            .collect());
    }
    if sizes.len() <= MAX_MIDDLE_SEGMENTS {
        return Ok(Vec::new());
    }
    Ok(plan_sizes(sizes))
}

pub fn full_compaction_threshold(base_bytes: u64) -> u64 {
    base_bytes
        .saturating_mul(FULL_COMPACT_PERCENT)
        .div_ceil(100)
        .max(MIN_FULL_COMPACT_BYTES)
}

/// Snapshot segment order follows IDs, not size: protect the largest segment
/// rather than assuming that the oldest segment is still the main baseline.
pub fn snapshot_plan(directory: &Path, segments: &[SegmentMeta]) -> Result<Vec<u64>> {
    let sizes = segments
        .iter()
        .enumerate()
        .map(|(order, meta)| Ok((bytes(directory, meta)?, order, meta.id)))
        .collect::<Result<Vec<_>>>()?;
    Ok(snapshot_plan_sizes(sizes))
}

fn snapshot_plan_sizes(mut sizes: Vec<(u64, usize, u64)>) -> Vec<u64> {
    sizes.sort_unstable();
    let Some((base_bytes, _, base_id)) = sizes.pop() else {
        return Vec::new();
    };
    let delta_bytes = sizes.iter().map(|(bytes, _, _)| bytes).sum::<u64>();
    if base_bytes < MIN_GENERATIONAL_BYTES {
        // Small snapshots accumulate bytes, not merge work on every few
        // commits. The caller batches a full consolidation if fan-in is high.
        return if delta_bytes >= SMALL_REBUILD_BYTES {
            std::iter::once(base_id)
                .chain(sizes.into_iter().map(|(_, _, id)| id))
                .collect()
        } else {
            Vec::new()
        };
    }
    // Commit baselines accumulate bytes, regardless of segment count. The
    // caller performs bounded fan-in batches when consolidation is due.
    if delta_bytes < full_compaction_threshold(base_bytes) {
        return Vec::new();
    }
    std::iter::once(base_id)
        .chain(sizes.into_iter().map(|(_, _, id)| id))
        .collect()
}

fn plan_sizes(mut sizes: Vec<(u64, usize, u64)>) -> Vec<u64> {
    sizes.sort_unstable();
    // Merge similarly sized inputs. Small new segments do not repeatedly drag a
    // large accumulated segment into every merge. Equal sizes prefer older M.
    for start in 0..sizes.len() {
        let mut total = 0_u64;
        let mut selected = Vec::new();
        for &(size, _, id) in sizes.iter().skip(start).take(MAX_AUTO_INPUTS) {
            if size > sizes[start].0.max(1).saturating_mul(4)
                || size > MAX_AUTO_BYTES.saturating_sub(total)
            {
                break;
            }
            selected.push(id);
            total += size;
        }
        if selected.len() >= 2 {
            return selected;
        }
    }
    Vec::new()
}

pub fn describe(directory: &Path, manifest: &Manifest, inputs: &[u64]) -> Result<Summary> {
    let mut summary = Summary {
        inputs: inputs.to_vec(),
        ..Summary::default()
    };
    summary.full = manifest
        .segments
        .first()
        .is_some_and(|base| inputs.contains(&base.id));
    for id in inputs {
        let meta = manifest
            .segments
            .iter()
            .find(|meta| meta.id == *id)
            .context("compaction input is absent from the manifest")?;
        summary.input_bytes += bytes(directory, meta)?;
    }
    Ok(summary)
}

pub fn merge(
    directory: &Path,
    manifest: &mut Manifest,
    inputs: &[u64],
    id: u64,
) -> Result<Summary> {
    let mut summary = describe(directory, manifest, inputs)?;
    let selected: HashSet<_> = inputs.iter().copied().collect();
    let segments = manifest
        .segments
        .iter()
        .filter(|meta| selected.contains(&meta.id))
        .map(|meta| segment::load(directory, meta))
        .collect::<Result<Vec<_>>>()?;
    // The existing encoder first writes postings and key metadata, so the final
    // key count is discovered after filtering, without a second decoding pass.
    let (output, _) =
        segment::write_partitioned(directory, id, 1, |_| Merged::new(&segments, manifest))?;
    summary.output_bytes = bytes(directory, &output)?;
    summary.output = Some(id);
    for document in &mut manifest.documents {
        if document.active && document.searchable && selected.contains(&document.segment_id) {
            document.segment_id = id;
        }
    }
    let position = manifest
        .segments
        .iter()
        .position(|meta| selected.contains(&meta.id))
        .context("empty compaction plan")?;
    manifest
        .segments
        .retain(|meta| !selected.contains(&meta.id));
    manifest.segments.insert(position, output);
    prune(manifest);
    Ok(summary)
}

/// Finish explicit small-index compaction with one segment, even for legacy
/// manifests exceeding the cursor limit. Intermediate outputs stay unpublished.
pub fn merge_to_base(
    directory: &Path,
    manifest: &mut Manifest,
    mut next_id: impl FnMut() -> u64,
) -> Result<Summary> {
    let inputs: Vec<_> = manifest.segments.iter().map(|meta| meta.id).collect();
    let mut summary = describe(directory, manifest, &inputs)?;
    loop {
        let inputs: Vec<_> = manifest
            .segments
            .iter()
            .take(MAX_MERGE_INPUTS)
            .map(|meta| meta.id)
            .collect();
        let batch = merge(directory, manifest, &inputs, next_id())?;
        if manifest.segments.len() == 1 {
            summary.output = batch.output;
            summary.output_bytes = batch.output_bytes;
            return Ok(summary);
        }
    }
}

struct Merged<'a> {
    cursors: Vec<Records<'a>>,
    ids: Vec<u64>,
    manifest: &'a Manifest,
    heap: BinaryHeap<Reverse<(u32, u32, usize)>>,
    previous: Option<(u32, u32)>,
    failed: bool,
}

impl<'a> Merged<'a> {
    fn new(segments: &'a [Segment], manifest: &'a Manifest) -> Result<Self> {
        let mut merged = Self {
            cursors: segments.iter().map(Segment::records).collect(),
            ids: segments.iter().map(|segment| segment.meta.id).collect(),
            manifest,
            heap: BinaryHeap::new(),
            previous: None,
            failed: false,
        };
        for cursor in 0..segments.len() {
            merged.advance(cursor)?;
        }
        Ok(merged)
    }

    fn advance(&mut self, cursor: usize) -> Result<()> {
        for record in &mut self.cursors[cursor] {
            let (key, id) = record?;
            let doc = self
                .manifest
                .documents
                .get(id as usize)
                .context("invalid document ID in compaction input")?;
            if doc.active && doc.searchable && doc.segment_id == self.ids[cursor] {
                self.heap.push(Reverse((key, id, cursor)));
                break;
            }
        }
        Ok(())
    }
}

impl Iterator for Merged<'_> {
    type Item = Result<(u32, u32)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }
        while let Some(Reverse((key, id, cursor))) = self.heap.pop() {
            if let Err(error) = self.advance(cursor) {
                self.failed = true;
                return Some(Err(error));
            }
            if self.previous != Some((key, id)) {
                self.previous = Some((key, id));
                return Some(Ok((key, id)));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Document;

    #[test]
    fn small_snapshots_wait_for_eight_mib_even_with_many_segments() {
        let mut sizes = vec![(4 << 20, 0, 1)];
        sizes.extend((2..66).map(|id| (128 * 1024, id as usize, id)));
        let mut below = sizes.clone();
        below.last_mut().unwrap().0 -= 1;
        assert!(snapshot_plan_sizes(below).is_empty());
        let plan = snapshot_plan_sizes(sizes);
        assert_eq!(plan.len(), 65);
        assert!(plan.contains(&1));
    }

    #[test]
    fn snapshot_small_mode_uses_largest_segment_and_exact_boundary() {
        // Below 32 MiB the byte threshold applies even to a two-segment view.
        assert_eq!(
            snapshot_plan_sizes(vec![(8 << 20, 0, 2), ((32 << 20) - 1, 1, 1)]),
            vec![1, 2]
        );
        // At 32 MiB the large-index threshold is exactly 8 MiB as well.
        assert!(snapshot_plan_sizes(vec![((8 << 20) - 1, 0, 2), (32 << 20, 1, 1)]).is_empty());
        assert_eq!(
            snapshot_plan_sizes(vec![(8 << 20, 0, 2), (32 << 20, 1, 1)]),
            vec![1, 2]
        );
    }

    #[test]
    fn snapshot_large_base_waits_for_bytes_even_with_many_segments() {
        let mut sizes = vec![(1024, 0, 1), (64 << 20, 1, 2)];
        sizes.extend((3..12).map(|id| (1024, id as usize, id)));
        let plan = snapshot_plan_sizes(sizes);
        assert!(plan.is_empty());
    }

    #[test]
    fn snapshot_full_merge_requires_bytes_not_segment_count() {
        assert!(snapshot_plan_sizes(vec![(64 << 20, 0, 1), ((16 << 20) - 1, 1, 2)]).is_empty());
        assert_eq!(
            snapshot_plan_sizes(vec![(64 << 20, 0, 1), (16 << 20, 1, 2)]),
            vec![1, 2]
        );
        let many_tiny = std::iter::once((64 << 20, 0, 1))
            .chain((2..102).map(|id| (1024, id as usize, id)))
            .collect();
        assert!(snapshot_plan_sizes(many_tiny).is_empty());
        let mut sizes = vec![(64 << 20, 0, 1)];
        sizes.extend((2..11).map(|id| (2 << 20, id as usize, id)));
        let plan = snapshot_plan_sizes(sizes);
        assert_eq!(plan.len(), 10);
        assert!(plan.contains(&1));
        // A small baseline still needs at least 8 MiB of accumulated deltas.
        assert!(snapshot_plan_sizes(vec![(4 << 20, 0, 1), (2 << 20, 1, 2)]).is_empty());
        assert!(snapshot_plan_sizes(Vec::new()).is_empty());
        assert!(snapshot_plan_sizes(vec![(64 << 20, 0, 1)]).is_empty());
    }

    #[test]
    fn snapshot_byte_threshold_selects_all_inputs_for_batched_merge() {
        let mut sizes = vec![(64 << 20, 0, 1)];
        sizes.extend((2..42).map(|id| (1 << 20, id as usize, id)));
        let plan = snapshot_plan_sizes(sizes);
        assert_eq!(plan.len(), 41);
        assert!(plan.contains(&1));
    }

    fn manifest(root: &Path, segments: Vec<SegmentMeta>) -> Manifest {
        Manifest {
            registry: None,
            publication: None,
            version: 4,
            root: root.to_owned(),
            generation: 1,
            git_repository: false,
            git_head: None,
            git_tree: None,
            source_state: Vec::new(),
            documents: Vec::new(),
            segments,
        }
    }

    #[test]
    fn full_merge_uses_size_threshold_even_for_one_middle_segment() {
        let directory = tempfile::tempdir().unwrap();
        let mut segments = Vec::new();
        for id in [1, 2] {
            let lookup = format!("{id}.lookup");
            let postings = format!("{id}.postings");
            fs::File::create(directory.path().join(&lookup)).unwrap();
            fs::File::create(directory.path().join(&postings)).unwrap();
            segments.push(SegmentMeta {
                id,
                lookup: lookup.into(),
                postings: postings.into(),
                ngrams: 0,
            });
        }
        let snapshot = manifest(directory.path(), segments);
        // Sparse files exercise threshold decisions without allocating buffers.
        let base_bytes = 64 * 1024 * 1024;
        fs::OpenOptions::new()
            .write(true)
            .open(directory.path().join("1.lookup"))
            .unwrap()
            .set_len(base_bytes)
            .unwrap();
        let threshold = full_compaction_threshold(base_bytes);
        assert_eq!(full_compaction_threshold(0), MIN_FULL_COMPACT_BYTES);
        assert_eq!(threshold, base_bytes * FULL_COMPACT_PERCENT / 100);
        let middle = fs::OpenOptions::new()
            .write(true)
            .open(directory.path().join("2.lookup"))
            .unwrap();
        middle.set_len(threshold - 1).unwrap();
        assert!(
            automatic_plan(directory.path(), &snapshot)
                .unwrap()
                .is_empty()
        );
        middle.set_len(threshold).unwrap();
        assert_eq!(
            automatic_plan(directory.path(), &snapshot).unwrap(),
            vec![1, 2]
        );
        // The mode boundary is based on B alone, even when B + M crosses it.
        let base = fs::OpenOptions::new()
            .write(true)
            .open(directory.path().join("1.lookup"))
            .unwrap();
        middle.set_len(1).unwrap();
        base.set_len(MIN_GENERATIONAL_BYTES - 1).unwrap();
        assert!(
            automatic_plan(directory.path(), &snapshot)
                .unwrap()
                .is_empty()
        );
        assert!(small_base(directory.path(), &snapshot).unwrap());
        assert!(!small_rebuild_due(directory.path(), &snapshot).unwrap());
        middle.set_len(SMALL_REBUILD_BYTES - 1).unwrap();
        assert!(!small_rebuild_due(directory.path(), &snapshot).unwrap());
        middle.set_len(SMALL_REBUILD_BYTES).unwrap();
        assert!(small_rebuild_due(directory.path(), &snapshot).unwrap());
        middle.set_len(1).unwrap();
        base.set_len(MIN_GENERATIONAL_BYTES).unwrap();
        assert!(
            automatic_plan(directory.path(), &snapshot)
                .unwrap()
                .is_empty()
        );
        assert!(!small_base(directory.path(), &snapshot).unwrap());
        middle
            .set_len(full_compaction_threshold(MIN_GENERATIONAL_BYTES))
            .unwrap();
        assert_eq!(
            automatic_plan(directory.path(), &snapshot).unwrap(),
            vec![1, 2]
        );
    }

    #[test]
    fn base_merge_preserves_newer_overlays_and_filters_deleted_and_binary_versions() {
        let directory = tempfile::tempdir().unwrap();
        let inputs = [
            vec![(10, 0), (11, 1), (12, 2)],
            vec![(13, 1), (14, 3)],
            vec![(20, 1), (30, 4)],
        ];
        let segments = inputs
            .into_iter()
            .enumerate()
            .map(|(i, records)| {
                segment::write_sorted(
                    directory.path(),
                    i as u64 + 1,
                    records.len() as u64,
                    records.into_iter().map(Ok),
                )
                .unwrap()
            })
            .collect();
        let mut snapshot = manifest(directory.path(), segments);
        snapshot.documents = [
            (1, true, true),
            (3, true, true),
            (1, false, true),
            (2, true, false),
            (3, true, true),
        ]
        .into_iter()
        .enumerate()
        .map(|(id, (segment_id, active, searchable))| Document {
            path: format!("missing-source-{id}").into(),
            len: 0,
            modified_nanos: 0,
            active,
            searchable,
            segment_id,
        })
        .collect();
        // No source files exist. Newest M (3) remains outside the first merge.
        merge(directory.path(), &mut snapshot, &[1, 2], 4).unwrap();
        assert_eq!(snapshot.documents[1].segment_id, 3);
        merge(directory.path(), &mut snapshot, &[4, 3], 5).unwrap();
        assert_eq!(snapshot.segments.len(), 1);
        let output = segment::load(directory.path(), &snapshot.segments[0]).unwrap();
        assert_eq!(
            output.records().collect::<Result<Vec<_>>>().unwrap(),
            vec![(10, 0), (20, 1), (30, 4)]
        );
        assert!(!snapshot.documents[2].active);
        assert!(!snapshot.documents[3].searchable);
    }

    #[test]
    fn automatic_work_has_byte_and_fan_in_limits_and_preserves_large_inputs() {
        let mut sizes = vec![(MAX_AUTO_BYTES * 2, 0, 1)];
        sizes.extend((2..12).map(|id| (1024, id as usize, id)));
        assert_eq!(plan_sizes(sizes), vec![2, 3, 4, 5]);
        assert!(plan_sizes(vec![(MAX_AUTO_BYTES, 0, 1), (MAX_AUTO_BYTES, 1, 2)]).is_empty());
        assert_eq!(
            plan_sizes(vec![(1, 0, 1), (1024, 1, 2), (1024, 2, 3)]),
            vec![2, 3]
        );
    }
}
