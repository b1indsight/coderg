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
// Bound a single class independently of combinations across small classes.
const MAX_INITIAL_CLASS_VARIANTS: usize = 16;

/// Union of literal branches, each requiring the intersection of its grams.
pub type GramGroup = Vec<Vec<ngram::GramHash>>;

/// Intersect groups, union branches within a group, intersect grams within a
/// branch. An empty plan means there is no safe index filter.
pub fn literal_strategies(pattern: &str, ignore_case: bool) -> Result<Vec<GramGroup>> {
    let hir = ParserBuilder::new()
        .case_insensitive(ignore_case)
        .multi_line(true)
        .utf8(false)
        .build()
        .parse(pattern)
        .with_context(|| format!("invalid regular expression {pattern:?}"))?;
    let mut strategies = Vec::new();
    for literals in required_literal_groups(&hir) {
        let covers: Vec<_> = literals
            .iter()
            .map(|literal| ngram::covering_hashes(literal, MAX_INDEX_LOOKUPS))
            .collect();
        push_strategy(covers, &mut strategies);
    }
    Ok(strategies)
}

type LiteralGroup = Vec<Vec<u8>>;

fn required_literal_groups(hir: &Hir) -> Vec<LiteralGroup> {
    let mut groups = Vec::new();
    // Small quote and case-folding classes can join their surrounding text;
    // the 53-character decorator name class stops at @torch. | @pytest.
    collect_required_literals(hir, MAX_INITIAL_CLASS_VARIANTS, &mut groups);
    if groups.is_empty() {
        // Retry with larger classes only when small classes cannot
        // provide a complete usable filter. Never truncate optional branches.
        collect_required_literals(hir, MAX_LITERAL_VARIANTS, &mut groups);
    }
    groups
}

fn collect_required_literals(hir: &Hir, class_limit: usize, groups: &mut Vec<LiteralGroup>) {
    if groups.len() >= MAX_INDEX_LOOKUPS {
        return;
    }
    extract_literals(hir, class_limit, groups);
    match hir.kind() {
        HirKind::Concat(parts) => {
            // Case folding turns an identifier into a concatenation of small
            // Unicode classes. Short windows avoid expanding the whole word.
            for window in parts.windows(3) {
                if groups.len() >= MAX_INDEX_LOOKUPS {
                    return;
                }
                extract_literals(&Hir::concat(window.to_vec()), class_limit, groups);
            }
            for part in parts {
                collect_required_literals(part, class_limit, groups);
            }
        }
        HirKind::Capture(capture) => collect_required_literals(&capture.sub, class_limit, groups),
        HirKind::Repetition(repetition) if repetition.min > 0 => {
            collect_required_literals(&repetition.sub, class_limit, groups);
        }
        // Optional repetitions and individual alternation branches are not
        // required. Only the complete expression's extracted unions are safe.
        _ => {}
    }
}

fn extract_literals(hir: &Hir, class_limit: usize, groups: &mut Vec<LiteralGroup>) {
    for kind in [ExtractKind::Prefix, ExtractKind::Suffix] {
        if groups.len() >= MAX_INDEX_LOOKUPS {
            return;
        }
        let mut extractor = Extractor::new();
        extractor
            .kind(kind)
            .limit_class(class_limit)
            .limit_total(MAX_LITERAL_VARIANTS);
        let sequence = extractor.extract(hir);
        let Some(literals) = sequence.literals() else {
            continue;
        };
        if literals.is_empty()
            || literals
                .iter()
                .any(|literal| literal.as_bytes().len() < ngram::MIN_GRAM)
        {
            continue;
        }
        // The extractor can shorten literals while preserving every variant.
        // Never truncate the alternative list ourselves: that would miss files.
        let group = literals
            .iter()
            .map(|literal| literal.as_bytes().to_vec())
            .collect();
        push_literal_group(group, groups);
    }
}

fn push_literal_group(mut group: LiteralGroup, groups: &mut Vec<LiteralGroup>) {
    group.sort_unstable();
    group.dedup();
    if groups
        .iter()
        .any(|existing| literal_group_implies(existing, &group))
    {
        return;
    }
    groups.retain(|existing| !literal_group_implies(&group, existing));
    groups.push(group);
}

fn literal_group_implies(stronger: &LiteralGroup, weaker: &LiteralGroup) -> bool {
    // A group is an OR of whole literals. EVERY stronger alternative must
    // contain a weaker alternative before that entire weaker group is redundant.
    // This removes torch | pytest once @torch. | @pytest. is available without
    // dropping an unrelated branch. All literals here contain at least 3 bytes.
    stronger.iter().all(|literal| {
        weaker.iter().any(|part| {
            literal
                .windows(part.len())
                .any(|window| window == part.as_slice())
        })
    })
}

fn push_strategy(covers: GramGroup, strategies: &mut Vec<GramGroup>) {
    let mut branches: GramGroup = Vec::new();
    for cover in covers {
        // Simplify using actual key requirements, not just literal lengths.
        // In an OR, a stricter branch adds no files to a weaker branch.
        if branches
            .iter()
            .any(|existing| existing.iter().all(|hash| cover.contains(hash)))
        {
            continue;
        }
        branches.retain(|existing| !cover.iter().all(|hash| existing.contains(hash)));
        branches.push(cover);
    }
    if !strategies.contains(&branches) {
        strategies.push(branches);
    }
}

