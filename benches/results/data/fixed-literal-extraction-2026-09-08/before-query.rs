use anyhow::{Context, Result};
use regex_syntax::{
    ParserBuilder,
    hir::{
        Hir, HirKind,
        literal::{ExtractKind, Extractor},
    },
};

use crate::ngram;

// Tuned on vLLM; see benches/results/vllm-case-budget-2026-09-07.md.
pub const MAX_LITERAL_VARIANTS: usize = 128;
pub const MAX_INDEX_LOOKUPS: usize = 128;

/// Every inner vector is a necessary condition: a matching document contains
/// at least one of its hashes. Union within each condition, intersect between
/// conditions. An empty outer vector means there is no safe index filter.
pub fn literal_strategies(pattern: &str, ignore_case: bool) -> Result<Vec<Vec<ngram::GramHash>>> {
    let hir = ParserBuilder::new()
        .case_insensitive(ignore_case)
        .multi_line(true)
        .utf8(false)
        .build()
        .parse(pattern)
        .with_context(|| format!("invalid regular expression {pattern:?}"))?;
    let mut strategies = Vec::new();
    let mut coverings = Vec::new();
    collect_required_strategies(&hir, &mut strategies, &mut coverings);
    // Preserve all original filters and their budget priority before spending
    // extra lookups on complementary pieces of an individual literal.
    for covers in coverings {
        // Each round includes one necessary hash from EVERY alternative; a
        // shorter cover repeats its last hash. For example, (A & B) | (C & D)
        // implies (A | C) & (B | D). This conservative approximation may admit
        // mixed-branch false positives, but never excludes a matching branch.
        let rounds = covers.iter().map(Vec::len).max().unwrap_or(0);
        for round in 1..rounds {
            if strategies.len() >= MAX_INDEX_LOOKUPS {
                break;
            }
            let hashes = covers
                .iter()
                .map(|cover| cover[round.min(cover.len() - 1)])
                .collect();
            push_strategy(hashes, &mut strategies);
        }
    }
    Ok(strategies)
}

type LiteralCovers = Vec<Vec<ngram::GramHash>>;

fn collect_required_strategies(
    hir: &Hir,
    strategies: &mut Vec<Vec<ngram::GramHash>>,
    coverings: &mut Vec<LiteralCovers>,
) {
    if strategies.len() >= MAX_INDEX_LOOKUPS {
        return;
    }
    extract_strategies(hir, strategies, coverings);
    match hir.kind() {
        HirKind::Concat(parts) => {
            // Case folding turns an identifier into a concatenation of small
            // Unicode classes. Short windows avoid expanding the whole word.
            for window in parts.windows(3) {
                if strategies.len() >= MAX_INDEX_LOOKUPS {
                    return;
                }
                extract_strategies(&Hir::concat(window.to_vec()), strategies, coverings);
            }
            for part in parts {
                collect_required_strategies(part, strategies, coverings);
            }
        }
        HirKind::Capture(capture) => {
            collect_required_strategies(&capture.sub, strategies, coverings)
        }
        HirKind::Repetition(repetition) if repetition.min > 0 => {
            collect_required_strategies(&repetition.sub, strategies, coverings);
        }
        // Optional repetitions and individual alternation branches are not
        // required. Only the complete expression's extracted unions are safe.
        _ => {}
    }
}

fn extract_strategies(
    hir: &Hir,
    strategies: &mut Vec<Vec<ngram::GramHash>>,
    coverings: &mut Vec<LiteralCovers>,
) {
    for kind in [ExtractKind::Prefix, ExtractKind::Suffix] {
        if strategies.len() >= MAX_INDEX_LOOKUPS {
            return;
        }
        let mut extractor = Extractor::new();
        extractor
            .kind(kind)
            .limit_class(MAX_LITERAL_VARIANTS)
            .limit_total(MAX_LITERAL_VARIANTS);
        let sequence = extractor.extract(hir);
        let Some(literals) = sequence.literals() else {
            continue;
        };
        if literals.is_empty() {
            continue;
        }
        // The extractor can shorten literals while preserving every variant.
        // Never truncate the alternative list ourselves: that would miss files.
        let covers: Vec<_> = literals
            .iter()
            .map(|literal| ngram::covering_hashes(literal.as_bytes(), MAX_INDEX_LOOKUPS))
            .collect();
        if covers.iter().all(|cover| !cover.is_empty()) {
            push_strategy(covers.iter().map(|cover| cover[0]).collect(), strategies);
            if !coverings.contains(&covers) {
                coverings.push(covers);
            }
        }
    }
}

fn push_strategy(mut hashes: Vec<ngram::GramHash>, strategies: &mut Vec<Vec<ngram::GramHash>>) {
    hashes.sort_unstable();
    hashes.dedup();
    if !strategies.contains(&hashes) {
        strategies.push(hashes);
    }
}

