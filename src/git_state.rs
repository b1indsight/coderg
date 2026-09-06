use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use git2::{ErrorCode, Repository, RepositoryState, StatusOptions};

pub struct GitState {
    pub head: Option<String>,
    pub tree: Option<String>,
    pub clean: bool,
    /// Paths are relative to the indexed root. `None` means libgit2 returned a
    /// path that cannot be represented as UTF-8, so callers must use metadata.
    pub changed_paths: Option<HashSet<PathBuf>>,
}

pub struct GitIdentity {
    pub head: Option<String>,
    pub tree: Option<String>,
}

/// Read only the current commit identity. Unlike `inspect`, this does not walk
/// the worktree, so it is suitable for the common per-search freshness check.
pub fn identity(root: &Path) -> Result<Option<GitIdentity>> {
    let Some(repository) = discover(root)? else {
        return Ok(None);
    };
    read_identity(&repository).map(Some)
}

/// Inspect Git in-process. Status is scoped to the indexed root and treats the
/// index directory itself as invisible, matching the source file walker.
pub fn inspect(root: &Path, index_dir: &Path) -> Result<Option<GitState>> {
    let Some(repository) = discover(root)? else {
        return Ok(None);
    };
    let workdir = repository
        .workdir()
        .context("bare Git repositories do not have a searchable worktree")?;
    let scoped_root = root
        .strip_prefix(workdir)
        .context("search root is outside the Git worktree")?;
    let relative_index = index_dir.strip_prefix(workdir).ok();

    let identity = read_identity(&repository)?;

    let mut options = StatusOptions::new();
    options
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false)
        .exclude_submodules(true);
    if !scoped_root.as_os_str().is_empty() {
        options.pathspec(scoped_root.to_string_lossy().as_ref());
    }
    let statuses = repository
        .statuses(Some(&mut options))
        .context("cannot inspect Git worktree")?;
    let mut changed_paths = HashSet::new();
    let mut paths_complete = true;
    for entry in statuses.iter() {
        let Ok(path) = entry.path().map(Path::new) else {
            paths_complete = false;
            continue;
        };
        if relative_index.is_some_and(|index| path.starts_with(index)) {
            continue;
        }
        let Ok(path) = path.strip_prefix(scoped_root) else {
            continue;
        };
        changed_paths.insert(path.to_path_buf());
    }
    let clean =
        changed_paths.is_empty() && paths_complete && repository.state() == RepositoryState::Clean;
    Ok(Some(GitState {
        head: identity.head,
        tree: identity.tree,
        clean,
        changed_paths: paths_complete.then_some(changed_paths),
    }))
}

fn discover(root: &Path) -> Result<Option<Repository>> {
    match Repository::discover(root) {
        Ok(repository) => Ok(Some(repository)),
        Err(error) if error.code() == ErrorCode::NotFound => Ok(None),
        Err(error) => Err(error).context("cannot discover Git repository"),
    }
}

fn read_identity(repository: &Repository) -> Result<GitIdentity> {
    let (head, tree) = match repository.head() {
        Ok(reference) => {
            let commit = reference
                .peel_to_commit()
                .context("Git HEAD does not resolve to a commit")?;
            (
                Some(commit.id().to_string()),
                Some(commit.tree_id().to_string()),
            )
        }
        Err(error) if error.code() == ErrorCode::UnbornBranch => (None, None),
        Err(error) => return Err(error).context("cannot read Git HEAD"),
    };
    Ok(GitIdentity { head, tree })
}
