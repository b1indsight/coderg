//! Bounded posting buffers and external sorting for index construction.
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Take, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, bail};
use rayon::prelude::*;
use tempfile::TempDir;

use crate::{ngram, segment};

pub type Record = (ngram::GramHash, u32);
const MIB: usize = 1024 * 1024;
pub const CHUNK_BYTES: usize = 32 * 1024;
// Every longer gram links a bigram to its nearest >= neighbor on one side.
// At most two such grams per bigram, plus trigrams: < 3 hashes per byte.
// Reserve space for u64 dedup, compact output, queued chunks, and I/O.
const WORKER_BYTES: usize = 8 * MIB;
const IO_BYTES: usize = 64 * 1024;
const MERGE_FAN_IN: usize = 32;
const MERGE_PARTITIONS: usize = 4;
const MERGE_SAMPLES: usize = 4096;
const FIXED_BYTES: usize = 4 * MIB;

#[derive(Clone, Copy, Debug)]
pub struct MemoryBudget(usize);

impl FromStr for MemoryBudget {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let mib: usize = value
            .parse()
            .map_err(|_| "expected an integer MiB budget")?;
        if mib < 16 {
            return Err("build memory budget must be at least 16 MiB".into());
        }
        mib.checked_mul(MIB)
            .filter(|&bytes| bytes <= isize::MAX as usize)
            .map(Self)
            .ok_or_else(|| "build memory budget is too large".into())
    }
}

impl MemoryBudget {
    pub fn workers(self, files: usize) -> usize {
        rayon::current_num_threads()
            .min(files.max(1))
            .min((self.0 / 4 / WORKER_BYTES).max(1))
    }

    pub fn record_bytes(self, workers: usize) -> usize {
        self.0 - workers * WORKER_BYTES - FIXED_BYTES
    }
}

struct Run {
    path: PathBuf,
    ngrams: u64,
    bytes: u64,
}

pub struct PostingsBuilder {
    index_dir: PathBuf,
    records: Vec<Record>,
    limit: usize,
    temporary: Option<TempDir>,
    runs: Vec<Run>,
    next_run: usize,
    spilled_runs: usize,
    temporary_bytes: u64,
    live_temporary_bytes: u64,
    peak_temporary_bytes: u64,
}

impl PostingsBuilder {
    pub fn new(index_dir: &Path, record_bytes: usize) -> Self {
        Self {
            index_dir: index_dir.to_owned(),
            records: Vec::new(),
            limit: (record_bytes / size_of::<Record>()).max(1),
            temporary: None,
            runs: Vec::new(),
            next_run: 0,
            spilled_runs: 0,
            temporary_bytes: 0,
            live_temporary_bytes: 0,
            peak_temporary_bytes: 0,
        }
    }

    pub fn extend(&mut self, id: u32, hashes: &[ngram::GramHash]) -> Result<()> {
        if self.records.capacity() == 0 && !hashes.is_empty() {
            // Allocate once: geometric Vec growth can temporarily retain both
            // the old and new allocation, exceeding the record budget.
            self.records = Vec::with_capacity(self.limit);
        }
        let mut hashes = hashes;
        while !hashes.is_empty() {
            if self.records.len() == self.limit {
                self.spill()?;
            }
            let count = hashes.len().min(self.limit - self.records.len());
            self.records
                .extend(hashes[..count].iter().map(|&hash| (hash, id)));
            hashes = &hashes[count..];
        }
        Ok(())
    }

    fn run_path(&mut self) -> Result<PathBuf> {
        if self.temporary.is_none() {
            self.temporary = Some(
                tempfile::Builder::new()
                    .prefix(".build-")
                    .tempdir_in(&self.index_dir)
                    .context("cannot create temporary index sort directory")?,
            );
        }
        let path = self
            .temporary
            .as_ref()
            .unwrap()
            .path()
            .join(format!("{}.run", self.next_run));
        self.next_run += 1;
        Ok(path)
    }

