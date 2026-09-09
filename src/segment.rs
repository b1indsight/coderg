use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

pub use crate::manifest::SegmentMeta;
use anyhow::{Context, Result, bail};
use memmap2::Mmap;
use rayon::prelude::*;

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

pub struct Segment {
    pub meta: SegmentMeta,
    lookup: Mmap,
    postings: Mmap,
}

// Retain the original slice encoder as a byte-for-byte format oracle.
#[cfg(test)]
fn write(
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

/// Encode sorted, unique records without retaining a posting list or the
/// lookup directory in memory. The directory and body use independent cursors.
pub fn write_sorted(
    index_dir: &Path,
    id: u64,
    ngrams: u64,
    records: impl IntoIterator<Item = Result<(ngram::GramHash, u32)>>,
) -> Result<SegmentMeta> {
    const ENCODE_BYTES: usize = 64 * 1024;
    let segments_dir = index_dir.join("segments");
    fs::create_dir_all(&segments_dir)?;
    let temporary = tempfile::Builder::new()
        .prefix(".write-")
        .tempdir_in(&segments_dir)?;
    let lookup_tmp = temporary.path().join("lookup");
    let postings_tmp = temporary.path().join("postings");
    let lookup_name = PathBuf::from(format!("segments/{id:020}.lookup"));
    let postings_name = PathBuf::from(format!("segments/{id:020}.postings"));
    let block_count = ngrams.div_ceil(BLOCK_SIZE as u64);
    let mut stream_offset = block_count
        .checked_mul(BLOCK_ENTRY_SIZE as u64)
        .and_then(|bytes| bytes.checked_add(HEADER_SIZE as u64))
        .context("too many lookup blocks")?;
    let mut lookup_file = BufWriter::new(File::create(&lookup_tmp)?);
    lookup_file.write_all(LOOKUP_MAGIC)?;
    lookup_file.write_all(&ngrams.to_le_bytes())?;
    lookup_file.write_all(&block_count.to_le_bytes())?;
    lookup_file.write_all(&(BLOCK_SIZE as u64).to_le_bytes())?;
    lookup_file.seek(SeekFrom::Start(stream_offset))?;
    let mut directory = BufWriter::new(File::options().write(true).open(&lookup_tmp)?);
    directory.seek(SeekFrom::Start(HEADER_SIZE as u64))?;
    let mut postings_file = BufWriter::new(File::create(&postings_tmp)?);
    postings_file.write_all(POSTINGS_MAGIC)?;
    let mut postings_offset = POSTINGS_MAGIC.len() as u64;
    let mut encoded_postings = Vec::with_capacity(ENCODE_BYTES);
    let mut encoded_block = Vec::with_capacity(BLOCK_SIZE * 15);
    let mut records = records.into_iter();
    let mut next = records.next().transpose()?;

    for block in 0..block_count {
        let first_key = next.context("missing sorted posting records")?.0;
        let entry_count = (ngrams - block * BLOCK_SIZE as u64).min(BLOCK_SIZE as u64);
        directory.write_all(&first_key.to_le_bytes())?;
        write_u40(stream_offset, &mut directory)?;
        write_u40(postings_offset, &mut directory)?;
        directory.write_all(&(entry_count as u16).to_le_bytes())?;
        encoded_block.clear();
        let mut previous_key = first_key;
        for position in 0..entry_count {
            let (hash, first_id) = next.context("missing sorted posting records")?;
            next = records.next().transpose()?;
            if position > 0 {
                write_varint(u64::from(hash - previous_key), &mut encoded_block);
                previous_key = hash;
            }
            if next.is_none_or(|record| record.0 != hash) {
                write_varint(u64::from(first_id) << 1, &mut encoded_block);
                continue;
            }
            encoded_postings.clear();
            write_varint(u64::from(first_id), &mut encoded_postings);
            let mut encoded_len = 0;
            let mut previous_id = first_id;
            while let Some((key, doc_id)) = next {
                if key != hash {
                    break;
                }
                write_varint(u64::from(doc_id - previous_id), &mut encoded_postings);
                previous_id = doc_id;
                next = records.next().transpose()?;
                if encoded_postings.len() >= ENCODE_BYTES - 5 {
                    postings_file.write_all(&encoded_postings)?;
                    encoded_len += encoded_postings.len() as u64;
                    encoded_postings.clear();
                }
            }
            postings_file.write_all(&encoded_postings)?;
            encoded_len += encoded_postings.len() as u64;
            postings_offset += encoded_len;
            write_varint((encoded_len << 1) | 1, &mut encoded_block);
        }
        lookup_file.write_all(&encoded_block)?;
        stream_offset += encoded_block.len() as u64;
    }
    if next.is_some() {
        bail!("unexpected extra sorted posting records");
    }
    directory.flush()?;
    lookup_file.flush()?;
    postings_file.flush()?;
    drop((directory, lookup_file, postings_file));
    replace(lookup_tmp, index_dir.join(&lookup_name))?;
    replace(postings_tmp, index_dir.join(&postings_name))?;
    Ok(SegmentMeta {
        id,
        lookup: lookup_name,
        postings: postings_name,
        ngrams,
    })
}

const PART_IO_BYTES: usize = 64 * 1024;

struct EncodedPart {
    postings: PathBuf,
    keys: PathBuf,
    ngrams: u64,
    postings_bytes: u64,
}

/// Encode disjoint, increasing gram ranges in parallel. Complete postings
/// first, then assemble the global lookup blocks from per-gram metadata.
/// Returns scratch bytes written, excluding the final index output files.
pub fn write_partitioned<R>(
    index_dir: &Path,
    id: u64,
    partition_count: usize,
    records: impl Fn(usize) -> Result<R> + Sync,
) -> Result<(SegmentMeta, u64)>
where
    R: IntoIterator<Item = Result<(ngram::GramHash, u32)>>,
{
    let segments_dir = index_dir.join("segments");
    fs::create_dir_all(&segments_dir)?;
    let temporary = tempfile::Builder::new()
        .prefix(".write-")
        .tempdir_in(&segments_dir)?;
    let parts = (0..partition_count)
        .into_par_iter()
        .map(|part| encode_postings_part(temporary.path(), part, records(part)?))
        .collect::<Result<Vec<_>>>()?;
    let ngrams = parts.iter().map(|part| part.ngrams).sum::<u64>();
    let scratch_bytes = parts
        .iter()
        .map(|part| part.postings_bytes + part.ngrams * 12)
        .sum();
    let postings_tmp = temporary.path().join("postings");
    let postings_bytes = join_postings(&postings_tmp, &parts)?;
    let lookup_tmp = temporary.path().join("lookup");
    assemble_lookup(&lookup_tmp, ngrams, postings_bytes, &parts)?;
    let lookup_name = PathBuf::from(format!("segments/{id:020}.lookup"));
    let postings_name = PathBuf::from(format!("segments/{id:020}.postings"));
    replace(lookup_tmp, index_dir.join(&lookup_name))?;
    replace(postings_tmp, index_dir.join(&postings_name))?;
    Ok((
        SegmentMeta {
            id,
            lookup: lookup_name,
            postings: postings_name,
            ngrams,
        },
        scratch_bytes,
    ))
}

fn encode_postings_part(
    directory: &Path,
    part: usize,
    records: impl IntoIterator<Item = Result<(ngram::GramHash, u32)>>,
) -> Result<EncodedPart> {
    let postings = directory.join(format!("{part}.postings"));
    let keys = directory.join(format!("{part}.keys"));
    let mut postings_file = BufWriter::with_capacity(PART_IO_BYTES, File::create(&postings)?);
    let mut keys_file = BufWriter::with_capacity(PART_IO_BYTES, File::create(&keys)?);
    let mut encoded = Vec::with_capacity(PART_IO_BYTES);
    let mut records = records.into_iter();
    let mut next = records.next().transpose()?;
    let mut ngrams = 0;
    let mut postings_bytes = 0;
    while let Some((hash, first_id)) = next {
        next = records.next().transpose()?;
        let value = if next.is_none_or(|record| record.0 != hash) {
            u64::from(first_id) << 1
        } else {
            encoded.clear();
            write_varint(u64::from(first_id), &mut encoded);
            let mut encoded_len = 0;
            let mut previous_id = first_id;
            while let Some((key, doc_id)) = next {
                if key != hash {
                    break;
                }
                write_varint(u64::from(doc_id - previous_id), &mut encoded);
                previous_id = doc_id;
                next = records.next().transpose()?;
                if encoded.len() >= PART_IO_BYTES - 5 {
                    postings_file.write_all(&encoded)?;
                    encoded_len += encoded.len() as u64;
                    encoded.clear();
                }
            }
            postings_file.write_all(&encoded)?;
            encoded_len += encoded.len() as u64;
            postings_bytes += encoded_len;
            (encoded_len << 1) | 1
        };
        // Keep only key + inline ID / encoded length. No raw postings are
        // retained, and metadata buffering stays bounded regardless of keys.
        let mut entry = [0; 12];
        entry[..4].copy_from_slice(&hash.to_le_bytes());
        entry[4..].copy_from_slice(&value.to_le_bytes());
        keys_file.write_all(&entry)?;
        ngrams += 1;
    }
    postings_file.flush()?;
    keys_file.flush()?;
    Ok(EncodedPart {
        postings,
        keys,
        ngrams,
        postings_bytes,
    })
}

fn join_postings(path: &Path, parts: &[EncodedPart]) -> Result<u64> {
    let mut output = BufWriter::with_capacity(PART_IO_BYTES, File::create(path)?);
    output.write_all(POSTINGS_MAGIC)?;
    let mut bytes = POSTINGS_MAGIC.len() as u64;
    for part in parts {
        let mut input = BufReader::with_capacity(PART_IO_BYTES, File::open(&part.postings)?);
        let copied = io::copy(&mut input, &mut output)?;
        if copied != part.postings_bytes {
            bail!("truncated encoded postings fragment");
        }
        bytes += copied;
    }
    output.flush()?;
    Ok(bytes)
}

struct EncodedKeys {
    input: BufReader<File>,
    remaining: u64,
}

impl EncodedKeys {
    fn new(part: &EncodedPart) -> Result<Self> {
        let input = File::open(&part.keys)?;
        if input.metadata()?.len() != part.ngrams * 12 {
            bail!("truncated encoded key metadata");
        }
        Ok(Self {
            input: BufReader::with_capacity(PART_IO_BYTES, input),
            remaining: part.ngrams,
        })
    }
}

impl Iterator for EncodedKeys {
    type Item = Result<(ngram::GramHash, u64)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let mut entry = [0; 12];
        Some(
            self.input
                .read_exact(&mut entry)
                .map(|()| {
                    (
                        u32::from_le_bytes(entry[..4].try_into().unwrap()),
                        u64::from_le_bytes(entry[4..].try_into().unwrap()),
                    )
                })
                .map_err(Into::into),
        )
    }
}