pub fn fixed_strategy(literal: &[u8]) -> Vec<GramGroup> {
    let cover = ngram::covering_hashes(literal, MAX_INDEX_LOOKUPS);
    if cover.is_empty() {
        Vec::new()
    } else {
        vec![vec![cover]]
    }
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
                    alternatives
                        .iter()
                        .any(|branch| branch.iter().all(|hash| hashes.contains(hash))),
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

    fn groups_for(pattern: &str) -> Vec<LiteralGroup> {
        let hir = ParserBuilder::new()
            .multi_line(true)
            .utf8(false)
            .build()
            .parse(pattern)
            .unwrap();
        required_literal_groups(&hir)
    }

    #[test]
    fn decorator_uses_two_fixed_literals_without_expanding_the_name() {
        let pattern = r"^[ \t]*@(?:torch|pytest)\.[A-Za-z_]+";
        assert_eq!(
            groups_for(pattern),
            vec![vec![b"@pytest.".to_vec(), b"@torch.".to_vec()]]
        );
        assert_eq!(
            literal_strategies(pattern, false).unwrap(),
            literal_strategies(r"@(?:torch|pytest)\.", false).unwrap()
        );
        let documents: Vec<_> = [
            "@torch.no_grad()",
            "    @pytest.mark.parametrize('x', [1])",
            "\t@torch._private()",
            "@pytest.Z",
            "@torch.",
            "@pytest.123",
            "value = torch.no_grad()",
        ]
        .into_iter()
        .map(|text| text.as_bytes().to_vec())
        .collect();
        assert_filter_preserves_matches(pattern, false, &documents);
    }

    #[test]
    fn large_classes_stop_literals_and_keep_separate_required_fragments() {
        assert_eq!(
            groups_for(r"\btorch\.cuda\.[a-z_]+\("),
            vec![vec![b"torch.cuda.".to_vec()]]
        );
        let mut groups = groups_for("prefix[0-9A-Z]+suffix");
        groups.sort_unstable();
        assert_eq!(
            groups,
            vec![vec![b"prefix".to_vec()], vec![b"suffix".to_vec()]]
        );
        assert_filter_preserves_matches(
            "prefix[0-9A-Z]+suffix",
            false,
            &[b"prefix1suffix".to_vec(), b"prefix12345suffix".to_vec()],
        );
    }

    #[test]
    fn small_quote_classes_preserve_all_twelve_device_literals() {
        let pattern = r#"["'](?:cuda|cpu|rocm)["']"#;
        let mut literals = Vec::new();
        for left in [b'\'', b'"'] {
            for word in [b"cuda".as_slice(), b"cpu", b"rocm"] {
                for right in [b'\'', b'"'] {
                    let mut literal = vec![left];
                    literal.extend_from_slice(word);
                    literal.push(right);
                    literals.push(literal);
                }
            }
        }
        literals.sort_unstable();
        assert_eq!(groups_for(pattern), vec![literals.clone()]);
        // The expression permits independently chosen opening and closing
        // quotes, so mixed quotes must be preserved too.
        assert_filter_preserves_matches(pattern, false, &literals);
    }

    #[test]
    fn small_classes_and_literal_branches_keep_the_wider_combination_budget() {
        assert_eq!(
            groups_for("prefix[ab]suffix"),
            groups_for("prefix(?:a|b)suffix")
        );
        for words in [
            vec!["aa", "bb", "cc", "dd"],
            vec!["aa", "bb", "cc", "dd", "ee"],
        ] {
            let pattern = format!("prefix(?:{})[0-3]+", words.join("|"));
            let groups = groups_for(&pattern);
            assert_eq!(groups.len(), 1);
            let expected: Vec<_> = words
                .iter()
                .flat_map(|word| {
                    (0..4).map(move |digit| format!("prefix{word}{digit}").into_bytes())
                })
                .collect();
            assert_eq!(groups[0], expected);
            let documents: Vec<_> = words
                .iter()
                .flat_map(|word| {
                    (0..4).map(move |digit| format!("prefix{word}{digit}").into_bytes())
                })
                .collect();
            assert_filter_preserves_matches(&pattern, false, &documents);
        }
    }

    #[test]
    fn redundancy_requires_every_alternative_to_imply_the_weaker_group() {
        let pattern = "(?:foobar|unrelated).*foo";
        assert_eq!(groups_for(pattern).len(), 2);
        assert_filter_preserves_matches(
            pattern,
            false,
            &[b"foobarfoo".to_vec(), b"unrelated foo".to_vec()],
        );
    }

    #[test]
    fn missing_small_literal_filter_keeps_wider_bounded_expansion() {
        assert_eq!(
            groups_for("ab[cde]"),
            vec![vec![b"abc".to_vec(), b"abd".to_vec(), b"abe".to_vec()]]
        );
        assert_eq!(groups_for("[a-c]{3}")[0].len(), 27);
        assert!(groups_for("(?:needle|[a-z]{1,2})").is_empty());
        assert!(groups_for("(?:needle)?").is_empty());
    }

    #[test]
    fn covers_long_literals_without_losing_unequal_or_optional_branches() {
        let left = "abcdefghijklmnopqrstuvwxyz_0123456789";
        let right = "zyxwvutsrqponmlkjihgfedcba_9876543210";
        let documents: Vec<_> = [left, right, "xyz", "x", "", "tail"]
            .into_iter()
            .map(|text| text.as_bytes().to_vec())
            .collect();
        assert!(fixed_strategy(left.as_bytes())[0][0].len() > 1);
        assert!(
            literal_strategies(left, false)
                .unwrap()
                .iter()
                .flatten()
                .any(|branch| branch.len() > 1)
        );
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