    fn spill(&mut self) -> Result<()> {
        self.records.par_sort_unstable();
        self.records.dedup();
        let path = self.run_path()?;
        let mut writer = RunWriter::new(&path)?;
        for &record in &self.records {
            writer.push(record)?;
        }
        let (run, bytes) = writer.finish(path)?;
        self.record_temporary_write(bytes);
        self.spilled_runs += 1;
        self.runs.push(run);
        self.records.clear();
        Ok(())
    }

    fn record_temporary_write(&mut self, bytes: u64) {
        self.temporary_bytes += bytes;
        self.live_temporary_bytes += bytes;
        self.peak_temporary_bytes = self.peak_temporary_bytes.max(self.live_temporary_bytes);
    }

    pub fn write(self, id: u64) -> Result<segment::SegmentMeta> {
        self.write_with_durability(id, true)
    }

    pub fn write_with_durability(mut self, id: u64, durable: bool) -> Result<segment::SegmentMeta> {
        if self.runs.is_empty() {
            self.records.par_sort_unstable();
            self.records.dedup();
            let ngrams = self.records.chunk_by(|a, b| a.0 == b.0).count() as u64;
            return segment::write_sorted_with_durability(
                &self.index_dir,
                id,
                ngrams,
                self.records.iter().copied().map(Ok),
                durable,
            );
        }
        self.prepare_runs()?;
        let meta = if self.runs.len() == 1 {
            let run = &self.runs[0];
            segment::write_sorted_with_durability(
                &self.index_dir,
                id,
                run.ngrams,
                RunReader::new(&run.path)?,
                durable,
            )?
        } else {
            let partitions = partition_runs(&self.runs)?;
            let (meta, scratch_bytes) = segment::write_partitioned_with_durability(
                &self.index_dir,
                id,
                partitions.len(),
                |part| {
                    MergedRuns::new(
                        partitions[part]
                            .iter()
                            .map(|range| RunReader::range(&range.run.path, range.start, range.end))
                            .collect::<Result<_>>()?,
                    )
                },
                durable,
            )?;
            self.record_temporary_write(scratch_bytes);
            meta
        };
        self.report_spills();
        Ok(meta)
    }

    fn prepare_runs(&mut self) -> Result<()> {
        if !self.records.is_empty() {
            self.spill()?;
        }
        // No extraction workers remain. Release the large sort buffer before
        // opening bounded merge readers or encoding the final segment.
        self.records = Vec::new();
        while self.runs.len() > MERGE_FAN_IN {
            let mut remaining = std::mem::take(&mut self.runs).into_iter();
            loop {
                let inputs: Vec<_> = remaining.by_ref().take(MERGE_FAN_IN).collect();
                if inputs.is_empty() {
                    break;
                }
                if inputs.len() == 1 {
                    self.runs.extend(inputs);
                    continue;
                }
                let output = self.run_path()?;
                let (run, bytes) = merge_runs(&inputs, output)?;
                self.record_temporary_write(bytes);
                for input in inputs {
                    fs::remove_file(input.path)?;
                    self.live_temporary_bytes -= input.bytes;
                }
                self.runs.push(run);
            }
        }
        Ok(())
    }

    fn report_spills(&self) {
        eprintln!(
            "coderg: spilled {} build runs ({} temporary bytes written, {} peak temporary bytes)",
            self.spilled_runs, self.temporary_bytes, self.peak_temporary_bytes,
        );
    }
}

struct RunWriter {
    file: BufWriter<File>,
    previous: Option<Record>,
    ngrams: u64,
    bytes: u64,
}

impl RunWriter {
    fn new(path: &Path) -> Result<Self> {
        Ok(Self {
            file: BufWriter::with_capacity(IO_BYTES, File::create(path)?),
            previous: None,
            ngrams: 0,
            bytes: 0,
        })
    }

