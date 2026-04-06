//! Pack manifest utilities

use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, warn};

use crate::types::{PackManifest, ValidationResult};

/// Read a manifest from a file path
pub async fn read_manifest(path: impl AsRef<Path>) -> Result<PackManifest, ManifestError> {
    let path = path.as_ref();

    // If path is a directory, look for phenotype.toml inside it
    let manifest_path = if path.is_dir() {
        path.join(PackManifest::file_name())
    } else {
        path.to_path_buf()
    };

    debug!("Reading manifest from: {}", manifest_path.display());

    if !manifest_path.exists() {
        return Err(ManifestError::NotFound(manifest_path));
    }

    let content = fs::read_to_string(&manifest_path).await
        .map_err(|e| ManifestError::Io(e))?;

    let manifest: PackManifest = toml::from_str(&content)
        .map_err(|e| ManifestError::Parse(e.to_string()))?;

    info!(
        "Loaded manifest: {} v{} ({} assets, {} dependencies)",
        manifest.name,
        manifest.version,
        manifest.assets.len(),
        manifest.dependencies.len()
    );

    Ok(manifest)
}

/// Write a manifest to a file
pub async fn write_manifest(
    path: impl AsRef<Path>,
    manifest: &PackManifest,
) -> Result<(), ManifestError> {
    let path = path.as_ref();

    // If path is a directory, create phenotype.toml inside it
    let manifest_path = if path.is_dir() {
        path.join(PackManifest::file_name())
    } else {
        path.to_path_buf()
    };

    debug!("Writing manifest to: {}", manifest_path.display());

    // Ensure parent directory exists
    if let Some(parent) = manifest_path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| ManifestError::Io(e))?;
    }

    let content = toml::to_string_pretty(manifest)
        .map_err(|e| ManifestError::Serialize(e.to_string()))?;

    fs::write(&manifest_path, content).await
        .map_err(|e| ManifestError::Io(e))?;

    info!("Wrote manifest to: {}", manifest_path.display());

    Ok(())
}

/// Find all manifests in a directory
pub async fn find_manifests(
    root: impl AsRef<Path>,
    recursive: bool,
) -> Result<Vec<PathBuf>, ManifestError> {
    let root = root.as_ref();
    let mut manifests = Vec::new();

    if !root.exists() {
        return Err(ManifestError::PathNotFound(root.to_path_buf()));
    }

    // Use a stack-based iterative approach
    let mut stack: Vec<(std::path::PathBuf, bool)> = vec![(root.to_path_buf(), recursive)];
    
    while let Some((current_dir, recurse)) = stack.pop() {
        let mut entries = fs::read_dir(&current_dir).await
            .map_err(|e| ManifestError::Io(e))?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| ManifestError::Io(e))? {
            let path = entry.path();
            
            if path.is_file() && path.file_name() == Some(std::ffi::OsStr::new("phenotype.toml")) {
                manifests.push(path);
            } else if path.is_dir() && recurse {
                stack.push((path, true));
            }
        }
    }

    Ok(manifests)
}

/// Create a new manifest file
pub async fn create_manifest(
    path: impl AsRef<Path>,
    name: impl Into<String>,
    version: impl Into<String>,
) -> Result<PackManifest, ManifestError> {
    let manifest = PackManifest::new(name, version);
    write_manifest(&path, &manifest).await?;
    Ok(manifest)
}

/// Manifest error types
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("Manifest not found: {0}")]
    NotFound(PathBuf),
    
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),
    
    #[error("Failed to parse manifest: {0}")]
    Parse(String),
    
    #[error("Failed to serialize manifest: {0}")]
    Serialize(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_read_manifest() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let content = r#"
name = "test-pack"
version = "1.0.0"
author = "Test"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), content).await.unwrap();
        
        let manifest = read_manifest(temp_dir.path()).await.unwrap();
        assert_eq!(manifest.name, "test-pack");
        assert_eq!(manifest.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_read_manifest_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("phenotype.toml");
        
        let content = r#"
name = "test-pack"
version = "1.0.0"
"#;
        fs::write(&path, content).await.unwrap();
        
        let manifest = read_manifest(&path).await.unwrap();
        assert_eq!(manifest.name, "test-pack");
    }

    #[tokio::test]
    async fn test_read_manifest_not_found() {
        let result = read_manifest("/nonexistent/path").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ManifestError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_write_manifest() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = PackManifest::new("my-pack", "2.0.0")
            .with_author("Author")
            .with_description("Description");
        
        write_manifest(temp_dir.path(), &manifest).await.unwrap();
        
        let path = temp_dir.path().join("phenotype.toml");
        assert!(path.exists());
        
        let content = fs::read_to_string(&path).await.unwrap();
        assert!(content.contains("my-pack"));
        assert!(content.contains("2.0.0"));
    }

    #[tokio::test]
    async fn test_find_manifests() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Create manifests in different directories
        fs::create_dir(temp_dir.path().join("pack1")).await.unwrap();
        fs::create_dir(temp_dir.path().join("pack2")).await.unwrap();
        fs::create_dir(temp_dir.path().join("pack2/nested")).await.unwrap();
        
        fs::write(
            temp_dir.path().join("pack1/phenotype.toml"),
            "name = 'pack1'\nversion = '1.0.0'"
        ).await.unwrap();
        fs::write(
            temp_dir.path().join("pack2/phenotype.toml"),
            "name = 'pack2'\nversion = '1.0.0'"
        ).await.unwrap();
        fs::write(
            temp_dir.path().join("pack2/nested/phenotype.toml"),
            "name = 'nested'\nversion = '1.0.0'"
        ).await.unwrap();
        
        // Non-recursive - should only find root level
        let manifests = find_manifests(temp_dir.path(), false).await.unwrap();
        assert_eq!(manifests.len(), 0); // Root has no manifest
        
        // Recursive - should find all
        let manifests = find_manifests(temp_dir.path(), true).await.unwrap();
        assert_eq!(manifests.len(), 3);
    }

    #[tokio::test]
    async fn test_create_manifest() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = create_manifest(
            temp_dir.path(),
            "new-pack",
            "0.1.0"
        ).await.unwrap();
        
        assert_eq!(manifest.name, "new-pack");
        assert_eq!(manifest.version, "0.1.0");
        
        // Verify file was created
        assert!(temp_dir.path().join("phenotype.toml").exists());
    }

    #[test]
    fn test_manifest_error_display() {
        let err = ManifestError::NotFound(PathBuf::from("/missing"));
        assert!(err.to_string().contains("Manifest not found"));
        
        let err = ManifestError::Parse("invalid toml".to_string());
        assert!(err.to_string().contains("Failed to parse"));
    }
}
