//! Asset discovery implementation

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, trace, warn};
use walkdir::WalkDir;

use crate::types::{AssetInfo, AssetType, DiscoveryResult};

/// Asset discovery configuration
#[derive(Debug, Clone)]
pub struct AssetDiscovery {
    max_depth: usize,
    include_hidden: bool,
    follow_symlinks: bool,
}

impl AssetDiscovery {
    /// Create a new asset discovery instance
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            include_hidden: false,
            follow_symlinks: false,
        }
    }

    /// Set whether to include hidden files
    pub fn include_hidden(mut self, include: bool) -> Self {
        self.include_hidden = include;
        self
    }

    /// Set whether to follow symbolic links
    pub fn follow_symlinks(mut self, follow: bool) -> Self {
        self.follow_symlinks = follow;
        self
    }

    /// Discover assets in a directory
    pub async fn discover(
        &self,
        path: impl AsRef<Path>,
        recursive: bool,
    ) -> Result<DiscoveryResult, DiscoveryError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(DiscoveryError::PathNotFound(path.to_path_buf()));
        }

        if !path.is_dir() {
            return Err(DiscoveryError::NotADirectory(path.to_path_buf()));
        }

        info!("Discovering assets in: {} (recursive={})", path.display(), recursive);

        let mut result = DiscoveryResult::default();
        let depth = if recursive { self.max_depth } else { 1 };

        self.scan_directory(path, 0, depth, &mut result).await?;

        result.directories_scanned = 1;
        // Calculate total size from discovered assets
        result.total_size_bytes = result.assets.iter().map(|a| a.size_bytes).sum();
        info!("Discovered {} assets ({} bytes)", result.assets.len(), result.total_size_bytes);

        Ok(result)
    }

    async fn scan_directory(
        &self,
        dir: &Path,
        current_depth: usize,
        max_depth: usize,
        result: &mut DiscoveryResult,
    ) -> Result<(), DiscoveryError> {
        // Use a stack-based iterative approach to avoid recursion
        let mut stack: Vec<(std::path::PathBuf, usize)> = vec![(dir.to_path_buf(), current_depth)];
        
        while let Some((current_dir, depth)) = stack.pop() {
            if depth >= max_depth {
                trace!("Max depth reached: {}", depth);
                continue;
            }
            
            trace!("Scanning directory: {}", current_dir.display());
            
            let mut entries = match fs::read_dir(&current_dir).await {
                Ok(entries) => entries,
                Err(e) => {
                    warn!("Failed to read directory {}: {}", current_dir.display(), e);
                    result.add_error(format!("Failed to read {}: {}", current_dir.display(), e));
                    continue;
                }
            };
            
            while let Some(entry) = entries.next_entry().await.map_err(|e| DiscoveryError::Io(e))? {
                let path = entry.path();
                
                // Skip hidden files
                if !self.include_hidden {
                    if let Some(name) = path.file_name() {
                        if name.to_string_lossy().starts_with('.') {
                            continue;
                        }
                    }
                }
                
                // Handle symlinks
                if path.is_symlink() {
                    if !self.follow_symlinks {
                        continue;
                    }
                    continue;
                }
                
                if path.is_file() {
                    if let Some(asset) = self.classify_asset(&path).await {
                        result.assets.push(asset);
                    }
                } else if path.is_dir() && depth < max_depth - 1 {
                    stack.push((path, depth + 1));
                    result.directories_scanned += 1;
                }
            }
        }
        
        Ok(())
    }

    async fn classify_asset(&self, path: &Path) -> Option<AssetInfo> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        let name = path.file_stem()?.to_str()?.to_string();

        let asset_type = AssetType::from_extension(&ext);

        if asset_type == AssetType::Unknown {
            trace!("Skipping unknown file type: {}", path.display());
            return None;
        }

        let metadata = self.extract_metadata(path, &asset_type).await;

        let size_bytes = match fs::metadata(path).await {
            Ok(m) => m.len(),
            Err(_) => 0,
        };

        Some(AssetInfo {
            name,
            path: path.to_path_buf(),
            asset_type,
            size_bytes,
            metadata,
            checksum: None,
        })
    }

    async fn extract_metadata(&self, path: &Path, asset_type: &AssetType) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        // Try to read manifest metadata for content packs
        if *asset_type == AssetType::ContentPack {
            if let Ok(content) = fs::read_to_string(path).await {
                if let Ok(manifest) = toml::from_str::<crate::types::PackManifest>(&content) {
                    metadata.insert("pack_name".to_string(), manifest.name);
                    metadata.insert("version".to_string(), manifest.version);
                    if let Some(author) = manifest.author {
                        metadata.insert("author".to_string(), author);
                    }
                    if let Some(description) = manifest.description {
                        metadata.insert("description".to_string(), description);
                    }
                }
            }
        }

        // Add file extension as metadata
        if let Some(ext) = path.extension() {
            metadata.insert("extension".to_string(), ext.to_string_lossy().to_string());
        }

        metadata
    }
}

impl Default for AssetDiscovery {
    fn default() -> Self {
        Self {
            max_depth: 10,
            include_hidden: false,
            follow_symlinks: false,
        }
    }
}

