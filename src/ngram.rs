use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
    sync::OnceLock,
};

pub const MIN_GRAM: usize = 3;
pub(crate) const MAX_GRAM: usize = 24;
const HASH_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const HASH_PRIME: u64 = 0x1000_0000_01b3;
const PAIR_TABLE_THRESHOLD: usize = 4096;
static PAIR_WEIGHT_TABLE: OnceLock<Box<[u64]>> = OnceLock::new();

#[derive(Default)]
pub(crate) struct GramHasher(u64);

impl Hasher for GramHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 = hash(bytes);
    }

    fn write_u64(&mut self, value: u64) {
        self.0 = value;
    }

    fn write_u32(&mut self, value: u32) {
        // HashSet uses the high hash bits to filter candidate buckets. Gram
        // hashes are already mixed, but widening them alone leaves those bits
        // zero. Spread them across u64 without changing the stored gram key.
        self.0 = u64::from(value).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }
}

type GramBuildHasher = BuildHasherDefault<GramHasher>;
pub(crate) type GramHash = u32;
pub(crate) type GramHashSet = HashSet<GramHash, GramBuildHasher>;
pub(crate) type GramHashMap<V> = HashMap<GramHash, V, GramBuildHasher>;

/// Emit every trigram plus longer sparse n-grams whose endpoint bigram
/// weights dominate all weights inside the span. The predicate only depends
/// on bytes inside the n-gram, so a gram selected from a query is guaranteed
/// to have been selected from every document containing that query.
pub fn hashes_for_chunk(bytes: &[u8]) -> Vec<GramHash> {
    // The full gram hash is already mixed, so u64 keys can use it directly
    // for table lookup. Compact only the hashes that survive chunk dedup.
    let mut seen =
        HashSet::<u64, GramBuildHasher>::with_capacity_and_hasher(bytes.len(), Default::default());
    let mut hashes = Vec::new();
    for_each_sparse_gram(bytes, |_, _, hash| {
        if seen.insert(hash) {
            // Different u64 hashes can compact to the same key. The postings
            // builder removes these duplicates after sorting the u32 records.
            hashes.push(compact_hash(hash));
        }
    });
    hashes
}

/// Original set-based extraction, retained as a test oracle.
#[cfg(test)]
pub fn hashes_for_document(bytes: &[u8]) -> GramHashSet {
    let mut hashes = GramHashSet::with_capacity_and_hasher(bytes.len(), Default::default());
    for_each_sparse_gram(bytes, |_, _, hash| {
        hashes.insert(compact_hash(hash));
    });
    hashes
}

/// The previous single-gram heuristic, retained as a test oracle.
#[cfg(test)]
pub fn best_hash(literal: &[u8]) -> Option<GramHash> {
    let mut best = None;
    for_each_sparse_gram(literal, |_, gram, hash| {
        if best.is_none_or(|(length, _)| gram.len() >= length) {
            best = Some((gram.len(), hash));
        }
    });
    best.map(|(_, hash)| compact_hash(hash))
}

#[derive(Clone, Copy)]
struct QueryGram {
    start: usize,
    len: usize,
    hash: GramHash,
}

/// Cover all trigram positions using indexed grams. Keep the previous longest
/// gram first, then prefer longer complementary grams. Dropping necessary
/// hashes at the budget boundary only broadens the candidate set.
pub fn covering_hashes(literal: &[u8], limit: usize) -> Vec<GramHash> {
    let mut grams = covering_grams(literal);
    grams.sort_unstable_by_key(|gram| std::cmp::Reverse((gram.len, gram.start)));
    let mut seen = GramHashSet::default();
    grams
        .into_iter()
        .map(|gram| gram.hash)
        .filter(|&hash| seen.insert(hash))
        .take(limit)
        .collect()
}

fn covering_grams(literal: &[u8]) -> Vec<QueryGram> {
    let mut longest: Vec<QueryGram> = Vec::new();
    for_each_sparse_gram(literal, |start, gram, hash| {
        let candidate = QueryGram {
            start,
            len: gram.len(),
            hash: compact_hash(hash),
        };
        if start == longest.len() {
            longest.push(candidate);
        } else {
            longest[start] = candidate;
        }
    });
    let Some(&anchor) = longest.iter().max_by_key(|gram| gram.len) else {
        return Vec::new();
    };
    let mut selected = vec![anchor];
    let mut next_start = 0;
    let mut covered = MIN_GRAM - 1;
    while covered < literal.len() {
        let mut best = longest[next_start];
        // Overlap by two bytes so every trigram position is covered, including
        // the joins. Of all eligible intervals, take the one reaching furthest.
        while next_start < longest.len() && next_start <= covered - (MIN_GRAM - 1) {
            let gram = longest[next_start];
            if gram.start + gram.len >= best.start + best.len {
                best = gram;
            }
            next_start += 1;
        }
        covered = best.start + best.len;
        selected.push(best);
    }
    selected
}

