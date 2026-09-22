//! Manifest wire schema. Changing binary field order/types requires a new magic.
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[path = "manifest/view.rs"]
pub mod view;

pub const FILE_NAME: &str = "manifest.bin";
const MAGIC: &[u8; 8] = b"CDRGMF01";
const CACHE_MAGIC: &[u8; 8] = b"CDRGMF02";

fn config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
        .with_limit::<1_073_741_824>()
}

pub fn encode(manifest: &Manifest) -> Result<Vec<u8>> {
    if manifest.registry.is_none() {
        let mut bytes = MAGIC.to_vec();
        bytes.extend(bincode::serde::encode_to_vec(
            (
                manifest.version,
                &manifest.root,
                manifest.generation,
                manifest.git_repository,
                &manifest.git_head,
                &manifest.git_tree,
                &manifest.source_state,
                &manifest.documents,
                &manifest.segments,
            ),
            config(),
        )?);
        return Ok(bytes);
    }
    let payload = bincode::serde::encode_to_vec(manifest, config())?;
    let mut bytes = CACHE_MAGIC.to_vec();
    bytes.extend(git2::Oid::hash_object(git2::ObjectType::Blob, &payload)?.as_bytes());
    bytes.extend(payload);
    Ok(bytes)
}

pub fn publication(bytes: &[u8]) -> Option<[u8; 20]> {
    bytes.strip_prefix(CACHE_MAGIC)?.get(..20)?.try_into().ok()
}

pub fn decode(bytes: &[u8]) -> Result<Manifest> {
    if let Some(data) = bytes.strip_prefix(CACHE_MAGIC) {
        if data.len() < 20 {
            bail!("truncated manifest identity");
        }
        let (identity, payload) = data.split_at(20);
        if git2::Oid::hash_object(git2::ObjectType::Blob, payload)?.as_bytes() != identity {
            bail!("manifest checksum mismatch");
        }
        let (mut manifest, consumed): (Manifest, usize) =
            bincode::serde::decode_from_slice(payload, config())?;
        if consumed != payload.len() {
            bail!("trailing data in binary manifest");
        }
        manifest.publication = Some(identity.try_into()?);
        Ok(manifest)
    } else if let Some(payload) = bytes.strip_prefix(MAGIC) {
        let (manifest, consumed): (LegacyManifest, usize) =
            bincode::serde::decode_from_slice(payload, config())?;
        if consumed != payload.len() {
            bail!("trailing data in binary manifest");
        }
        Ok(manifest.into())
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
pub struct RegistryIdentity {
    pub epoch: u64,
    pub count: usize,
    pub digest: [u8; 20],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub registry: Option<RegistryIdentity>,
    // Valid only after checksum validation on load or publication preparation.
    #[serde(skip)]
    pub publication: Option<[u8; 20]>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyManifest {
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

impl From<LegacyManifest> for Manifest {
    fn from(m: LegacyManifest) -> Self {
        Self {
            registry: None,
            publication: None,
            version: m.version,
            root: m.root,
            generation: m.generation,
            git_repository: m.git_repository,
            git_head: m.git_head,
            git_tree: m.git_tree,
            source_state: m.source_state,
            documents: m.documents,
            segments: m.segments,
        }
    }
}
impl From<Manifest> for LegacyManifest {
    fn from(m: Manifest) -> Self {
        Self {
            version: m.version,
            root: m.root,
            generation: m.generation,
            git_repository: m.git_repository,
            git_head: m.git_head,
            git_tree: m.git_tree,
            source_state: m.source_state,
            documents: m.documents,
            segments: m.segments,
        }
    }
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