pub fn fixed_strategy(literal: &[u8]) -> Vec<Vec<ngram::GramHash>> {
    ngram::covering_hashes(literal, MAX_INDEX_LOOKUPS)
        .into_iter()
        .map(|hash| vec![hash])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::bytes::RegexBuilder;

    fn assert_filter_preserves_matches(pattern: &str, ignore_case: bool, documents: &[Vec<u8>]) {
        let regex = RegexBuilder::new(pattern)
            .case_insensitive(ignore_case)
            .multi_line(true)
            .build()
            .unwrap();
        let strategies = literal_strategies(pattern, ignore_case).unwrap();
        assert!(strategies.iter().all(|s| s.len() <= MAX_LITERAL_VARIANTS));
        let mut matching = 0;
        for document in documents {
            if !regex.is_match(document) {
                continue;
            }
            matching += 1;
            let hashes = ngram::hashes_for_document(document);
            for alternatives in &strategies {
                assert!(
                    alternatives.iter().any(|hash| hashes.contains(hash)),
                    "filter lost {:?} for {pattern:?}",
                    String::from_utf8_lossy(document)
                );
            }
        }
        assert!(matching > 0, "test corpus must exercise a match");
    }

    #[test]
    fn alternation_becomes_a_union() {
        let strategies = literal_strategies("foobar|quuxxx", false).unwrap();
        assert!(strategies.iter().any(|strategy| strategy.len() == 2));
    }

    #[test]
    fn covers_long_literals_without_losing_unequal_or_optional_branches() {
        let left = "abcdefghijklmnopqrstuvwxyz_0123456789";
        let right = "zyxwvutsrqponmlkjihgfedcba_9876543210";
        let documents: Vec<_> = [left, right, "xyz", "x", "", "tail"]
            .into_iter()
            .map(|text| text.as_bytes().to_vec())
            .collect();
        assert!(fixed_strategy(left.as_bytes()).len() > 1);
        assert!(literal_strategies(left, false).unwrap().len() > 1);
        for pattern in [
            format!("{left}|{right}"),
            format!("{left}|xyz"),
            format!("{left}|x"),
            format!("(?:{left})?"),
            format!("(?:{left}|{right})*tail"),
        ] {
            assert_filter_preserves_matches(&pattern, false, &documents);
        }
        assert!(
            literal_strategies(&format!("{left}|x"), false)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn unbounded_prefix_can_use_suffix() {
        let strategies = literal_strategies(".*needle", false).unwrap();
        assert!(strategies.iter().any(|strategy| !strategy.is_empty()));
    }

    #[test]
    fn indexes_every_ascii_case_variant_of_a_long_identifier() {
        let word = b"asyncmock";
        let documents: Vec<_> = (0..1 << word.len())
            .map(|mask| {
                word.iter()
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
        assert!(!literal_strategies("AsyncMock", true).unwrap().is_empty());
        assert_filter_preserves_matches("AsyncMock", true, &documents);
    }

    #[test]
    fn unicode_folding_and_optional_branches_preserve_matches() {
        let documents: Vec<_> = [
            "Kelvin",
            "KELVIN",
            "ſigma",
            "SIGMA",
            "Σςσ",
            "σσσ",
            "FOOBAR",
            "fooX",
            "X",
            "ab",
            "bar",
            "FOOFOObar",
            "fOoBAR",
            "NEEDLE",
            "12",
        ]
        .into_iter()
        .map(|text| text.as_bytes().to_vec())
        .collect();
        for pattern in [
            "kelvin",
            "sigma",
            "σσσ",
            "foo(?:bar|x)",
            "(?:foo|x)",
            "(?:foo)?bar",
            "(?:foo)*bar",
            "(?:foo)+bar",
            "(?i:foo)(?-i:BAR)",
            "(?:needle|[a-z]{1,2})",
            r"\b[0-9]{2}\b",
        ] {
            assert_filter_preserves_matches(pattern, true, &documents);
        }
        assert!(literal_strategies("(?:foo|x)", true).unwrap().is_empty());
        assert_filter_preserves_matches("(?i:foo)(?-i:BAR)", false, &documents);
    }

    #[test]
    fn expansion_limit_keeps_all_alternation_branches() {
        let alternatives: Vec<_> = (0..100).map(|i| format!("item{i:03}")).collect();
        let pattern = format!("prefix(?:{})tail", alternatives.join("|"));
        let documents: Vec<_> = alternatives
            .iter()
            .map(|item| format!("PREFIX{item}TAIL").into_bytes())
            .collect();
        assert!(!literal_strategies(&pattern, true).unwrap().is_empty());
        assert_filter_preserves_matches(&pattern, true, &documents);
    }

    #[test]
    fn byte_mode_patterns_use_the_same_parser_rules_as_search() {
        assert_filter_preserves_matches(
            r"(?-u:\xFF)needle",
            true,
            &[b"\xffNEEDLE".to_vec(), b"\xffneedle".to_vec()],
        );
    }
}