fn compact_hash(hash: u64) -> GramHash {
    hash as u32 ^ (hash >> 32) as u32
}

fn for_each_sparse_gram<'a>(bytes: &'a [u8], mut emit: impl FnMut(usize, &'a [u8], u64)) {
    if bytes.len() < MIN_GRAM {
        return;
    }

    // Only nearby pairs are inspected. Look up their weights as needed instead
    // of allocating an eight-byte weight for every byte in the document.
    let table = (bytes.len() >= PAIR_TABLE_THRESHOLD).then(pair_weight_table);
    let weight = |index: usize| {
        let pair = &bytes[index..index + 2];
        match table {
            Some(table) => table[usize::from(u16::from_le_bytes([pair[0], pair[1]]))],
            None => pair_weight(pair),
        }
    };
    for start in 0..=bytes.len() - MIN_GRAM {
        let max_len = (bytes.len() - start).min(MAX_GRAM);
        let mut hash_state = HASH_OFFSET;
        for &byte in &bytes[start..start + MIN_GRAM] {
            hash_state = extend_hash(hash_state, byte);
        }
        emit(
            start,
            &bytes[start..start + MIN_GRAM],
            finish_hash(hash_state, MIN_GRAM),
        );
        let left = weight(start);
        let mut max_inside = weight(start + 1);
        if max_inside >= left {
            continue;
        }
        for len in MIN_GRAM + 1..=max_len {
            hash_state = extend_hash(hash_state, bytes[start + len - 1]);
            let right_index = start + len - 2;
            let right = weight(right_index);
            if left > max_inside && right > max_inside {
                emit(
                    start,
                    &bytes[start..start + len],
                    finish_hash(hash_state, len),
                );
            }
            max_inside = max_inside.max(right);
            if max_inside >= left {
                break;
            }
        }
    }
}

fn pair_weight_table() -> &'static [u64] {
    PAIR_WEIGHT_TABLE.get_or_init(|| {
        (0..=u16::MAX)
            .map(pair_weight_value)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    })
}

fn pair_weight(pair: &[u8]) -> u64 {
    pair_weight_value(u16::from_le_bytes([pair[0], pair[1]]))
}

fn pair_weight_value(value: u16) -> u64 {
    mix(u64::from(value) ^ 0x9e37_79b9_7f4a_7c15)
}

pub fn hash(bytes: &[u8]) -> u64 {
    let mut value = HASH_OFFSET;
    for &byte in bytes {
        value = extend_hash(value, byte);
    }
    finish_hash(value, bytes.len())
}

fn extend_hash(value: u64, byte: u8) -> u64 {
    (value ^ u64::from(byte)).wrapping_mul(HASH_PRIME)
}

fn finish_hash(value: u64, len: usize) -> u64 {
    mix(value ^ len as u64)
}

