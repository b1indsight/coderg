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

// Chromium ASCII all letter counts; fixed normalized frequencies per 100000.
// Derived from counts.json; no document-context-dependent query weights.
const LETTER_FREQUENCIES: [u32; 26] = [8249, 2023, 4433, 4095, 11608, 2151, 1970, 2157, 6949, 237, 981, 4519, 2699, 6638, 6468, 3040, 299, 6755, 6744, 9332, 3099, 1269, 1101, 1740, 1185, 259];
const COMMON_FREQUENCY: u32 = 11608;

// Empirical Chromium pair probabilities scaled by 100000^2, row-major a..z.
const PAIR_FREQUENCIES: [u32; 26 * 26] = [
    90354411, 31773357, 53546270, 32865152, 3441372, 7155351, 33891169, 1823585, 23803073, 1275329, 9408035, 81859874, 48029770,
    118927363, 1077086, 26234107, 1384727, 70971891, 67195045, 165828978, 19782430, 11652056, 4145619, 4331095, 16263626, 1484849,
    33950105, 3218639, 6028949, 3067333, 22086068, 2093922, 1643698, 572429, 11000961, 3420097, 693799, 29491836, 1461283,
    950162, 19864291, 890080, 503201, 14870069, 11175678, 1246215, 22642757, 1105769, 405255, 312474, 11534070, 241802,
    46167883, 1857512, 11329296, 4058513, 61459925, 2033409, 1044319, 55657449, 8682877, 342902, 31561046, 31122902, 1861483,
    685171, 109103954, 4985871, 584792, 26981011, 7817394, 58376467, 15984378, 589909, 671055, 359889, 6446938, 350041,
    36283281, 4954128, 6239831, 11604338, 103643855, 4807487, 3353221, 1086382, 51589822, 549288, 700709, 8519505, 2531925,
    1918410, 24989065, 3376315, 754427, 11637759, 11155772, 6776743, 8338849, 1603104, 1656742, 707942, 3736987, 342902,
    44835345, 14959412, 63982978, 79396396, 16137949, 27636623, 12651406, 2990190, 11452593, 1158051, 2542407, 52139616, 34886502,
    126453537, 7191226, 16127190, 15908287, 186374713, 134930022, 91095598, 4101797, 19067181, 17142914, 46418314, 9277167, 1176123,
    22879153, 1295174, 2037912, 2788833, 19066184, 15607299, 1054403, 424474, 42278829, 209520, 320327, 7939768, 689829,
    2488844, 37358554, 1020085, 295918, 15764032, 4470638, 5112741, 7771334, 304250, 364878, 1725081, 2463525, 156638,
    11978315, 2388182, 2624511, 1270246, 62582249, 2363403, 4620225, 8605546, 13827192, 341251, 437389, 10174076, 2357646,
    7836168, 9321590, 2674582, 409651, 15530898, 8966116, 4381415, 6974514, 836194, 646856, 200912, 636785, 313505,
    34786999, 950809, 1176433, 1173353, 67694060, 685926, 399020, 872298, 27314846, 346832, 400314, 1012879, 2265801,
    756462, 21397655, 1165884, 337328, 20976302, 2162531, 20386987, 2495841, 403179, 594371, 147410, 829824, 300037,
    21241138, 10426738, 54593373, 72074322, 30042372, 26090155, 25742359, 856443, 1541500, 1218874, 2304420, 44153827, 32102845,
    175025349, 109365063, 15795640, 3115295, 15514801, 64679521, 65590633, 5704571, 22639549, 518792, 4561431, 394733, 14282452,
    3451806, 386206, 281311, 232547, 5333330, 156908, 190268, 161094, 841539, 409442, 401885, 277367, 205799,
    615288, 5346920, 392684, 334618, 379343, 3753744, 304540, 1702371, 296963, 241964, 149735, 228833, 151448,
    6150002, 1526076, 2451216, 1874250, 28036088, 1266640, 2838460, 1242474, 7425075, 274772, 655221, 1878295, 2734138,
    2408910, 2590658, 1994946, 379728, 1919731, 6864684, 2924487, 3116009, 678882, 1097248, 173039, 632073, 183662,
    83052222, 7354107, 2990231, 18569259, 104783416, 6173305, 1226788, 681578, 65803517, 591580, 1340547, 45588983, 1661487,
    1054518, 43114059, 6096601, 303866, 1658609, 19683979, 18476922, 29879862, 2013571, 1403716, 210087, 7979331, 279430,
    50073453, 6214391, 1494340, 2414809, 96115868, 860784, 1026738, 486517, 22098795, 345545, 335096, 8401020, 8698267,
    2486080, 31506688, 27561664, 262369, 873619, 7690302, 2032614, 6563522, 808239, 655086, 187484, 878789, 209776,
    53827305, 2199848, 40769733, 64119576, 55408778, 12934867, 58796802, 1680503, 26236325, 1119843, 9745329, 6778313, 3118449,
    10819678, 28874668, 6763322, 466193, 3256597, 101138672, 119440392, 16590876, 6922548, 1256589, 273228, 3061765, 464218,
    10379275, 11743314, 26465056, 28631645, 2004261, 20231292, 15259180, 718619, 15699225, 4621755, 8542289, 34968579, 51868989,
    196162745, 23331582, 27611924, 357011, 109551752, 22805745, 29023473, 45053244, 17330789, 32465195, 4510672, 1304908, 747558,
    51110498, 4193459, 2492787, 5233275, 58770169, 735235, 591338, 9709251, 10179597, 277987, 850720, 24484846, 1430356,
    1237196, 35369507, 15290039, 362498, 44839126, 9046123, 29128024, 19499016, 566585, 668393, 4069245, 3915910, 162051,
    1197047, 363253, 533454, 301810, 596993, 339956, 499985, 271786, 715997, 271678, 326974, 491209, 216578,
    330445, 284782, 219126, 1063274, 604685, 375387, 232048, 19019139, 188253, 295210, 204417, 292528, 177986,
    96294259, 3317164, 20951152, 16579794, 156204473, 6623071, 12324258, 1326431, 68985587, 294948, 8580598, 12770707, 14358179,
    21650701, 91819994, 4612682, 559514, 19458955, 22934159, 42942766, 22311935, 13667292, 2316641, 332110, 17568372, 450500,
    20414901, 1938416, 24415887, 3205919, 128272890, 3563490, 2228174, 22538577, 50326310, 288395, 8466036, 43506857, 3418641,
    2532835, 22487205, 27536413, 974274, 5134042, 49348511, 164289299, 24945714, 2320793, 4829118, 241149, 8499552, 639346,
    86861842, 3633932, 15418608, 17002953, 169563349, 7564112, 1318235, 87399199, 149222289, 415805, 915554, 8529158, 10349946,
    2845538, 70218266, 13980345, 381945, 109122566, 32628311, 30036514, 32130368, 2682031, 6118374, 1530586, 35540024, 651123,
    8478365, 14966800, 7594278, 16650620, 34911301, 5223069, 4476577, 318622, 17838472, 326556, 1315383, 31366370, 20315243,
    43881644, 867566, 18243175, 202045, 54040000, 29479575, 42935412, 689114, 699482, 263805, 905625, 360691, 721255,
    28083598, 451768, 677446, 334382, 50590957, 330715, 1144050, 290310, 36836054, 165145, 420800, 530703, 508216,
    302612, 10248934, 832985, 209237, 469611, 622251, 824026, 573656, 451013, 304277, 279700, 341952, 227700,
    13625606, 799355, 1331137, 757089, 16291365, 578503, 745819, 6094181, 24524301, 192365, 277307, 749391, 708852,
    6508091, 9391499, 1392984, 307162, 4946302, 13534645, 1034369, 333566, 340819, 3436080, 293141, 383691, 177508,
    8983763, 9333285, 5551121, 6929700, 11729320, 7682887, 237926, 933728, 2522622, 173409, 209021, 427460, 1669792,
    483767, 429253, 17338049, 230377, 627523, 689903, 20291677, 228610, 329016, 198573, 2557702, 1540846, 265928,
    2705145, 2253553, 2408290, 1476463, 5210295, 1081960, 565021, 509328, 2687127, 187552, 435947, 7420646, 3183283,
    5957496, 4309160, 18528921, 193207, 2606182, 8260298, 7276377, 709505, 537074, 874280, 205449, 458994, 498293,
    3098503, 271563, 218735, 316505, 13698442, 151569, 247937, 277779, 1792058, 141033, 238917, 332636, 410493,
    303380, 1460359, 217704, 192823, 177501, 252702, 284897, 423813, 222240, 428026, 265908, 481037, 757763,
];
const MAX_PAIR_FREQUENCY: u32 = 196162745;

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
    let [left, right] = value.to_le_bytes();
    let rarity = MAX_PAIR_FREQUENCY - pair_frequency(left, right);
    let tie_break = mix(u64::from(value) ^ 0x9e37_79b9_7f4a_7c15) & u64::from(u32::MAX);
    (u64::from(rarity) << 32) | tie_break
}