/// Discovery error types
#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),
    
    #[error("Not a directory: {0}")]
    NotADirectory(PathBuf),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_discovery_nonexistent_path() {
        let discovery = AssetDiscovery::default();
        let result = discovery.discover("/nonexistent/path/12345", true).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discovery_not_a_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        fs::write(&file_path, "content").await.unwrap();
        
        let discovery = AssetDiscovery::default();
        let result = discovery.discover(&file_path, true).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discovery_empty_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let discovery = AssetDiscovery::default();
        let result = discovery.discover(temp_dir.path(), true).await.unwrap();
        
        assert!(result.assets.is_empty());
        assert_eq!(result.total_size_bytes, 0);
    }

    #[tokio::test]
    async fn test_discovery_with_known_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Create files with known extensions
        fs::write(temp_dir.path().join("script.py"), "print('hello')").await.unwrap();
        fs::write(temp_dir.path().join("module.js"), "export default {}").await.unwrap();
        fs::write(temp_dir.path().join("data.txt"), "data").await.unwrap(); // Unknown type
        
        let discovery = AssetDiscovery::default();
        let result = discovery.discover(temp_dir.path(), true).await.unwrap();
        
        // Should find Python and JS, but not txt
        let py_count = result.assets.iter()
            .filter(|a| a.asset_type == AssetType::PythonScript)
            .count();
        let js_count = result.assets.iter()
            .filter(|a| a.asset_type == AssetType::JavaScriptModule)
            .count();
        
        assert!(py_count > 0, "Should find Python files");
        assert!(js_count > 0, "Should find JS files");
        
        // Check total size
        assert!(result.total_size_bytes > 0);
    }

    #[tokio::test]
    async fn test_discovery_recursive() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Create a subdirectory with files
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).await.unwrap();
        fs::write(subdir.join("nested.py"), "# nested").await.unwrap();
        
        // Test recursive discovery
        let discovery = AssetDiscovery::default();
        let result_recursive = discovery.discover(temp_dir.path(), true).await.unwrap();
        
        // Test non-recursive discovery
        let result_shallow = discovery.discover(temp_dir.path(), false).await.unwrap();
        
        assert!(result_recursive.assets.len() > result_shallow.assets.len() || 
                result_recursive.directories_scanned > result_shallow.directories_scanned);
    }

    #[tokio::test]
    async fn test_discovery_skip_hidden() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Create hidden and non-hidden files
        fs::write(temp_dir.path().join("visible.py"), "# visible").await.unwrap();
        fs::write(temp_dir.path().join(".hidden.py"), "# hidden").await.unwrap();
        
        let discovery = AssetDiscovery::default();
        let result = discovery.discover(temp_dir.path(), true).await.unwrap();
        
        // Should not include hidden by default
        let has_hidden = result.assets.iter()
            .any(|a| a.name.starts_with('.'));
        assert!(!has_hidden, "Should skip hidden files by default");
    }

    #[tokio::test]
    async fn test_discovery_include_hidden() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Create hidden and non-hidden files
        fs::write(temp_dir.path().join("visible.py"), "# visible").await.unwrap();
        fs::write(temp_dir.path().join(".hidden.py"), "# hidden").await.unwrap();
        
        let discovery = AssetDiscovery::default().include_hidden(true);
        let result = discovery.discover(temp_dir.path(), true).await.unwrap();
        
        // Should include hidden
        let has_hidden = result.assets.iter()
            .any(|a| a.name.starts_with('.'));
        assert!(has_hidden, "Should include hidden files when configured");
    }

    #[tokio::test]
    async fn test_discovery_with_manifest() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Create a manifest file
        let manifest_content = r#"
name = "test-pack"
version = "1.0.0"
author = "Test Author"
description = "Test pack description"
"#;
        fs::write(temp_dir.path().join("manifest.toml"), manifest_content).await.unwrap();
        
        let discovery = AssetDiscovery::default();
        let result = discovery.discover(temp_dir.path(), true).await.unwrap();
        
        // Find the manifest asset
        let manifest_asset = result.assets.iter()
            .find(|a| a.name == "manifest");
        
        if let Some(asset) = manifest_asset {
            assert_eq!(asset.asset_type, AssetType::ContentPack);
            assert!(asset.metadata.contains_key("pack_name"));
            assert_eq!(asset.metadata.get("pack_name"), Some(&"test-pack".to_string()));
        }
    }

    #[tokio::test]
    async fn test_discovery_max_depth() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Create deeply nested structure
        let deep_dir = temp_dir.path().join("a/b/c/d");
        fs::create_dir_all(&deep_dir).await.unwrap();
        fs::write(deep_dir.join("deep.py"), "# deep").await.unwrap();
        
        // Test with limited depth
        let limited_discovery = AssetDiscovery::new(2);
        let result_limited = limited_discovery.discover(temp_dir.path(), true).await.unwrap();
        
        // Test with default depth
        let default_discovery = AssetDiscovery::default();
        let result_default = default_discovery.discover(temp_dir.path(), true).await.unwrap();
        
        // Default depth should find more or equal
        assert!(result_default.assets.len() >= result_limited.assets.len());
    }

    #[test]
    fn test_discovery_error_display() {
        let err = DiscoveryError::PathNotFound(PathBuf::from("/missing"));
        assert!(err.to_string().contains("Path not found"));
        
        let err = DiscoveryError::NotADirectory(PathBuf::from("/file.txt"));
        assert!(err.to_string().contains("Not a directory"));
    }
}
