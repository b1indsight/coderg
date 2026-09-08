use std::{collections::BTreeSet, fs, path::Path};

use anyhow::{Context, Result};
use rayon::prelude::*;
use regex::bytes::{Regex, RegexBuilder};

use crate::{build::MemoryBudget, index, ngram, query};

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
    budget: MemoryBudget,
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
            index::build(path, requested_index_dir, budget)?;
            index::load(path, requested_index_dir)?
        }
        Err(error) => return Err(error),
    };
    if !options.no_refresh {
        match index::refresh(&disk_index, requested_index_dir, budget)? {
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
        .map_init(
            || regex.clone(),
            |regex, &id| -> Result<_> {
                let document = disk_index
                    .manifest
                    .documents
                    .get(id as usize)
                    .with_context(|| "index contains an invalid document ID; rebuild it")?;
                let full_path = root.join(&document.path);
                let bytes = fs::read(&full_path)
                    .with_context(|| format!("cannot read {}", full_path.display()))?;
                let (output, matches) = match_file(regex, &bytes, &document.path, options);
                Ok((matches > 0).then_some(FileResult {
                    id,
                    output,
                    matches,
                }))
            },
        )
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
    strategies: &[query::GramGroup],
) -> Result<Vec<u32>> {
    Ok(
        match intersect_postings(strategies, |hash| index.postings(hash))? {
            Some(candidates) => candidates.into_iter().collect(),
            None => index.all_document_ids(),
        },
    )
}

fn intersect_postings(
    strategies: &[query::GramGroup],
    mut load_postings: impl FnMut(ngram::GramHash) -> Result<Vec<u32>>,
) -> Result<Option<BTreeSet<u32>>> {
    let mut cache = ngram::GramHashMap::default();
    let mut candidates: Option<BTreeSet<u32>> = None;
    let mut seeded = Vec::new();
    // Give every group one complete OR of branch anchors before spending the
    // shared budget on refinement. An unvisited OR branch cannot be discarded.
    for alternatives in strategies {
        if alternatives.is_empty() || alternatives.iter().any(Vec::is_empty) {
            continue;
        }
        let new_hashes: BTreeSet<_> = alternatives
            .iter()
            .map(|branch| branch[0])
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
            .flat_map(|branch| cache[&branch[0]].iter().copied())
            .collect();
        if let Some(current) = &mut candidates {
            current.retain(|id| union.contains(id));
        } else {
            candidates = Some(union);
        }
        if candidates.as_ref().is_some_and(BTreeSet::is_empty) {
            return Ok(candidates);
        }
        seeded.push(alternatives);
    }
    let Some(mut candidates) = candidates else {
        return Ok(None);
    };
    for alternatives in seeded {
        if alternatives.iter().all(|branch| branch.len() == 1) {
            continue;
        }
        let mut branches: Vec<_> = alternatives.iter().collect();
        // Reuse free cached filters first, then prefer branches needing fewer
        // new lookups. Posting lengths break ties using observed file counts.
        branches.sort_by_key(|branch| {
            (
                branch
                    .iter()
                    .filter(|hash| !cache.contains_key(hash))
                    .count(),
                cache[&branch[0]].len(),
            )
        });
        let mut accepted = BTreeSet::new();
        for branch in branches {
            let mut remaining: Vec<_> = cache[&branch[0]]
                .iter()
                .copied()
                .filter(|id| candidates.contains(id) && !accepted.contains(id))
                .collect();
            if remaining.is_empty() {
                continue;
            }
            let (mut cached, unread): (Vec<_>, Vec<_>) = branch[1..]
                .iter()
                .copied()
                .partition(|hash| cache.contains_key(hash));
            cached.sort_by_key(|hash| cache[hash].len());
            for hash in cached.into_iter().chain(unread) {
                if !cache.contains_key(&hash) {
                    if cache.len() == query::MAX_INDEX_LOOKUPS {
                        // Dropping a conjunction term only broadens this
                        // branch. All alternatives still contribute to the OR.
                        continue;
                    }
                    cache.insert(hash, load_postings(hash)?);
                }
                let posting = &cache[&hash];
                remaining.retain(|id| posting.binary_search(id).is_ok());
                if remaining.is_empty() {
                    break;
                }
            }
            accepted.extend(remaining);
            // Other branches cannot add anything outside the current global
            // candidate set, so no more refinement can change this group.
            if accepted.len() == candidates.len() {
                break;
            }
        }
        candidates = accepted;
        if candidates.is_empty() {
            break;
        }
    }
    Ok(Some(candidates))
}

fn match_file(regex: &Regex, bytes: &[u8], path: &Path, options: &Options) -> (Vec<String>, usize) {
    let mut lines = Vec::new();
    let mut matches = 0;
    let limit = options.max_count.unwrap_or(usize::MAX);
    let mut start = 0;
    // Exclude LF from each haystack, but retain CR for the regex to interpret.
    // An empty file or the position after a final LF is not an extra line.
    for (line_index, end) in memchr::memchr_iter(b'\n', bytes)
        .chain(std::iter::once(bytes.len()))
        .enumerate()
    {
        if start == bytes.len() || matches >= limit {
            break;
        }
        let line = &bytes[start..end];
        start = end + 1;
        if !regex.is_match(line) {
            continue;
        }
        matches += 1;
        if options.files_with_matches {
            break;
        }
        if !options.count {
            let text = String::from_utf8_lossy(line);
            lines.push(format!("{}:{}:{}", path.display(), line_index + 1, text));
        }
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
    fn whitespace_and_dotall_cannot_match_across_lines() {
        for pattern in [r"foo\s+bar", r"(?s)foo.*bar", "foo\nbar"] {
            let regex = RegexBuilder::new(pattern).multi_line(true).build().unwrap();
            let (lines, count) =
                match_file(&regex, b"foo\nbar\nfoo bar\n", Path::new("x"), &options());
            if pattern.contains('\n') {
                assert_eq!((lines, count), (vec![], 0));
            } else {
                assert_eq!((lines, count), (vec!["x:3:foo bar".to_owned()], 1));
            }
        }
    }

    #[test]
    fn empty_matches_only_report_real_lines() {
        let regex = Regex::new("").unwrap();
        for (bytes, expected) in [
            (b"".as_slice(), vec![]),
            (b"\n\n".as_slice(), vec!["x:1:", "x:2:"]),
            (b"foo\n".as_slice(), vec!["x:1:foo"]),
            (b"foo\nbar".as_slice(), vec!["x:1:foo", "x:2:bar"]),
        ] {
            let (lines, count) = match_file(&regex, bytes, Path::new("x"), &options());
            assert_eq!(count, expected.len());
            assert_eq!(lines, expected);
        }
    }

    #[test]
    fn anchors_apply_to_each_line_and_preserve_carriage_returns() {
        for pattern in [r"\Afoo\z", r"(?-m)^foo$"] {
            let regex = RegexBuilder::new(pattern).multi_line(true).build().unwrap();
            let (lines, count) =
                match_file(&regex, b"bar\nfoo\r\nfoo\nfoo", Path::new("x"), &options());
            assert_eq!(count, 2);
            assert_eq!(lines, ["x:3:foo", "x:4:foo"]);
        }
        let regex = Regex::new(r"foo\r?$").unwrap();
        let (lines, count) = match_file(&regex, b"foo\r\n", Path::new("x"), &options());
        assert_eq!(count, 1);
        assert_eq!(lines, ["x:1:foo\r"]);
    }

    #[test]
    fn count_limits_and_file_names_count_matching_lines() {
        let regex = Regex::new("foo").unwrap();
        let bytes = b"foo foo\nbar\nfoo\n";
        let mut options = options();
        options.count = true;
        assert_eq!(
            match_file(&regex, bytes, Path::new("x"), &options),
            (vec![], 2)
        );
        options.max_count = Some(1);
        assert_eq!(
            match_file(&regex, bytes, Path::new("x"), &options),
            (vec![], 1)
        );
        options.max_count = Some(0);
        assert_eq!(
            match_file(&regex, bytes, Path::new("x"), &options),
            (vec![], 0)
        );
        options.max_count = None;
        options.count = false;
        options.files_with_matches = true;
        assert_eq!(
            match_file(&regex, bytes, Path::new("x"), &options),
            (vec![], 1)
        );
    }

    #[test]
    fn unions_variants_intersects_fragments_and_reuses_lookups() {
        let mut looked_up = BTreeSet::new();
        let candidates =
            intersect_postings(&[vec![vec![1], vec![2]], vec![vec![2], vec![3]]], |hash| {
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
    fn literal_branches_reject_mixed_fragments() {
        // All 16 possible combinations of four keys, including A+D and B+C.
        let mut looked_up = BTreeSet::new();
        let candidates = intersect_postings(&[vec![vec![0, 1], vec![2, 3]]], |hash| {
            assert!(looked_up.insert(hash));
            Ok((0..16).filter(|mask| mask & (1 << hash) != 0).collect())
        })
        .unwrap()
        .unwrap();
        let expected = (0..16)
            .filter(|mask| mask & 0b0011 == 0b0011 || mask & 0b1100 == 0b1100)
            .collect();
        assert_eq!(candidates, expected);
        assert_eq!(looked_up.len(), 4);
    }

    #[test]
    fn refinement_skips_accepted_files_and_stops_empty_branches() {
        let mut looked_up = BTreeSet::new();
        let candidates = intersect_postings(&[vec![vec![1, 2, 3], vec![4]]], |hash| {
            assert!(looked_up.insert(hash));
            Ok(match hash {
                1 => vec![1, 2],
                2 | 4 => vec![1],
                3 => panic!("remaining branch files were already excluded"),
                _ => unreachable!(),
            })
        })
        .unwrap()
        .unwrap();
        assert_eq!(candidates, BTreeSet::from([1]));
        assert_eq!(looked_up, BTreeSet::from([1, 2, 4]));
    }

    #[test]
    fn refinement_budget_keeps_every_branch() {
        let mut branches: query::GramGroup = (0..query::MAX_INDEX_LOOKUPS as u32)
            .map(|hash| vec![hash])
            .collect();
        branches[0].push(query::MAX_INDEX_LOOKUPS as u32);
        let candidates = intersect_postings(&[branches], |hash| {
            assert!(hash < query::MAX_INDEX_LOOKUPS as u32);
            Ok(vec![hash])
        })
        .unwrap()
        .unwrap();
        assert_eq!(candidates.len(), query::MAX_INDEX_LOOKUPS);
        assert!(candidates.contains(&0));
    }

    #[test]
    fn budget_never_applies_an_incomplete_alternative_union() {
        let limit = query::MAX_INDEX_LOOKUPS as u32;
        let mut strategies: Vec<query::GramGroup> = (0..limit)
            .collect::<Vec<_>>()
            .chunks(query::MAX_LITERAL_VARIANTS)
            .map(|chunk| chunk.iter().map(|&hash| vec![hash]).collect())
            .collect();
        // The cached alternative only contains document 9. Document 7 is in
        // the unvisited alternative, so applying a partial union would lose it.
        strategies.push(vec![vec![limit - 1], vec![limit]]);
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
        let alternatives = (0..=query::MAX_INDEX_LOOKUPS as u32)
            .map(|hash| vec![hash])
            .collect();
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
            intersect_postings(&[vec![vec![anchor]]], load)
                .unwrap()
                .unwrap(),
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
