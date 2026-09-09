//! Manifest wire schema. Changing binary field order/types requires a new magic.
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub const FILE_NAME: &str = "manifest.bin";
const MAGIC: &[u8; 8] = b"CDRGMF01";

fn config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
        .with_limit::<1_073_741_824>()
}

pub fn encode(manifest: &Manifest) -> Result<Vec<u8>> {
    let mut bytes = MAGIC.to_vec();
    bytes.extend(bincode::serde::encode_to_vec(manifest, config())?);
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<Manifest> {
    if let Some(payload) = bytes.strip_prefix(MAGIC) {
        let (manifest, consumed) = bincode::serde::decode_from_slice(payload, config())?;
        if consumed != payload.len() {
            bail!("trailing data in binary manifest");
        }
        Ok(manifest)
    } else if bytes.iter().find(|byte| !byte.is_ascii_whitespace()) == Some(&b'{') {
        Ok(serde_json::from_slice(bytes)?)
    } else {
        bail!("unsupported manifest header; rebuild the index")
    }
}

pub fn resolve_path(path: &Path) -> io::Result<PathBuf> {
    match fs::metadata(path) {
        Ok(_) => Ok(path.to_owned()),
        Err(error)
            if error.kind() == io::ErrorKind::NotFound
                && path.extension().is_some_and(|ext| ext == "bin") =>
        {
            let legacy = path.with_extension("json");
            fs::metadata(&legacy)?;
            Ok(legacy)
        }
        Err(error) => Err(error),
    }
}

pub fn read(path: &Path) -> Result<Manifest> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error)
            if error.kind() == io::ErrorKind::NotFound
                && path.extension().is_some_and(|ext| ext == "bin") =>
        {
            fs::read(path.with_extension("json"))?
        }
        Err(error) => return Err(error.into()),
    };
    decode(&bytes).with_context(|| format!("cannot decode manifest {}", path.display()))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub root: PathBuf,
    pub generation: u64,
    pub git_repository: bool,
    pub git_head: Option<String>,
    pub git_tree: Option<String>,
    pub source_state: Vec<FileState>,
    pub documents: Vec<Document>,
    pub segments: Vec<SegmentMeta>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileState {
    pub path: PathBuf,
    pub len: u64,
    pub modified_nanos: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    pub path: PathBuf,
    pub len: u64,
    pub modified_nanos: u128,
    pub active: bool,
    pub searchable: bool,
    pub segment_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SegmentMeta {
    pub id: u64,
    pub lookup: PathBuf,
    pub postings: PathBuf,
    pub ngrams: u64,
}