    fn push(&mut self, record: Record) -> Result<()> {
        if self.previous == Some(record) {
            return Ok(());
        }
        if self.previous.is_none_or(|previous| previous.0 != record.0) {
            self.ngrams += 1;
        }
        let mut bytes = [0; 8];
        bytes[..4].copy_from_slice(&record.0.to_le_bytes());
        bytes[4..].copy_from_slice(&record.1.to_le_bytes());
        self.file.write_all(&bytes)?;
        self.bytes += 8;
        self.previous = Some(record);
        Ok(())
    }

    fn finish(mut self, path: PathBuf) -> Result<(Run, u64)> {
        self.file.flush()?;
        Ok((
            Run {
                path,
                ngrams: self.ngrams,
                bytes: self.bytes,
            },
            self.bytes,
        ))
    }
}

struct RunReader(BufReader<Take<File>>);

impl RunReader {
    fn new(path: &Path) -> Result<Self> {
        Self::range(path, 0, fs::metadata(path)?.len())
    }

    fn range(path: &Path, start: u64, end: u64) -> Result<Self> {
        let mut file = File::open(path)?;
        let len = file.metadata()?.len();
        if !len.is_multiple_of(8) {
            bail!("truncated temporary posting run: {}", path.display());
        }
        if start > end || end > len || !start.is_multiple_of(8) || !end.is_multiple_of(8) {
            bail!("invalid temporary posting range: {}", path.display());
        }
        file.seek(SeekFrom::Start(start))?;
        Ok(Self(BufReader::with_capacity(
            IO_BYTES,
            file.take(end - start),
        )))
    }
}

impl Iterator for RunReader {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.fill_buf() {
                Ok([]) => return None,
                Ok(bytes) if bytes.len() >= 8 => {
                    let record = (
                        u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                        u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
                    );
                    self.0.consume(8);
                    return Some(Ok(record));
                }
                Ok(_) => break,
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => return Some(Err(error.into())),
            }
        }
        // A short buffered read can split a record. read_exact rejects an
        // incomplete final record instead of treating it as a clean EOF.
        let mut bytes = [0; 8];
        Some(
            self.0
                .read_exact(&mut bytes)
                .map(|()| {
                    (
                        u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                        u32::from_le_bytes(bytes[4..].try_into().unwrap()),
                    )
                })
                .map_err(Into::into),
        )
    }
}

fn merge_runs(inputs: &[Run], output: PathBuf) -> Result<(Run, u64)> {
    let readers = inputs
        .iter()
        .map(|run| RunReader::new(&run.path))
        .collect::<Result<_>>()?;
    let mut writer = RunWriter::new(&output)?;
    for record in MergedRuns::new(readers)? {
        writer.push(record?)?;
    }
    writer.finish(output)
}

struct MergedRuns {
    readers: Vec<RunReader>,
    heap: BinaryHeap<Reverse<(ngram::GramHash, u32, usize)>>,
    previous: Option<Record>,
}

impl MergedRuns {
    fn new(mut readers: Vec<RunReader>) -> Result<Self> {
        let mut heap = BinaryHeap::new();
        for (index, reader) in readers.iter_mut().enumerate() {
            if let Some((hash, id)) = reader.next().transpose()? {
                heap.push(Reverse((hash, id, index)));
            }
        }
        Ok(Self {
            readers,
            heap,
            previous: None,
        })
    }
}

impl Iterator for MergedRuns {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Reverse((hash, id, index))) = self.heap.pop() {
            match self.readers[index].next() {
                Some(Ok((key, doc))) => self.heap.push(Reverse((key, doc, index))),
                Some(Err(error)) => {
                    self.heap.clear();
                    return Some(Err(error));
                }
                None => {}
            }
            let record = (hash, id);
            if self.previous != Some(record) {
                self.previous = Some(record);
                return Some(Ok(record));
            }
        }
        None
    }
}

struct RunRange<'a> {
    run: &'a Run,
    start: u64,
    end: u64,
}

