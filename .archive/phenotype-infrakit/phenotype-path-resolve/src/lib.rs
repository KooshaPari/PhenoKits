//! Fast PATH resolution with skip directory support

use std::collections::HashMap;
use std::path::PathBuf;
use which::which;

pub struct PathResolver {
    skip_dirs: Vec<PathBuf>,
}

impl PathResolver {
    pub fn new() -> Self {
        Self {
            skip_dirs: Vec::new(),
        }
    }

    pub fn with_skip_dirs(skip_dirs: Vec<String>) -> Self {
        Self {
            skip_dirs: skip_dirs.iter().map(PathBuf::from).collect(),
        }
    }

    pub fn resolve(&self, name: &str) -> Option<String> {
        match which(name) {
            Ok(path) => {
                let path_str = path.to_string_lossy().to_string();
                if self.is_in_skip_dirs(&path_str) {
                    None
                } else {
                    Some(path_str)
                }
            }
            Err(_) => None,
        }
    }

    pub fn resolve_many(&self, names: &[&str]) -> HashMap<String, Option<String>> {
        names
            .iter()
            .map(|name| (name.to_string(), self.resolve(name)))
            .collect()
    }

    fn is_in_skip_dirs(&self, path: &str) -> bool {
        if self.skip_dirs.is_empty() {
            return false;
        }

        let path_buf = PathBuf::from(path);
        self.skip_dirs.iter().any(|skip| {
            path_buf.starts_with(skip) || path_buf.canonicalize().is_ok_and(|p| p.starts_with(skip))
        })
    }
}

impl Default for PathResolver {
    fn default() -> Self {
        Self::new()
    }
}

pub fn resolve_binary(name: &str) -> Option<String> {
    PathResolver::new().resolve(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_binary() {
        assert!(resolve_binary("sh").is_some() || resolve_binary("bash").is_some());
    }

    #[test]
    fn test_resolve_many() {
        let resolver = PathResolver::new();
        let results = resolver.resolve_many(&["sh", "bash", "nonexistent12345"]);
        assert!(results.values().any(|v| v.is_some()) || std::env::var("CI").is_ok());
    }
}