fn mix(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gram_hasher_distributes_bucket_and_tag_bits() {
        let mut tags = HashSet::new();
        let mut buckets = HashSet::new();
        for key in 0..4096 {
            let mut hasher = GramHasher::default();
            hasher.write_u32(key);
            let hash = hasher.finish();
            tags.insert(hash >> 57);
            buckets.insert(hash & 255);
        }
        assert_eq!(tags.len(), 128);
        assert_eq!(buckets.len(), 256);
    }

    #[test]
    fn chunk_output_matches_set_extraction() {
        let mut state = 12345_u32;
        let chunk_bytes = crate::build::CHUNK_BYTES;
        let random: Vec<u8> = (0..chunk_bytes + MAX_GRAM - 1)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                state as u8
            })
            .collect();
        let repeated_line = b"fn repeated_name() { repeated_name(); }\n";
        let repeated = repeated_line.repeat((chunk_bytes + MAX_GRAM).div_ceil(repeated_line.len()));
        for bytes in [&random[..], &repeated[..]] {
            for len in [
                0,
                1,
                2,
                3,
                24,
                4095,
                4096,
                16 * 1024,
                chunk_bytes,
                bytes.len(),
            ] {
                let mut emitted = 0;
                for_each_sparse_gram(&bytes[..len], |_, _, _| emitted += 1);
                assert!(emitted <= 3 * len, "worker memory bound at length {len}");
                let hashes = hashes_for_chunk(&bytes[..len]);
                let actual: GramHashSet = hashes.iter().copied().collect();
                assert_eq!(actual, hashes_for_document(&bytes[..len]), "length {len}");
            }
        }
    }

    #[test]
    fn chunk_output_preserves_compacted_collisions() {
        let first = b"'U\"";
        let second = b")gh";
        let key = compact_hash(hash(first));
        assert_ne!(hash(first), hash(second));
        assert_eq!(key, compact_hash(hash(second)));

        let bytes = [first.as_slice(), second.as_slice()].concat();
        let mut hashes = hashes_for_chunk(&bytes);
        assert_eq!(hashes.iter().filter(|&&value| value == key).count(), 2);
        hashes.sort_unstable();
        hashes.dedup();
        let mut expected: Vec<_> = hashes_for_document(&bytes).into_iter().collect();
        expected.sort_unstable();
        assert_eq!(hashes, expected);
    }

    #[test]
    fn chosen_query_gram_is_always_in_document_grams() {
        let query = b"MAX_FILE_SIZE";
        let document = b"prefix MAX_FILE_SIZE suffix";
        let selected = best_hash(query).unwrap();
        assert!(hashes_for_document(document).contains(&selected));
    }

    #[test]
    fn short_literals_have_no_gram() {
        assert_eq!(best_hash(b"ab"), None);
        for bytes in [b"".as_slice(), b"a", b"ab"] {
            assert!(covering_hashes(bytes, 128).is_empty());
        }
    }

    #[test]
    fn covering_spans_every_trigram_and_only_uses_indexed_keys() {
        let mut state = 17u64;
        for len in 3..160 {
            for alphabet in [2, 7, 256] {
                let literal: Vec<_> = (0..len)
                    .map(|_| {
                        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                        ((state >> 32) % alphabet) as u8
                    })
                    .collect();
                let grams = covering_grams(&literal);
                for start in 0..=len - MIN_GRAM {
                    assert!(grams.iter().any(
                        |gram| gram.start <= start && gram.start + gram.len >= start + MIN_GRAM
                    ));
                }
                let selected = covering_hashes(&literal, 128);
                assert_eq!(selected.first().copied(), best_hash(&literal));
                assert_eq!(
                    selected.len(),
                    selected.iter().collect::<HashSet<_>>().len()
                );
                for context in [0, 97, 255] {
                    let mut document = vec![context; 25];
                    document.extend_from_slice(&literal);
                    document.extend_from_slice(&[context; 25]);
                    let indexed = hashes_for_document(&document);
                    assert!(selected.iter().all(|hash| indexed.contains(hash)));
                }
            }
        }
    }

    #[test]
    fn covering_deduplicates_repeated_bytes_and_respects_budget() {
        let repeated = vec![b'a'; 5000];
        assert_eq!(
            covering_hashes(&repeated, 128),
            vec![best_hash(&repeated).unwrap()]
        );
        let mut state = 17u64;
        let literal: Vec<_> = (0..5000)
            .map(|_| {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                (state >> 32) as u8
            })
            .collect();
        let all = covering_hashes(&literal, usize::MAX);
        assert!(all.len() > 128);
        for limit in [0, 1, 4, 128] {
            assert_eq!(covering_hashes(&literal, limit), all[..limit]);
        }
    }

    #[test]
    fn incremental_maximum_preserves_sparse_selection() {
        let long: Vec<u8> = (0..PAIR_TABLE_THRESHOLD + MAX_GRAM)
            .map(|i| (i * 73 + i / 256) as u8)
            .collect();
        for bytes in [
            b"abc".as_slice(),
            b"MAX_FILE_SIZE".as_slice(),
            b"0123456789abcdefghijklmnopqrstuvwxyz".as_slice(),
            &long[..PAIR_TABLE_THRESHOLD - 1],
            &long[..PAIR_TABLE_THRESHOLD],
            long.as_slice(),
        ] {
            let mut actual = Vec::new();
            for_each_sparse_gram(bytes, |_, gram, gram_hash| {
                actual.push((gram.to_vec(), gram_hash));
            });

            let weights: Vec<u64> = bytes.windows(2).map(pair_weight).collect();
            let mut expected = Vec::new();
            for start in 0..=bytes.len() - MIN_GRAM {
                let max_len = (bytes.len() - start).min(MAX_GRAM);
                for len in MIN_GRAM..=max_len {
                    let right_index = start + len - 2;
                    let sparse = weights[start + 1..right_index]
                        .iter()
                        .all(|&inside| weights[start] > inside && weights[right_index] > inside);
                    if len == MIN_GRAM || sparse {
                        let gram = &bytes[start..start + len];
                        expected.push((gram.to_vec(), hash(gram)));
                    }
                }
            }
            assert_eq!(actual, expected);
            assert_eq!(
                best_hash(bytes),
                expected
                    .iter()
                    .max_by_key(|(gram, _)| gram.len())
                    .map(|(_, gram_hash)| compact_hash(*gram_hash))
            );
        }
    }

    #[test]
    fn pair_weight_table_preserves_original_weights() {
        let table = pair_weight_table();
        for value in 0..=u16::MAX {
            assert_eq!(table[usize::from(value)], pair_weight_value(value));
        }
    }
}