/// Sample by record count, then cut every run at the same gram boundaries.
/// Keeping an entire gram in one partition makes both dedup and encoding local.
fn partition_runs(runs: &[Run]) -> Result<Vec<Vec<RunRange<'_>>>> {
    let total: u64 = runs.iter().map(|run| run.bytes / 8).sum();
    let count = total.min(MERGE_SAMPLES as u64);
    let mut samples = Vec::with_capacity(count as usize);
    let mut sample = 0_u64;
    let mut base = 0_u64;
    for run in runs {
        let mut file = File::open(&run.path)?;
        let end = base + run.bytes / 8;
        while sample < count {
            let rank =
                ((u128::from(sample) * 2 + 1) * u128::from(total) / (u128::from(count) * 2)) as u64;
            if rank >= end {
                break;
            }
            file.seek(SeekFrom::Start((rank - base) * 8))?;
            let mut key = [0; 4];
            file.read_exact(&mut key)?;
            samples.push(u32::from_le_bytes(key));
            sample += 1;
        }
        base = end;
    }
    samples.sort_unstable();
    let mut boundaries = vec![0_u64];
    if !samples.is_empty() {
        for part in 1..MERGE_PARTITIONS {
            let key = u64::from(samples[samples.len() * part / MERGE_PARTITIONS]);
            if key > *boundaries.last().unwrap() {
                boundaries.push(key);
            }
        }
    }
    boundaries.push(1_u64 << 32);
    let mut partitions: Vec<Vec<RunRange<'_>>> =
        (1..boundaries.len()).map(|_| Vec::new()).collect();
    for run in runs {
        let mut file = File::open(&run.path)?;
        let mut start = 0;
        for (part, &upper) in boundaries[1..].iter().enumerate() {
            let end = lower_bound(&mut file, run.bytes / 8, upper)? * 8;
            if start < end {
                partitions[part].push(RunRange { run, start, end });
            }
            start = end;
        }
    }
    partitions.retain(|part| !part.is_empty());
    Ok(partitions)
}

