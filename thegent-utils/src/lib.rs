//! Utility functions for thegent
//!
//! This crate provides common utilities for binary resolution, PATH handling,
//! and command execution used across the thegent ecosystem.

use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("Binary not found: {0}")]
    NotFound(String),
    #[error("Binary is a shim: {0}")]
    IsShim(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub use binary::{first_available, resolve_binary, resolve_git_binary, resolve_real_binary};
pub use command::{command_exists, exec_command};
pub use path::{get_repo_root, get_safe_path};

mod binary {
    use super::*;
    use std::path::Path;

    /// Resolve a binary from PATH, preferring THEGENT_TOOL_BIN_PATH if set
    pub fn resolve_binary(name: &str) -> Option<PathBuf> {
        if let Ok(tool_path) = std::env::var("THEGENT_TOOL_BIN_PATH") {
            let mut search_paths = tool_path.split(':').collect::<Vec<_>>();
            let path_env = std::env::var("PATH").unwrap_or_default();
            search_paths.extend(path_env.split(':'));

            for dir in search_paths {
                let candidate = Path::new(dir).join(name);
                if candidate.exists() && is_executable(&candidate) {
                    return Some(candidate);
                }
            }
        }
        which::which(name).ok()
    }

    /// Find first available tool from a list of candidates
    pub fn first_available(candidates: &[&str]) -> Option<PathBuf> {
        for candidate in candidates {
            if let Ok(path) = which::which(candidate) {
                return Some(path);
            }
        }
        None
    }

    /// Check if a path is executable
    fn is_executable(path: &Path) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(path) {
                let permissions = metadata.permissions();
                (permissions.mode() & 0o111) != 0
            } else {
                false
            }
        }
        #[cfg(not(unix))]
        {
            if let Some(ext) = path.extension() {
                matches!(ext.to_str(), Some("exe" | "bat" | "cmd" | "com"))
            } else {
                false
            }
        }
    }

    /// Resolve real binary from safe PATH, skipping shims in ~/.local/bin
    pub fn resolve_real_binary(binary: &str) -> Result<PathBuf, UtilsError> {
        let safe_path = get_safe_path();

        let output = Command::new("command")
            .arg("-v")
            .arg(binary)
            .env("PATH", &safe_path)
            .output()?;

        if !output.status.success() {
            return Err(UtilsError::NotFound(binary.to_string()));
        }

        let candidate = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let candidate_path = PathBuf::from(&candidate);

        if let Some(file_name) = candidate_path.file_name() {
            let file_name_str = file_name.to_string_lossy();
            if (file_name_str == binary || file_name_str == format!("{}.exe", binary))
                && candidate.contains("/.local/bin/")
            {
                return Err(UtilsError::IsShim(candidate));
            }
        }

        if !candidate_path.is_file() {
            return Err(UtilsError::NotFound(binary.to_string()));
        }

        Ok(candidate_path)
    }

    /// Resolve git binary, caching result
    pub fn resolve_git_binary() -> Option<PathBuf> {
        if let Ok(git_bin) = std::env::var("THEGENT_GIT_BIN") {
            let path = PathBuf::from(git_bin);
            if path.is_file() {
                return Some(path);
            }
        }
        resolve_real_binary("git").ok()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_resolve_binary() {
            let git = resolve_binary("git");
            assert!(git.is_some() || which::which("git").is_err());
        }

        #[test]
        fn test_first_available() {
            let result = first_available(&["sh", "bash", "zsh"]);
            assert!(result.is_some());
        }
    }
}

mod command {
    use std::process::{Command, ExitCode};

    /// Execute a command and return its exit code
    pub fn exec_command(cmd: &str, args: &[String]) -> ExitCode {
        match Command::new(cmd).args(args).status() {
            Ok(status) => {
                let code = status.code().unwrap_or(1);
                ExitCode::from(code as u8)
            }
            Err(e) => {
                eprintln!("thegent-utils: failed to execute {}: {}", cmd, e);
                ExitCode::from(127)
            }
        }
    }

    /// Check if a command exists in PATH
    pub fn command_exists(cmd: &str) -> bool {
        Command::new("command")
            .arg("-v")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_command_exists() {
            assert!(command_exists("ls") || command_exists("dir"));
        }
    }
}

mod path {
    use super::*;

    /// Get safe PATH for tool resolution
    pub fn get_safe_path() -> String {
        std::env::var("THEGENT_TOOL_BIN_PATH")
            .unwrap_or_else(|_| "/usr/bin:/opt/homebrew/bin:/bin:/usr/sbin:/sbin".to_string())
    }

    /// Get repo root (for git operations)
    pub fn get_repo_root() -> PathBuf {
        if let Some(path) = resolve_binary("git") {
            if let Ok(output) = Command::new(&path)
                .args(["rev-parse", "--show-toplevel"])
                .output()
            {
                if output.status.success() {
                    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !root.is_empty() {
                        return PathBuf::from(root);
                    }
                }
            }
        }
        PathBuf::from(".")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_repo_root() {
            let root = get_repo_root();
            assert!(root.exists());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_git_binary() {
        if command_exists("git") {
            assert!(resolve_git_binary().is_some());
        }
    }

    #[test]
    fn test_safe_path_has_default() {
        let path = get_safe_path();
        assert!(!path.is_empty());
    }
}
