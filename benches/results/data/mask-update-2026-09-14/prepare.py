from pathlib import Path
import shutil

repo = Path.cwd()
work = repo / '.cache/2026-09-10/mask-update-profile'
source = work / 'source'
shutil.copytree(repo / 'src', source / 'src', dirs_exist_ok=True)
shutil.copyfile(repo / 'Cargo.lock', source / 'Cargo.lock')
cargo = (repo / 'Cargo.toml').read_text()
cargo = cargo[:cargo.index('[[bench]]')] + cargo[cargo.index('[profile.release]'):]
cargo += '\n[features]\nmask-experiment = []\n\n[[bin]]\nname = "update_probe"\npath = "src/update_probe.rs"\n'
(source / 'Cargo.toml').write_text(cargo)
index = (source / 'src/index.rs').read_text()
index = index.replace('let Some((_, state)) = files.get(position)', 'let Some((id, state)) = files.get(position)')
index = index.replace('let result = hash_file_chunks(&path, |hashes| {', 'let result = hash_file_chunks_with_mask(&path, Some((index_dir, *id)), |hashes| {')
needle = '    let mut file = File::open(path).with_context(|| format!("cannot read {}", path.display()))?;'
replacement = '''    hash_file_chunks_with_mask(path, None, emit)
}

fn hash_file_chunks_with_mask(
    path: &Path,
    mask_target: Option<(&Path, u32)>,
    mut emit: impl FnMut(Vec<ngram::GramHash>) -> Result<()>,
) -> Result<()> {
    let mut masks = crate::update_masks::Accumulator::new(mask_target);
''' + needle
assert index.count(needle) == 1
index = index.replace(needle, replacement)
index = index.replace('        emit(ngram::hashes_for_chunk(&bytes[..len]))?;', '        emit(ngram::hashes_for_chunk(&bytes[..len]))?;\n        masks.observe(&bytes[..len]);')
needle = '        bytes.copy_within(len - retained..len, 0);\n    }\n    Ok(())'
assert index.count(needle) == 1
index = index.replace(needle, '        bytes.copy_within(len - retained..len, 0);\n    }\n    masks.finish()?;\n    Ok(())')
(source / 'src/index_mask.rs').write_text(index)
print(source)