fn pair_frequency(left: u8, right: u8) -> u32 {
    let a = left.to_ascii_lowercase();
    let b = right.to_ascii_lowercase();
    if a.is_ascii_lowercase() && b.is_ascii_lowercase() {
        PAIR_FREQUENCIES[usize::from(a - b'a') * 26 + usize::from(b - b'a')]
    } else {
        // Preserve the previous unigram estimate for nonletter/mixed pairs.
        byte_frequency(left) * byte_frequency(right)
    }
}

fn byte_frequency(byte: u8) -> u32 {
    let lower = byte.to_ascii_lowercase();
    if lower.is_ascii_lowercase() {
        LETTER_FREQUENCIES[usize::from(lower - b'a')]
    } else {
        // Do not treat punctuation, digits, or individual UTF-8 bytes as rare
        // letters. Nonletter-only pairs retain their hash-based ordering.
        COMMON_FREQUENCY
    }
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
    fn empirical_pair_order_and_case_folding_are_preserved() {
        assert!(pair_frequency(b'q', b'u') > pair_frequency(b'u', b'q'));
        assert!(pair_weight(b"qu") < pair_weight(b"uq"));
        for left in b'a'..=b'z' {
            for right in b'a'..=b'z' {
                let expected = pair_weight(&[left, right]) >> 32;
                for a in [left, left.to_ascii_uppercase()] {
                    for b in [right, right.to_ascii_uppercase()] {
                        assert_eq!(pair_weight(&[a, b]) >> 32, expected);
                    }
                }
            }
        }
    }

    #[test]
    fn nonletter_fallback_keeps_the_unigram_scale_and_order() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                if !left.is_ascii_alphabetic() || !right.is_ascii_alphabetic() {
                    assert_eq!(pair_frequency(left, right), byte_frequency(left) * byte_frequency(right));
                }
            }
        }
    }

    #[test]
    fn unigram_fallback_folds_ascii_case_and_assigns_common_nonletters() {
        for letter in b'a'..=b'z' {
            assert_eq!(
                byte_frequency(letter),
                byte_frequency(letter.to_ascii_uppercase())
            );
            assert_eq!(
                pair_weight(&[letter, b'_']) >> 32,
                pair_weight(&[letter.to_ascii_uppercase(), b'_']) >> 32
            );
        }
        for byte in 0..=u8::MAX {
            if !byte.is_ascii_alphabetic() {
                assert_eq!(pair_weight(&[byte, b'_']) >> 32, u64::from(MAX_PAIR_FREQUENCY - COMMON_FREQUENCY * COMMON_FREQUENCY));
            }
        }
    }

    #[test]
    fn pair_weight_table_matches_direct_weights() {
        let table = pair_weight_table();
        for value in 0..=u16::MAX {
            assert_eq!(table[usize::from(value)], pair_weight_value(value));
        }
    }
}
