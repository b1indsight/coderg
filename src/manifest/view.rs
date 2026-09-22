//! Read-only search view of the existing fixed-integer binary layouts.
//! Only the small header and segment directory are deserialized. File records
//! stay mapped; integrity verification is deferred until cache state is used.
use super::*;
#[cfg(unix)]
use memmap2::{Mmap, MmapOptions};
#[cfg(not(unix))]
use std::io::Read;
use std::{fs::File, ops::Range};

#[cfg(unix)]
type Bytes = Mmap;
#[cfg(not(unix))]
type Bytes = Vec<u8>;

/// Fixed-layout search metadata. This header contains no document records and
/// does not imply that the manifest payload has passed checksum verification.
#[derive(Deserialize)]
pub struct Header {
    pub version: u32,
    pub root: PathBuf,
    #[allow(dead_code)]
    generation: u64,
    pub git_repository: bool,
    pub git_head: Option<String>,
    pub git_tree: Option<String>,
}

struct DocumentSlot {
    path: Range<usize>,
    tail: usize,
}

/// Search metadata and mapped records; a full manifest requires materialize().
pub struct View {
    pub header: Header,
    pub segments: Vec<SegmentMeta>,
    bytes: Bytes,
    source_offset: usize,
    source_count: usize,
    documents: Vec<DocumentSlot>,
}

struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Cursor<'_> {
    fn take(&mut self, count: usize) -> Result<Range<usize>> {
        let end = self
            .at
            .checked_add(count)
            .context("manifest offset overflow")?;
        if end > self.bytes.len() {
            bail!("truncated manifest record");
        }
        let range = self.at..end;
        self.at = end;
        Ok(range)
    }

    fn u64(&mut self) -> Result<u64> {
        let range = self.take(8)?;
        Ok(u64::from_le_bytes(self.bytes[range].try_into()?))
    }

    fn count(&mut self, minimum: usize) -> Result<usize> {
        let count = usize::try_from(self.u64()?)?;
        if count > (self.bytes.len() - self.at) / minimum {
            bail!("manifest record count exceeds payload");
        }
        Ok(count)
    }

    fn path(&mut self) -> Result<Range<usize>> {
        let len = usize::try_from(self.u64()?)?;
        let range = self.take(len)?;
        std::str::from_utf8(&self.bytes[range.clone()])?;
        Ok(range)
    }
}

impl View {
    /// Open mapped records without full deserialization. Legacy formats return
    /// None so callers can use the complete reader; checksum policy is unchanged.
    pub fn open(path: &Path) -> Result<Option<Self>> {
        let file = File::open(resolve_path(path)?)?;
        let len = file.metadata()?.len();
        if len == 0 || len > 1_073_741_824 {
            bail!("invalid manifest size");
        }
        // Writers atomically replace manifests and never modify mapped bytes.
        #[cfg(unix)]
        let bytes = unsafe { MmapOptions::new().map(&file)? };
        // Do not hold a mapping that could prevent atomic replacement on other
        // platforms. The same record view still avoids owned deserialization.
        #[cfg(not(unix))]
        let bytes = {
            let mut bytes = Vec::with_capacity(len as usize);
            (&file).read_to_end(&mut bytes)?;
            bytes
        };
        let mut at = if bytes.starts_with(CACHE_MAGIC) {
            let payload = bytes.get(28..).context("truncated manifest identity")?;
            let (_, used): (Option<RegistryIdentity>, usize) =
                bincode::serde::decode_from_slice(payload, config())?;
            28 + used
        } else if bytes.starts_with(MAGIC) {
            8
        } else {
            return Ok(None); // Legacy JSON and unsupported headers use the full reader.
        };
        let (header, used): (Header, usize) =
            bincode::serde::decode_from_slice(&bytes[at..], config())?;
        at += used;
        let mut cursor = Cursor { bytes: &bytes, at };
        let source_count = cursor.count(32)?;
        let source_offset = cursor.at;
        for _ in 0..source_count {
            cursor.path()?;
            cursor.take(24)?; // length and mtime; no FileState allocation
        }
        let count = cursor.count(42)?;
        let mut documents = Vec::with_capacity(count);
        for _ in 0..count {
            let path = cursor.path()?;
            let tail = cursor.take(34)?.start;
            if bytes[tail + 24] > 1 || bytes[tail + 25] > 1 {
                bail!("invalid document flags");
            }
            documents.push(DocumentSlot { path, tail });
        }
        let (segments, used): (Vec<SegmentMeta>, usize) =
            bincode::serde::decode_from_slice(&bytes[cursor.at..], config())?;
        if cursor.at + used != bytes.len() {
            bail!("trailing data in binary manifest");
        }
        Ok(Some(Self {
            header,
            segments,
            bytes,
            source_offset,
            source_count,
            documents,
        }))
    }

    /// Decode complete records and verify integrity using the canonical reader.
    pub fn materialize(&self) -> Result<Manifest> {
        decode(&self.bytes)
    }

    pub fn source_len(&self) -> usize {
        self.source_count
    }

    pub fn same_source(&self, current: &[FileState]) -> Result<bool> {
        if current.len() != self.source_count {
            return Ok(false);
        }
        let mut cursor = Cursor {
            bytes: &self.bytes,
            at: self.source_offset,
        };
        for state in current {
            let path = cursor.path()?;
            let len = cursor.u64()?;
            let modified = cursor.take(16)?;
            if state.path.to_str().map(str::as_bytes) != Some(&self.bytes[path])
                || state.len != len
                || state.modified_nanos != u128::from_le_bytes(self.bytes[modified].try_into()?)
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn document_path(&self, id: u32) -> Result<&Path> {
        let slot = self
            .documents
            .get(id as usize)
            .context("invalid document ID")?;
        Ok(Path::new(std::str::from_utf8(
            &self.bytes[slot.path.clone()],
        )?))
    }

    pub fn is_live(&self, id: u32, segment: Option<u64>) -> Result<bool> {
        let slot = self
            .documents
            .get(id as usize)
            .context("invalid document ID")?;
        if self.bytes[slot.tail + 24] == 0 || self.bytes[slot.tail + 25] == 0 {
            return Ok(false);
        }
        Ok(match segment {
            None => true,
            Some(id) => {
                id == u64::from_le_bytes(self.bytes[slot.tail + 26..slot.tail + 34].try_into()?)
            }
        })
    }

    pub fn active_document_ids(&self) -> Vec<u32> {
        self.documents
            .iter()
            .enumerate()
            .filter(|(_, slot)| self.bytes[slot.tail + 24] != 0 && self.bytes[slot.tail + 25] != 0)
            .map(|(id, _)| id as u32)
            .collect()
    }
}
