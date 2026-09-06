use std::{collections::BTreeSet, fs, path::Path};

use anyhow::{Context, Result};
use rayon::prelude::*;
use regex::bytes::{Regex, RegexBuilder};

use crate::{index, query};

pub struct Options {
    pub ignore_case: bool,
    pub fixed_strings: bool,
    pub files_with_matches: bool,
    pub count: bool,
    pub max_count: Option<usize>,
    pub no_refresh: bool,
}

struct FileResult {
    id: u32,
    output: Vec<String>,
    matches: usize,
}

pub fn run(
    path: &Path,
    requested_index_dir: Option<&Path>,
    pattern: &str,
    options: &Options,
) -> Result<bool> {
    let regex_pattern = if options.fixed_strings {
        regex::escape(pattern)
    } else {
        pattern.to_owned()
    };
    let regex = RegexBuilder::new(&regex_pattern)
        .case_insensitive(options.ignore_case)
        .multi_line(true)
        .build()
        .with_context(|| format!("invalid regular expression {pattern:?}"))?;

    let mut disk_index = match index::load(path, requested_index_dir) {
        Ok(index) => index,
        Err(error) if !options.no_refresh => {
            eprintln!("coderg: building index ({error})");
            index::build(path, requested_index_dir)?;
            index::load(path, requested_index_dir)?
        }
        Err(error) => return Err(error),
    };
    if !options.no_refresh {
        match index::refresh(&disk_index, requested_index_dir)? {
            index::RefreshOutcome::Unchanged => {}
            index::RefreshOutcome::CommitAdvanced => {
                eprintln!("coderg: advanced index to the current Git tree");
                disk_index = index::load(path, requested_index_dir)?;
            }
            index::RefreshOutcome::Incremental { changed } => {
                eprintln!("coderg: incrementally indexed {changed} changed files");
                disk_index = index::load(path, requested_index_dir)?;
            }
            index::RefreshOutcome::SwitchedToCachedTree => {
                eprintln!("coderg: switched to cached Git tree index");
                disk_index = index::load(path, requested_index_dir)?;
            }
            index::RefreshOutcome::Rebuilt => {
                eprintln!("coderg: rebuilt index after a large change");
                disk_index = index::load(path, requested_index_dir)?;
            }
        }
    }

    let strategies = if options.fixed_strings {
        query::fixed_strategy(pattern.as_bytes(), options.ignore_case)
    } else {
        query::literal_strategies(pattern, options.ignore_case)?
    };
    let candidates = choose_candidates(&disk_index, &strategies)?;
    let root = &disk_index.manifest.root;
    let results: Vec<Option<FileResult>> = candidates
        .par_iter()
        .map(|&id| -> Result<_> {
            let document = disk_index
                .manifest
                .documents
                .get(id as usize)
                .with_context(|| "index contains an invalid document ID; rebuild it")?;
            let full_path = root.join(&document.path);
            let bytes = fs::read(&full_path)
                .with_context(|| format!("cannot read {}", full_path.display()))?;
            let (output, matches) = match_file(&regex, &bytes, &document.path, options);
            Ok((matches > 0).then_some(FileResult {
                id,
                output,
                matches,
            }))
        })
        .collect::<Result<_>>()?;
    let mut results: Vec<FileResult> = results.into_iter().flatten().collect();
    results.sort_by_key(|result| result.id);

    for result in &results {
        let path = &disk_index.manifest.documents[result.id as usize].path;
        if options.files_with_matches {
            println!("{}", path.display());
        } else if options.count {
            println!("{}:{}", path.display(), result.matches);
        } else {
            for line in &result.output {
                println!("{line}");
            }
        }
    }
    Ok(!results.is_empty())
}

fn choose_candidates(
    index: &index::DiskIndex,
    strategies: &[Vec<crate::ngram::GramHash>],
) -> Result<Vec<u32>> {
    let all = || index.all_document_ids();
    if strategies.is_empty() {
        return Ok(all());
    }
    let mut best: Option<BTreeSet<u32>> = None;
    for alternatives in strategies {
        if alternatives.is_empty() {
            continue;
        }
        let mut union = BTreeSet::new();
        for &hash in alternatives {
            union.extend(index.postings(hash)?);
        }
        if best
            .as_ref()
            .is_none_or(|current| union.len() < current.len())
        {
            best = Some(union);
        }
    }
    Ok(best.map_or_else(all, |set| set.into_iter().collect()))
}

fn match_file(regex: &Regex, bytes: &[u8], path: &Path, options: &Options) -> (Vec<String>, usize) {
    let mut lines = Vec::new();
    let mut line_starts = vec![0_usize];
    line_starts.extend(
        bytes
            .iter()
            .enumerate()
            .filter_map(|(position, &byte)| (byte == b'\n').then_some(position + 1)),
    );
    let mut seen_lines = BTreeSet::new();
    let limit = options.max_count.unwrap_or(usize::MAX);
    for found in regex.find_iter(bytes) {
        if seen_lines.len() >= limit {
            break;
        }
        let offset = found.start();
        let line_index = line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        seen_lines.insert(line_index);
    }
    let matches = seen_lines.len();
    if options.files_with_matches || options.count {
        return (lines, matches);
    }
    for line_index in seen_lines {
        let start = line_starts[line_index];
        let end = bytes[start..]
            .iter()
            .position(|&byte| byte == b'\n')
            .map_or(bytes.len(), |offset| start + offset);
        let text = String::from_utf8_lossy(&bytes[start..end]);
        lines.push(format!("{}:{}:{}", path.display(), line_index + 1, text));
    }
    (lines, matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> Options {
        Options {
            ignore_case: false,
            fixed_strings: false,
            files_with_matches: false,
            count: false,
            max_count: None,
            no_refresh: false,
        }
    }

    #[test]
    fn reports_each_matching_line_once() {
        let regex = RegexBuilder::new("foo").multi_line(true).build().unwrap();
        let (lines, count) = match_file(&regex, b"foo foo\nbar\nfoo\n", Path::new("x"), &options());
        assert_eq!(count, 2);
        assert_eq!(lines, ["x:1:foo foo", "x:3:foo"]);
    }
}