fn assemble_lookup(
    path: &Path,
    ngrams: u64,
    postings_bytes: u64,
    parts: &[EncodedPart],
) -> Result<()> {
    let block_count = ngrams.div_ceil(BLOCK_SIZE as u64);
    let mut stream_offset = block_count
        .checked_mul(BLOCK_ENTRY_SIZE as u64)
        .and_then(|bytes| bytes.checked_add(HEADER_SIZE as u64))
        .context("too many lookup blocks")?;
    let mut body = BufWriter::with_capacity(PART_IO_BYTES, File::create(path)?);
    body.write_all(LOOKUP_MAGIC)?;
    body.write_all(&ngrams.to_le_bytes())?;
    body.write_all(&block_count.to_le_bytes())?;
    body.write_all(&(BLOCK_SIZE as u64).to_le_bytes())?;
    body.seek(SeekFrom::Start(stream_offset))?;
    let mut directory =
        BufWriter::with_capacity(PART_IO_BYTES, File::options().write(true).open(path)?);
    directory.seek(SeekFrom::Start(HEADER_SIZE as u64))?;
    let readers = parts
        .iter()
        .map(EncodedKeys::new)
        .collect::<Result<Vec<_>>>()?;
    let mut keys = readers.into_iter().flatten();
    let mut postings_offset = POSTINGS_MAGIC.len() as u64;
    let mut previous_key = None;
    let mut encoded = Vec::with_capacity(BLOCK_SIZE * 15);
    for block in 0..block_count {
        let entry_count = (ngrams - block * BLOCK_SIZE as u64).min(BLOCK_SIZE as u64);
        encoded.clear();
        for position in 0..entry_count {
            let (hash, value) = keys.next().context("missing encoded key metadata")??;
            if previous_key.is_some_and(|previous| hash <= previous) {
                bail!("encoded gram partitions overlap or are out of order");
            }
            if position == 0 {
                directory.write_all(&hash.to_le_bytes())?;
                write_u40(stream_offset, &mut directory)?;
                write_u40(postings_offset, &mut directory)?;
                directory.write_all(&(entry_count as u16).to_le_bytes())?;
            } else {
                write_varint(u64::from(hash - previous_key.unwrap()), &mut encoded);
            }
            write_varint(value, &mut encoded);
            if value & 1 == 1 {
                postings_offset += value >> 1;
            }
            previous_key = Some(hash);
        }
        body.write_all(&encoded)?;
        stream_offset += encoded.len() as u64;
    }
    if keys.next().is_some() || postings_offset != postings_bytes {
        bail!("encoded key metadata does not match postings");
    }
    directory.flush()?;
    body.flush()?;
    Ok(())
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
    fn streaming_encoder_matches_original_format_with_long_postings() {
        let directory = tempfile::tempdir().unwrap();
        for mut source in [
            Vec::new(),
            vec![(u32::MAX, u32::MAX)],
            (0..127).map(|key| (key, key)).collect(),
            (0..128).map(|key| (key, key)).collect(),
            (0..129).map(|key| (key, key)).collect(),
            {
                let mut records: Vec<_> = (0..260).map(|key| (key * 17, key * 128)).collect();
                records.extend((0..100_000).map(|id| (128 * 17, id * 127)));
                records.extend([(0, u32::MAX), (u32::MAX, 0), (u32::MAX, u32::MAX)]);
                records
            },
        ] {
            let expected = write(directory.path(), 1, source.clone()).unwrap();
            source.sort_unstable();
            source.dedup();
            let actual = write_sorted(
                directory.path(),
                2,
                expected.ngrams,
                source.iter().copied().map(Ok),
            )
            .unwrap();
            assert_eq!(
                fs::read(directory.path().join(actual.lookup)).unwrap(),
                fs::read(directory.path().join(&expected.lookup)).unwrap()
            );
            assert_eq!(
                fs::read(directory.path().join(actual.postings)).unwrap(),
                fs::read(directory.path().join(&expected.postings)).unwrap()
            );
            // Deliberately cut inside global 128-key lookup blocks. A shard
            // boundary must not introduce an extra partial lookup block.
            let cuts = [
                0,
                source.partition_point(|record| record.0 < 31 * 17),
                source.partition_point(|record| record.0 < 129 * 17),
                source.partition_point(|record| record.0 < 190 * 17),
                source.len(),
            ];
            let (parallel, _) = write_partitioned(directory.path(), 3, 4, |part| {
                Ok(source[cuts[part]..cuts[part + 1]].iter().copied().map(Ok))
            })
            .unwrap();
            assert_eq!(parallel.ngrams, expected.ngrams);
            assert_eq!(
                fs::read(directory.path().join(parallel.lookup)).unwrap(),
                fs::read(directory.path().join(&expected.lookup)).unwrap()
            );
            assert_eq!(
                fs::read(directory.path().join(parallel.postings)).unwrap(),
                fs::read(directory.path().join(&expected.postings)).unwrap()
            );
        }
    }

    #[test]
    fn failed_stream_cleans_partial_segment_files() {
        let directory = tempfile::tempdir().unwrap();
        let records = [Ok((1, 0)), Err(anyhow::anyhow!("injected read failure"))];
        assert!(write_sorted(directory.path(), 1, 1, records).is_err());
        assert_eq!(
            fs::read_dir(directory.path().join("segments"))
                .unwrap()
                .count(),
            0
        );
    }

    #[test]
    fn failed_partitioned_stream_cleans_all_fragments() {
        let directory = tempfile::tempdir().unwrap();
        let result = write_partitioned(directory.path(), 1, 4, |part| {
            Ok([
                Ok((part as u32, 0)),
                if part == 2 {
                    Err(anyhow::anyhow!("injected partition read failure"))
                } else {
                    Ok((part as u32, 1))
                },
            ])
        });
        assert!(result.is_err());
        assert_eq!(
            fs::read_dir(directory.path().join("segments"))
                .unwrap()
                .count(),
            0
        );
    }

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
