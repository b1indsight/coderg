use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
    sync::OnceLock,
};

pub const MIN_GRAM: usize = 3;
const MAX_GRAM: usize = 24;
const HASH_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const HASH_PRIME: u64 = 0x1000_0000_01b3;
const PAIR_TABLE_THRESHOLD: usize = 4096;
static PAIR_WEIGHT_TABLE: OnceLock<Box<[u64]>> = OnceLock::new();

#[derive(Default)]
pub(crate) struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
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
        self.0 = u64::from(value);
    }
}

type IdentityBuildHasher = BuildHasherDefault<IdentityHasher>;
pub(crate) type GramHash = u32;
pub(crate) type GramHashSet = HashSet<GramHash, IdentityBuildHasher>;
pub(crate) type GramHashMap<V> = HashMap<GramHash, V, IdentityBuildHasher>;

/// Emit every trigram plus longer sparse n-grams whose endpoint bigram
/// weights dominate all weights inside the span. The predicate only depends
/// on bytes inside the n-gram, so a gram selected from a query is guaranteed
/// to have been selected from every document containing that query.
pub fn hashes_for_document(bytes: &[u8]) -> GramHashSet {
    let mut hashes = GramHashSet::with_capacity_and_hasher(bytes.len(), Default::default());
    for_each_sparse_gram(bytes, |_, hash| {
        hashes.insert(compact_hash(hash));
    });
    hashes
}

/// Pick one highly selective gram from a literal. Returning a single longer
/// gram keeps index lookups cheap; hash collisions can only add candidates.
pub fn best_hash(literal: &[u8]) -> Option<GramHash> {
    let mut best = None;
    for_each_sparse_gram(literal, |gram, hash| {
        if best.is_none_or(|(length, _)| gram.len() >= length) {
            best = Some((gram.len(), hash));
        }
    });
    best.map(|(_, hash)| compact_hash(hash))
}

fn compact_hash(hash: u64) -> GramHash {
    hash as u32 ^ (hash >> 32) as u32
}

fn for_each_sparse_gram<'a>(bytes: &'a [u8], mut emit: impl FnMut(&'a [u8], u64)) {
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
                emit(&bytes[start..start + len], finish_hash(hash_state, len));
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
    fn chosen_query_gram_is_always_in_document_grams() {
        let query = b"MAX_FILE_SIZE";
        let document = b"prefix MAX_FILE_SIZE suffix";
        let selected = best_hash(query).unwrap();
        assert!(hashes_for_document(document).contains(&selected));
    }

    #[test]
    fn short_literals_have_no_gram() {
        assert_eq!(best_hash(b"ab"), None);
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
            for_each_sparse_gram(bytes, |gram, gram_hash| {
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
