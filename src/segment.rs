use std::{
    fs::{self, File},
    io::{BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use anyhow::{Result, bail};
use memmap2::Mmap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ngram;

const LOOKUP_MAGIC: &[u8; 8] = b"CODERGL4";
const POSTINGS_MAGIC: &[u8; 8] = b"CODERGP4";
const HEADER_SIZE: usize = 32;
const BLOCK_SIZE: usize = 128;
const BLOCK_ENTRY_SIZE: usize = 16;
const MAX_OFFSET: u64 = (1_u64 << 40) - 1;

struct BlockDescriptor {
    first_key: ngram::GramHash,
    stream_offset: u64,
    postings_base: u64,
    entry_count: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SegmentMeta {
    pub id: u64,
    pub lookup: PathBuf,
    pub postings: PathBuf,
    pub ngrams: u64,
}

pub struct Segment {
    pub meta: SegmentMeta,
    lookup: Mmap,
    postings: Mmap,
}

pub fn write(
    index_dir: &Path,
    id: u64,
    mut postings: Vec<(ngram::GramHash, u32)>,
) -> Result<SegmentMeta> {
    // Sorting eight-byte (key, document) records avoids a hash table and a
    // separate allocation for every key. The unstable sort works in place.
    postings.par_sort_unstable();
    postings.dedup();
    let ngrams = postings.chunk_by(|a, b| a.0 == b.0).count();
    let segments_dir = index_dir.join("segments");
    fs::create_dir_all(&segments_dir)?;
    let lookup_name = PathBuf::from(format!("segments/{id:020}.lookup"));
    let postings_name = PathBuf::from(format!("segments/{id:020}.postings"));
    let lookup_path = index_dir.join(&lookup_name);
    let postings_path = index_dir.join(&postings_name);
    let lookup_tmp = lookup_path.with_extension("lookup.tmp");
    let postings_tmp = postings_path.with_extension("postings.tmp");

    let block_count = ngrams.div_ceil(BLOCK_SIZE);
    let directory_len = block_count
        .checked_mul(BLOCK_ENTRY_SIZE)
        .ok_or_else(|| anyhow::anyhow!("too many lookup blocks"))?;
    let mut stream_offset = HEADER_SIZE
        .checked_add(directory_len)
        .ok_or_else(|| anyhow::anyhow!("lookup segment is too large"))?
        as u64;
    let mut lookup_file = BufWriter::new(File::create(&lookup_tmp)?);
    lookup_file.write_all(LOOKUP_MAGIC)?;
    lookup_file.write_all(&(ngrams as u64).to_le_bytes())?;
    lookup_file.write_all(&(block_count as u64).to_le_bytes())?;
    lookup_file.write_all(&(BLOCK_SIZE as u64).to_le_bytes())?;
    lookup_file.write_all(&vec![0_u8; directory_len])?;
    let mut postings_file = BufWriter::new(File::create(&postings_tmp)?);
    postings_file.write_all(POSTINGS_MAGIC)?;
    let mut postings_offset = POSTINGS_MAGIC.len() as u64;
    let mut directory = Vec::with_capacity(block_count);

    let mut groups = postings.chunk_by(|a, b| a.0 == b.0);
    while let Some(first) = groups.next() {
        let entry_count = (ngrams - directory.len() * BLOCK_SIZE).min(BLOCK_SIZE);
        let mut encoded_block = Vec::with_capacity(entry_count * 4);
        directory.push(BlockDescriptor {
            first_key: first[0].0,
            stream_offset,
            postings_base: postings_offset,
            entry_count: entry_count as u16,
        });
        let mut previous_key = first[0].0;
        for (position, list) in std::iter::once(first)
            .chain(groups.by_ref().take(BLOCK_SIZE - 1))
            .enumerate()
        {
            let hash = list[0].0;
            if position > 0 {
                write_varint(u64::from(hash - previous_key), &mut encoded_block);
                previous_key = hash;
            }
            if list.len() == 1 {
                write_varint(u64::from(list[0].1) << 1, &mut encoded_block);
                continue;
            }
            let mut encoded_postings = Vec::with_capacity(list.len() * 2);
            let mut previous = 0_u32;
            for (list_position, &(_, id)) in list.iter().enumerate() {
                let delta = if list_position == 0 {
                    id
                } else {
                    id - previous
                };
                write_varint(u64::from(delta), &mut encoded_postings);
                previous = id;
            }
            let encoded_len = encoded_postings.len() as u64;
            write_varint((encoded_len << 1) | 1, &mut encoded_block);
            postings_file.write_all(&encoded_postings)?;
            postings_offset += encoded_len;
        }
        lookup_file.write_all(&encoded_block)?;
        stream_offset += encoded_block.len() as u64;
    }
    lookup_file.seek(SeekFrom::Start(HEADER_SIZE as u64))?;
    for block in directory {
        lookup_file.write_all(&block.first_key.to_le_bytes())?;
        write_u40(block.stream_offset, &mut lookup_file)?;
        write_u40(block.postings_base, &mut lookup_file)?;
        lookup_file.write_all(&block.entry_count.to_le_bytes())?;
    }
    lookup_file.flush()?;
    postings_file.flush()?;
    replace(lookup_tmp, lookup_path)?;
    replace(postings_tmp, postings_path)?;
    Ok(SegmentMeta {
        id,
        lookup: lookup_name,
        postings: postings_name,
        ngrams: ngrams as u64,
    })
}

pub fn load(index_dir: &Path, meta: &SegmentMeta) -> Result<Segment> {
    let lookup_file = File::open(index_dir.join(&meta.lookup))?;
    let postings_file = File::open(index_dir.join(&meta.postings))?;
    // SAFETY: published segment files are immutable.
    let lookup = unsafe { Mmap::map(&lookup_file)? };
    // SAFETY: published segment files are immutable.
    let postings = unsafe { Mmap::map(&postings_file)? };
    validate(&lookup, &postings)?;
    Ok(Segment {
        meta: meta.clone(),
        lookup,
        postings,
    })
}

impl Segment {
    pub fn postings(&self, hash: ngram::GramHash) -> Result<Vec<u32>> {
        let block_count = read_u64(&self.lookup[16..24]) as usize;
        let mut low = 0;
        let mut high = block_count;
        while low < high {
            let mid = low + (high - low) / 2;
            if block_descriptor(&self.lookup, mid).first_key <= hash {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        if low == 0 {
            return Ok(Vec::new());
        }
        self.postings_in_block(low - 1, block_count, hash)
    }

    fn postings_in_block(
        &self,
        block_index: usize,
        block_count: usize,
        hash: ngram::GramHash,
    ) -> Result<Vec<u32>> {
        let block = block_descriptor(&self.lookup, block_index);
        let stream_end = if block_index + 1 == block_count {
            self.lookup.len()
        } else {
            block_descriptor(&self.lookup, block_index + 1).stream_offset as usize
        };
        let stream_offset = block.stream_offset as usize;
        if stream_offset > stream_end || stream_end > self.lookup.len() {
            bail!("invalid lookup block offset; rebuild the index");
        }
        let mut stream = &self.lookup[stream_offset..stream_end];
        let mut key = block.first_key;
        let mut postings_offset = block.postings_base as usize;
        if postings_offset < POSTINGS_MAGIC.len() || postings_offset > self.postings.len() {
            bail!("invalid posting block offset; rebuild the index");
        }
        for position in 0..usize::from(block.entry_count) {
            if position > 0 {
                let delta = u32::try_from(read_varint(&mut stream)?)
                    .map_err(|_| anyhow::anyhow!("invalid key delta; rebuild the index"))?;
                key = key
                    .checked_add(delta)
                    .ok_or_else(|| anyhow::anyhow!("key overflow; rebuild the index"))?;
            }
            if key > hash {
                return Ok(Vec::new());
            }
            let descriptor = read_varint(&mut stream)?;
            if descriptor & 1 == 0 {
                if key == hash {
                    let id = u32::try_from(descriptor >> 1).map_err(|_| {
                        anyhow::anyhow!("invalid inline document ID; rebuild the index")
                    })?;
                    return Ok(vec![id]);
                }
                continue;
            }
            let len = usize::try_from(descriptor >> 1)
                .map_err(|_| anyhow::anyhow!("invalid posting length; rebuild the index"))?;
            let end = postings_offset
                .checked_add(len)
                .filter(|&end| end <= self.postings.len())
                .ok_or_else(|| anyhow::anyhow!("invalid posting offset; rebuild the index"))?;
            if key == hash {
                return decode_postings(&self.postings[postings_offset..end]);
            }
            postings_offset = end;
        }
        Ok(Vec::new())
    }
}

fn validate(lookup: &[u8], postings: &[u8]) -> Result<()> {
    if lookup.len() < HEADER_SIZE || &lookup[..8] != LOOKUP_MAGIC {
        bail!("invalid lookup segment; rebuild the index");
    }
    if postings.len() < 8 || &postings[..8] != POSTINGS_MAGIC {
        bail!("invalid postings segment; rebuild the index");
    }
    let key_count = read_u64(&lookup[8..16]) as usize;
    let block_count = read_u64(&lookup[16..24]) as usize;
    if read_u64(&lookup[24..32]) as usize != BLOCK_SIZE
        || block_count != key_count.div_ceil(BLOCK_SIZE)
    {
        bail!("invalid lookup block header; rebuild the index");
    }
    let stream_start = block_count
        .checked_mul(BLOCK_ENTRY_SIZE)
        .and_then(|size| size.checked_add(HEADER_SIZE))
        .filter(|&size| size <= lookup.len())
        .ok_or_else(|| anyhow::anyhow!("truncated lookup directory; rebuild the index"))?;
    if block_count > 0 {
        let first = block_descriptor(lookup, 0);
        if first.stream_offset as usize != stream_start
            || first.postings_base as usize != POSTINGS_MAGIC.len()
        {
            bail!("invalid first lookup block; rebuild the index");
        }
        let last = block_descriptor(lookup, block_count - 1);
        let expected_last_count = key_count - (block_count - 1) * BLOCK_SIZE;
        if usize::from(last.entry_count) != expected_last_count {
            bail!("invalid last lookup block; rebuild the index");
        }
    } else if lookup.len() != HEADER_SIZE {
        bail!("invalid empty lookup segment; rebuild the index");
    }
    Ok(())
}

fn block_descriptor(lookup: &[u8], index: usize) -> BlockDescriptor {
    let entry = HEADER_SIZE + index * BLOCK_ENTRY_SIZE;
    BlockDescriptor {
        first_key: read_u32(&lookup[entry..entry + 4]),
        stream_offset: read_u40(&lookup[entry + 4..entry + 9]),
        postings_base: read_u40(&lookup[entry + 9..entry + 14]),
        entry_count: u16::from_le_bytes(lookup[entry + 14..entry + 16].try_into().unwrap()),
    }
}

fn replace(from: PathBuf, to: PathBuf) -> Result<()> {
    if cfg!(windows) && to.exists() {
        fs::remove_file(&to)?;
    }
    fs::rename(from, to)?;
    Ok(())
}

fn read_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes.try_into().unwrap())
}

fn read_u64(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes.try_into().unwrap())
}

fn read_u40(bytes: &[u8]) -> u64 {
    let mut value = [0_u8; 8];
    value[..5].copy_from_slice(bytes);
    u64::from_le_bytes(value)
}

fn write_u40(value: u64, output: &mut impl Write) -> Result<()> {
    if value > MAX_OFFSET {
        bail!("segment exceeds the 1 TiB format limit");
    }
    output.write_all(&value.to_le_bytes()[..5])?;
    Ok(())
}

fn write_varint(mut value: u64, output: &mut Vec<u8>) {
    while value >= 0x80 {
        output.push((value as u8) | 0x80);
        value >>= 7;
    }
    output.push(value as u8);
}

fn read_varint(bytes: &mut &[u8]) -> Result<u64> {
    let mut value = 0_u64;
    let mut shift = 0;
    loop {
        let Some((&byte, rest)) = bytes.split_first() else {
            bail!("truncated varint; rebuild the index");
        };
        *bytes = rest;
        let part = u64::from(byte & 0x7f);
        if shift >= 64 || (shift == 63 && part > 1) {
            bail!("invalid varint; rebuild the index");
        }
        value |= part << shift;
        if byte & 0x80 == 0 {
            return Ok(value);
        }
        shift += 7;
    }
}

fn decode_postings(mut bytes: &[u8]) -> Result<Vec<u32>> {
    let mut result = Vec::with_capacity(bytes.len());
    let mut previous = 0_u32;
    while !bytes.is_empty() {
        let value = u32::try_from(read_varint(&mut bytes)?)
            .map_err(|_| anyhow::anyhow!("invalid posting delta; rebuild the index"))?;
        let id = if result.is_empty() {
            value
        } else {
            previous
                .checked_add(value)
                .ok_or_else(|| anyhow::anyhow!("posting ID overflow; rebuild the index"))?
        };
        result.push(id);
        previous = id;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn posting_varints_round_trip() {
        let ids = [0, 1, 127, 128, 16_384];
        let mut bytes = Vec::new();
        let mut previous = 0;
        for (position, &id) in ids.iter().enumerate() {
            write_varint(
                u64::from(if position == 0 { id } else { id - previous }),
                &mut bytes,
            );
            previous = id;
        }
        assert_eq!(decode_postings(&bytes).unwrap(), ids);
    }

    #[test]
    fn block_lookup_round_trips_inline_and_external_postings() {
        let directory = tempfile::tempdir().unwrap();
        let mut source = Vec::new();
        for key in 0..300_u32 {
            let ids = if key % 3 == 0 {
                vec![key % 17]
            } else {
                vec![key % 11, 100 + key % 13, 300 + key]
            };
            source.extend(ids.into_iter().map(|id| (key * 10, id)));
        }
        source.extend([(0, 0), (1_280, 111), (u32::MAX, u32::MAX), (u32::MAX, 0)]);
        source.reverse();
        let meta = write(directory.path(), 1, source).unwrap();
        assert_eq!(meta.ngrams, 301);
        let segment = load(directory.path(), &meta).unwrap();
        assert_eq!(segment.postings(0).unwrap(), vec![0]);
        assert_eq!(segment.postings(1_280).unwrap(), vec![7, 111, 428]);
        assert_eq!(segment.postings(2_970).unwrap(), vec![8]);
        assert!(segment.postings(1_281).unwrap().is_empty());
        assert!(segment.postings(4_000).unwrap().is_empty());
        assert_eq!(segment.postings(u32::MAX).unwrap(), vec![0, u32::MAX]);
    }

    #[test]
    fn empty_segment_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let meta = write(directory.path(), 1, Vec::new()).unwrap();
        assert_eq!(meta.ngrams, 0);
        let segment = load(directory.path(), &meta).unwrap();
        assert!(segment.postings(0).unwrap().is_empty());
        assert!(segment.postings(u32::MAX).unwrap().is_empty());
    }
}
