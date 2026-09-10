use std::path::Path;

use anyhow::{Context, Result};
use git2::{ErrorCode, Repository};

pub struct GitIdentity {
    pub head: Option<String>,
    pub tree: Option<String>,
}

/// Read Git identity without a worktree status walk. Content differences are
/// computed against the last indexed worktree snapshot, including dirty files.
pub fn identity(root: &Path) -> Result<Option<GitIdentity>> {
    let Some(repository) = discover(root)? else {
        return Ok(None);
    };
    read_identity(&repository).map(Some)
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
