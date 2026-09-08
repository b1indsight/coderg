use std::{collections::BTreeSet, fs, path::Path};

use anyhow::{Context, Result};
use rayon::prelude::*;
use regex::bytes::{Regex, RegexBuilder};

use crate::{index, ngram, query};

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

    let strategies = if options.fixed_strings && !options.ignore_case {
        query::fixed_strategy(pattern.as_bytes())
    } else {
        query::literal_strategies(&regex_pattern, options.ignore_case)?
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
    strategies: &[Vec<ngram::GramHash>],
) -> Result<Vec<u32>> {
    Ok(
        match intersect_postings(strategies, |hash| index.postings(hash))? {
            Some(candidates) => candidates.into_iter().collect(),
            None => index.all_document_ids(),
        },
    )
}

fn intersect_postings(
    strategies: &[Vec<ngram::GramHash>],
    mut load_postings: impl FnMut(ngram::GramHash) -> Result<Vec<u32>>,
) -> Result<Option<BTreeSet<u32>>> {
    let mut cache = ngram::GramHashMap::default();
    let mut candidates: Option<BTreeSet<u32>> = None;
    for alternatives in strategies {
        if alternatives.is_empty() {
            continue;
        }
        let new_hashes: BTreeSet<_> = alternatives
            .iter()
            .copied()
            .filter(|hash| !cache.contains_key(hash))
            .collect();
        if cache.len() + new_hashes.len() > query::MAX_INDEX_LOOKUPS {
            // A partial union could exclude an unvisited case variant or regex
            // branch. Keep earlier complete filters and skip this whole one.
            continue;
        }
        for hash in new_hashes {
            cache.insert(hash, load_postings(hash)?);
        }
        let union: BTreeSet<_> = alternatives
            .iter()
            .flat_map(|hash| cache[hash].iter().copied())
            .collect();
        if let Some(current) = &mut candidates {
            current.retain(|id| union.contains(id));
        } else {
            candidates = Some(union);
        }
        if candidates.as_ref().is_some_and(BTreeSet::is_empty) {
            break;
        }
    }
    Ok(candidates)
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

    #[test]
    fn unions_variants_intersects_fragments_and_reuses_lookups() {
        let mut looked_up = BTreeSet::new();
        let candidates = intersect_postings(&[vec![1, 2], vec![2, 3]], |hash| {
            assert!(looked_up.insert(hash), "each key should be read only once");
            Ok(match hash {
                1 => vec![10, 20],
                2 => vec![30],
                3 => vec![20],
                _ => unreachable!(),
            })
        })
        .unwrap()
        .unwrap();
        assert_eq!(candidates, BTreeSet::from([20, 30]));
        assert_eq!(looked_up.len(), 3);
    }

    #[test]
    fn budget_never_applies_an_incomplete_alternative_union() {
        let limit = query::MAX_INDEX_LOOKUPS as u32;
        let mut strategies: Vec<Vec<_>> = (0..limit)
            .collect::<Vec<_>>()
            .chunks(query::MAX_LITERAL_VARIANTS)
            .map(<[u32]>::to_vec)
            .collect();
        // The cached alternative only contains document 9. Document 7 is in
        // the unvisited alternative, so applying a partial union would lose it.
        strategies.push(vec![limit - 1, limit]);
        let mut looked_up = BTreeSet::new();
        let candidates = intersect_postings(&strategies, |hash| {
            assert!(looked_up.insert(hash));
            Ok(if hash.is_multiple_of(query::MAX_LITERAL_VARIANTS as u32) {
                vec![7, 9]
            } else {
                vec![9]
            })
        })
        .unwrap()
        .unwrap();
        assert_eq!(candidates, BTreeSet::from([7, 9]));
        assert_eq!(looked_up.len(), query::MAX_INDEX_LOOKUPS);
        assert!(!looked_up.contains(&limit));
    }

    #[test]
    fn no_complete_filter_means_full_scan() {
        let alternatives = (0..=query::MAX_INDEX_LOOKUPS as u32).collect();
        let candidates = intersect_postings(&[alternatives], |_| {
            panic!("an over-budget condition must be skipped before any reads")
        })
        .unwrap();
        assert!(candidates.is_none());
    }

    #[test]
    fn covering_filters_files_that_only_share_the_longest_gram() {
        let literal = b"abcdefghijklmnopqrstuvwxyz_0123456789";
        let anchor = ngram::best_hash(literal).unwrap();
        let shared = (3..=24)
            .find_map(|len| {
                literal
                    .windows(len)
                    .find(|bytes| ngram::hashes_for_document(bytes).contains(&anchor))
            })
            .unwrap();
        let documents = [
            ngram::hashes_for_document(literal),
            ngram::hashes_for_document(shared),
        ];
        let load = |hash| {
            Ok(documents
                .iter()
                .enumerate()
                .filter(|(_, hashes)| hashes.contains(&hash))
                .map(|(id, _)| id as u32)
                .collect())
        };
        assert_eq!(
            intersect_postings(&[vec![anchor]], load).unwrap().unwrap(),
            BTreeSet::from([0, 1])
        );
        assert_eq!(
            intersect_postings(&query::fixed_strategy(literal), load)
                .unwrap()
                .unwrap(),
            BTreeSet::from([0])
        );
    }

    #[test]
    fn covering_execution_preserves_matches_with_case_expansion_and_budget_limits() {
        let mut documents: Vec<Vec<u8>> = (0..512)
            .map(|mask| {
                b"asyncmock"
                    .iter()
                    .enumerate()
                    .map(|(i, &byte)| {
                        if mask & (1 << i) == 0 {
                            byte
                        } else {
                            byte.to_ascii_uppercase()
                        }
                    })
                    .collect()
            })
            .collect();
        let long: String = (0..200).map(|i| format!("item{i:03}_")).collect();
        documents.extend([long.as_bytes().to_vec(), b"x".to_vec(), b"".to_vec()]);
        let indexed: Vec<_> = documents
            .iter()
            .map(|doc| ngram::hashes_for_document(doc))
            .collect();
        for (pattern, ignore_case, fixed) in [
            ("AsyncMock".to_owned(), true, false),
            (long.clone(), false, true),
            (format!("{long}|AsyncMock"), true, false),
            (format!("{long}|x"), false, false),
            (format!("(?:{long})?"), false, false),
        ] {
            let strategies = if fixed {
                query::fixed_strategy(pattern.as_bytes())
            } else {
                query::literal_strategies(&pattern, ignore_case).unwrap()
            };
            let mut lookups = 0;
            let candidates = intersect_postings(&strategies, |hash| {
                lookups += 1;
                Ok(indexed
                    .iter()
                    .enumerate()
                    .filter(|(_, hashes)| hashes.contains(&hash))
                    .map(|(id, _)| id as u32)
                    .collect())
            })
            .unwrap();
            assert!(lookups <= query::MAX_INDEX_LOOKUPS);
            if fixed {
                assert_eq!(lookups, query::MAX_INDEX_LOOKUPS);
            }
            let regex = RegexBuilder::new(&pattern)
                .case_insensitive(ignore_case)
                .build()
                .unwrap();
            for (id, document) in documents.iter().enumerate() {
                if regex.is_match(document) {
                    assert!(
                        candidates
                            .as_ref()
                            .is_none_or(|ids| ids.contains(&(id as u32)))
                    );
                }
            }
        }
    }
}
