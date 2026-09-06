use anyhow::{Context, Result};
use regex_syntax::{
    ParserBuilder,
    hir::literal::{ExtractKind, Extractor},
};

use crate::ngram;

/// Each inner vector is an alternative. A matching document must contain at
/// least one alternative, so their posting lists are unioned.
pub fn literal_strategies(pattern: &str, ignore_case: bool) -> Result<Vec<Vec<ngram::GramHash>>> {
    if ignore_case {
        return Ok(Vec::new());
    }

    let hir = ParserBuilder::new()
        .build()
        .parse(pattern)
        .with_context(|| format!("invalid regular expression {pattern:?}"))?;
    let mut strategies = Vec::new();
    for kind in [ExtractKind::Prefix, ExtractKind::Suffix] {
        let mut extractor = Extractor::new();
        extractor.kind(kind);
        let sequence = extractor.extract(&hir);
        let Some(literals) = sequence.literals() else {
            continue;
        };
        if literals.is_empty() {
            strategies.push(Vec::new());
            continue;
        }
        let hashes: Option<Vec<ngram::GramHash>> = literals
            .iter()
            .map(|literal| ngram::best_hash(literal.as_bytes()))
            .collect();
        if let Some(mut hashes) = hashes {
            hashes.sort_unstable();
            hashes.dedup();
            strategies.push(hashes);
        }
    }
    Ok(strategies)
}

pub fn fixed_strategy(literal: &[u8], ignore_case: bool) -> Vec<Vec<ngram::GramHash>> {
    if ignore_case {
        return Vec::new();
    }
    ngram::best_hash(literal)
        .map(|hash| vec![vec![hash]])
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternation_becomes_a_union() {
        let strategies = literal_strategies("foobar|quuxxx", false).unwrap();
        assert!(strategies.iter().any(|strategy| strategy.len() == 2));
    }

    #[test]
    fn unbounded_prefix_can_use_suffix() {
        let strategies = literal_strategies(".*needle", false).unwrap();
        assert!(strategies.iter().any(|strategy| !strategy.is_empty()));
    }
}
