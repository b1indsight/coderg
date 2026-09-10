use std::{fs::File, io::{BufReader, Read}, path::PathBuf, time::Instant};
use anyhow::{Context, Result, ensure};
use ignore::WalkBuilder;
use rayon::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct Counts {
    files: u64,
    binary_files: u64,
    source_bytes: u64,
    all: [u64; 26],
    initial: [u64; 26],
    pairs: Vec<u64>,
}

impl Default for Counts {
    fn default() -> Self {
        Self { files: 0, binary_files: 0, source_bytes: 0, all: [0; 26], initial: [0; 26], pairs: vec![0; 26 * 26] }
    }
}

impl Counts {
    fn add(&mut self, bytes: &[u8], previous: &mut Option<usize>) {
        self.source_bytes += bytes.len() as u64;
        for &byte in bytes {
            let lower = byte.to_ascii_lowercase();
            if lower.is_ascii_lowercase() {
                let i = usize::from(lower - b'a');
                self.all[i] += 1;
                if let Some(left) = *previous {
                    self.pairs[left * 26 + i] += 1;
                } else {
                    self.initial[i] += 1;
                }
                *previous = Some(i);
            } else {
                *previous = None;
            }
        }
    }
    fn merge(mut self, other: Self) -> Self {
        self.files += other.files;
        self.binary_files += other.binary_files;
        self.source_bytes += other.source_bytes;
        for i in 0..26 {
            self.all[i] += other.all[i];
            self.initial[i] += other.initial[i];
        }
        for (a, b) in self.pairs.iter_mut().zip(other.pairs) { *a += b; }
        self
    }
}

fn count_file(path: &PathBuf) -> Result<Counts> {
    let mut reader = BufReader::with_capacity(65536, File::open(path)?);
    let before = reader.get_ref().metadata()?;
    let mut prefix = Vec::with_capacity(8192);
    reader.by_ref().take(8192).read_to_end(&mut prefix)?;
    if prefix.contains(&0) {
        return Ok(Counts { binary_files: 1, ..Counts::default() });
    }
    let mut counts = Counts { files: 1, ..Counts::default() };
    let mut in_word = None;
    counts.add(&prefix, &mut in_word);
    let mut buffer = vec![0u8; 65536];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 { break; }
        counts.add(&buffer[..n], &mut in_word);
    }
    let after = reader.get_ref().metadata()?;
    ensure!(before.len() == after.len() && before.modified()? == after.modified()?
        && counts.source_bytes == after.len(), "file changed during count: {}", path.display());
    Ok(counts)
}

fn main() -> Result<()> {
    let root = PathBuf::from(std::env::args_os().nth(1).context("source root required")?).canonicalize()?;
    let started = Instant::now();
    let mut walker = WalkBuilder::new(&root);
    walker.hidden(false).follow_links(false).filter_entry(|e| e.file_name() != ".git" && e.file_name() != ".coderg-index");
    let files: Vec<PathBuf> = walker.build().filter_map(|entry| match entry {
        Ok(e) if e.file_type().is_some_and(|t| t.is_file()) => Some(Ok(e.into_path())),
        Ok(_) => None,
        Err(e) => Some(Err(e)),
    }).collect::<Result<_, _>>()?;
    let counts = files.par_iter().map(|p| count_file(p).with_context(|| p.display().to_string()))
        .try_reduce(Counts::default, |a, b| Ok(a.merge(b)))?;
    let result = serde_json::json!({
        "root":root, "snapshot":"398630472335c10b9ca610a4d1b7888a040f702a",
        "walked_files":files.len(), "elapsed_seconds":started.elapsed().as_secs_f64(),
        "counts":counts, "all_total":counts.all.iter().sum::<u64>(),
        "initial_total":counts.initial.iter().sum::<u64>(),
        "pair_total":counts.pairs.iter().sum::<u64>(),
        "rules":"Directed overlapping ASCII letter pairs, row=first, column=second, 26x26 row-major; ASCII [A-Za-z]+ runs; lowercase for counting; no camelCase split; file/chunk boundaries handled; all searchable text including docs and third_party; NUL in first 8192 bytes excluded"
    });
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initial_letters_survive_chunk_boundaries_without_splitting_camel_case() {
        let mut counts = Counts::default();
        let mut state = None;
        for chunk in [b"_fooB".as_slice(), b"ar 9ZI", b"P\xc3\x80x\na", b"b"] {
            counts.add(chunk, &mut state);
        }
        let expected = [(b'f',1), (b'z',1), (b'x',1), (b'a',1)];
        assert_eq!(counts.initial.iter().sum::<u64>(), 4);
        for (letter,n) in expected { assert_eq!(counts.initial[usize::from(letter-b'a')],n); }
        assert_eq!(counts.all.iter().sum::<u64>(), 12);
        assert_eq!(counts.all[usize::from(b'b'-b'a')],2);
        assert_eq!(counts.all[usize::from(b'o'-b'a')],2);
        assert_eq!(counts.pairs.iter().sum::<u64>(), 8);
        assert_eq!(counts.pairs[usize::from(b'b'-b'a') * 26], 1);
        assert_eq!(counts.pairs[25 * 26 + 8], 1);
        assert_eq!(counts.pairs[15 * 26 + 23], 0);
    }
}