fn lower_bound(file: &mut File, records: u64, key: u64) -> Result<u64> {
    if key == 1_u64 << 32 {
        return Ok(records);
    }
    let (mut low, mut high) = (0, records);
    while low < high {
        let mid = low + (high - low) / 2;
        file.seek(SeekFrom::Start(mid * 8))?;
        let mut bytes = [0; 4];
        file.read_exact(&mut bytes)?;
        if u64::from(u32::from_le_bytes(bytes)) < key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    Ok(low)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_reserves_space_for_producers_and_io() {
        for mib in [16, 32, 64, 256, 1024] {
            let budget: MemoryBudget = mib.to_string().parse().unwrap();
            let workers = budget.workers(100_000);
            assert!(workers >= 1);
            assert!(budget.record_bytes(workers) >= 4 * MIB);
            assert_eq!(
                budget.record_bytes(workers) + workers * WORKER_BYTES + FIXED_BYTES,
                mib * MIB
            );
        }
        for invalid in ["0", "15", "-1", "1.5", "18446744073709551615"] {
            assert!(invalid.parse::<MemoryBudget>().is_err());
        }
    }

    #[test]
    fn many_merge_runs_match_in_memory_bytes_and_clean_up() {
        let disk = tempfile::tempdir().unwrap();
        let memory = tempfile::tempdir().unwrap();
        // More than 32 runs exercises multiple merge passes, repeated keys,
        // duplicate IDs across runs, and a partial final lookup block.
        let mut external = PostingsBuilder::new(disk.path(), 3 * size_of::<Record>());
        let mut in_memory = PostingsBuilder::new(memory.path(), MIB);
        for id in (0..5).rev().chain([u32::MAX, 0]) {
            let hashes: Vec<_> = (0..259).rev().chain([u32::MAX, 0, 128]).collect();
            external.extend(id, &hashes).unwrap();
            in_memory.extend(id, &hashes).unwrap();
            assert!(external.records.capacity() <= external.limit);
        }
        assert!(external.runs.len() > MERGE_FAN_IN);
        let actual = external.write(1).unwrap();
        let expected = in_memory.write(1).unwrap();
        assert_eq!(actual.ngrams, 260);
        for (actual_path, expected_path) in [
            (actual.lookup, expected.lookup),
            (actual.postings, expected.postings),
        ] {
            assert_eq!(
                fs::read(disk.path().join(actual_path)).unwrap(),
                fs::read(memory.path().join(expected_path)).unwrap()
            );
        }
        assert_eq!(fs::read_dir(disk.path()).unwrap().count(), 1);
        assert_eq!(
            fs::read_dir(disk.path().join("segments")).unwrap().count(),
            2
        );
    }

    #[test]
    fn cached_writes_match_durable_bytes_with_and_without_spills() {
        let directory = tempfile::tempdir().unwrap();
        let mut outputs = Vec::new();
        for (id, (budget, durable)) in [(MIB, true), (MIB, false), (24, false)]
            .into_iter()
            .enumerate()
        {
            let mut builder = PostingsBuilder::new(directory.path(), budget);
            for doc in (0..5).rev().chain([0]) {
                builder
                    .extend(doc, &(0..259).rev().collect::<Vec<_>>())
                    .unwrap();
            }
            let meta = builder.write_with_durability(id as u64, durable).unwrap();
            outputs.push((
                fs::read(directory.path().join(meta.lookup)).unwrap(),
                fs::read(directory.path().join(meta.postings)).unwrap(),
            ));
        }
        assert_eq!(outputs[0], outputs[1]);
        assert_eq!(outputs[0], outputs[2]);
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
        assert_eq!(
            fs::read_dir(directory.path().join("segments"))
                .unwrap()
                .count(),
            6
        );
    }

    #[test]
    fn failed_segment_write_removes_temporary_runs() {
        let directory = tempfile::tempdir().unwrap();
        let mut builder = PostingsBuilder::new(directory.path(), 8);
        builder.extend(0, &[1, 2, 3]).unwrap();
        let temporary = builder.temporary.as_ref().unwrap().path().to_owned();
        assert!(temporary.is_dir());
        fs::write(
            directory.path().join("segments"),
            "cannot create a directory here",
        )
        .unwrap();
        assert!(builder.write(1).is_err());
        assert!(!temporary.exists());
    }

    #[test]
    fn partitioned_merge_matches_global_dedup_without_splitting_grams() {
        for one_key in [false, true] {
            let directory = tempfile::tempdir().unwrap();
            let mut runs = Vec::new();
            let mut expected = Vec::new();
            for run in 0..3 {
                let path = directory.path().join(format!("{run}.run"));
                let mut records = Vec::new();
                for key in 0..2048 {
                    let hash = if one_key { 7 } else { key };
                    records.extend([(hash, key), (hash, key + run)]);
                }
                if !one_key {
                    records.push((u32::MAX, u32::MAX));
                }
                records.sort_unstable();
                let mut writer = RunWriter::new(&path).unwrap();
                for &record in &records {
                    writer.push(record).unwrap();
                }
                runs.push(writer.finish(path).unwrap().0);
                expected.extend(records);
            }
            expected.sort_unstable();
            expected.dedup();
            let partitions = partition_runs(&runs).unwrap();
            assert_eq!(partitions.len(), if one_key { 1 } else { 4 });
            let mut actual: Vec<Record> = Vec::new();
            for part in partitions {
                let readers = part
                    .iter()
                    .map(|range| RunReader::range(&range.run.path, range.start, range.end).unwrap())
                    .collect();
                let records = MergedRuns::new(readers)
                    .unwrap()
                    .collect::<Result<Vec<_>>>()
                    .unwrap();
                if let (Some(last), Some(first)) = (actual.last(), records.first()) {
                    assert!(last.0 < first.0, "a gram crossed a partition boundary");
                }
                actual.extend(records);
            }
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn truncated_run_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("broken.run");
        fs::write(&path, [0; 7]).unwrap();
        assert!(RunReader::new(&path).is_err());
        fs::write(&path, [0; 16]).unwrap();
        assert!(RunReader::range(&path, 1, 8).is_err());
        assert!(RunReader::range(&path, 0, 24).is_err());
        assert!(RunReader::range(&path, 8, 0).is_err());
    }
}
