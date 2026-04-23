//! phench - Project hatchery for thegent in Rust
//!
//! Provides fast git operations and repository management.

use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhenChError {
    #[error("Git error: {0}")]
    Git(String),
    #[error("Path error: {0}")]
    Path(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PhenChError>;

/// Repository metadata
#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub name: String,
    pub path: String,
    pub is_clean: bool,
    pub branch: String,
    pub has_remote: bool,
}

/// phench service for repository operations
pub struct PhenCh {
    workspace_path: String,
}

impl PhenCh {
    pub fn new(workspace_path: &str) -> Self {
        Self {
            workspace_path: workspace_path.to_string(),
        }
    }

    /// List all repositories in the workspace
    pub fn list_repos(&self) -> Result<Vec<RepoInfo>> {
        let path = Path::new(&self.workspace_path);
        if !path.exists() {
            return Err(PhenChError::Path(format!(
                "Workspace does not exist: {}",
                self.workspace_path
            )));
        }

        let mut repos = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && path.join(".git").exists() {
                let name = path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                repos.push(RepoInfo {
                    name,
                    path: path.to_string_lossy().to_string(),
                    is_clean: true, // TODO: check git status
                    branch: "main".to_string(),
                    has_remote: true, // TODO: check git remote
                });
            }
        }
        Ok(repos)
    }

    /// Materialize a target (checkout to specific ref)
    pub fn materialize_target(&self, repo_name: &str, refspec: &str) -> Result<String> {
        let repo_path = Path::new(&self.workspace_path).join(repo_name);
        if !repo_path.exists() {
            return Err(PhenChError::Path(format!(
                "Repository not found: {}",
                repo_name
            )));
        }
        // TODO: Implement git checkout
        Ok(format!("Materialized {} at {}", repo_name, refspec))
    }

    /// Lock a target (prevent changes during materialization)
    pub fn lock_target(&self, repo_name: &str) -> Result<String> {
        let lock_path = Path::new(&self.workspace_path)
            .join(repo_name)
            .join(".thegent.lock");
        std::fs::write(&lock_path, format!("{}", std::process::id()))?;
        Ok(lock_path.to_string_lossy().to_string())
    }

    /// Unlock a target
    pub fn unlock_target(&self, repo_name: &str) -> Result<()> {
        let lock_path = Path::new(&self.workspace_path)
            .join(repo_name)
            .join(".thegent.lock");
        if lock_path.exists() {
            std::fs::remove_file(lock_path)?;
        }
        Ok(())
    }
}
