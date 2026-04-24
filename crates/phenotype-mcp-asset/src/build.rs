//! Pack building functionality

use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, warn, error};
use sha2::{Sha256, Digest};

use crate::types::{BuildResult, PackManifest, AssetSpec};

/// Pack builder
pub struct PackBuilder {
    root_dir: PathBuf,
}

impl PackBuilder {
    /// Create a new pack builder
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    /// Build a pack
    pub async fn build(
        &self,
        source_dir: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<BuildResult, BuildError> {
        let source = source_dir.as_ref();
        let output = output_path.as_ref();

        info!("Building pack from: {} to: {}", source.display(), output.display());

        // Read the manifest
        let manifest_path = source.join(PackManifest::file_name());
        if !manifest_path.exists() {
            return Err(BuildError::ManifestNotFound(manifest_path));
        }

        let manifest_content = fs::read_to_string(&manifest_path).await
            .map_err(|e| BuildError::Io(e))?;
        
        let manifest: PackManifest = toml::from_str(&manifest_content)
            .map_err(|e| BuildError::ManifestParse(e.to_string()))?;

        debug!("Building pack: {} v{}", manifest.name, manifest.version);

        // Validate all assets exist
        let mut result = BuildResult::success(output);
        let mut missing_assets = Vec::new();

        for asset in &manifest.assets {
            let asset_path = source.join(&asset.path);
            if !asset_path.exists() {
                missing_assets.push(asset.name.clone());
                result.add_error(format!("Asset not found: {}", asset.path));
            } else {
                result = result.add_artifact(format!("{}/{}", manifest.name, asset.path));
            }
        }

        if !missing_assets.is_empty() {
            return Ok(result);
        }

        // Create output directory if needed
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).await.map_err(|e| BuildError::Io(e))?;
        }

        // Create the pack archive
        match self.create_archive(source, output, &manifest).await {
            Ok(_) => {
                info!("Pack built successfully: {}", output.display());
                Ok(result)
            }
            Err(e) => {
                error!("Failed to create archive: {}", e);
                Ok(BuildResult::error(format!("Archive creation failed: {}", e)))
            }
        }
    }

    /// Create a pack archive
    async fn create_archive(
        &self,
        source: &Path,
        output: &Path,
        manifest: &PackManifest,
    ) -> Result<(), BuildError> {
        // For now, we create a simple directory-based pack
        // In production, this would create a tar.gz or zip archive
        
        // Create output directory
        fs::create_dir_all(output).await.map_err(|e| BuildError::Io(e))?;

        // Copy manifest
        let manifest_source = source.join(PackManifest::file_name());
        let manifest_dest = output.join(PackManifest::file_name());
        fs::copy(&manifest_source, &manifest_dest).await.map_err(|e| BuildError::Io(e))?;

        // Copy all assets
        for asset in &manifest.assets {
            let src = source.join(&asset.path);
            let dst = output.join(&asset.path);
            
            // Create parent directory
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent).await.map_err(|e| BuildError::Io(e))?;
            }
            
            fs::copy(&src, &dst).await.map_err(|e| BuildError::Io(e))?;
            debug!("Copied asset: {} -> {}", src.display(), dst.display());
        }

        // Generate checksums file
        self.generate_checksums(output, manifest).await?;

        Ok(())
    }

    /// Generate checksums for all assets
    async fn generate_checksums(
        &self,
        pack_dir: &Path,
        manifest: &PackManifest,
    ) -> Result<(), BuildError> {
        let mut checksums = Vec::new();

        // Add manifest checksum
        let manifest_path = pack_dir.join(PackManifest::file_name());
        if let Ok(content) = fs::read(&manifest_path).await {
            let hash = compute_checksum(&content);
            checksums.push(format!("{}  {}", hash, PackManifest::file_name()));
        }

        // Add asset checksums
        for asset in &manifest.assets {
            let asset_path = pack_dir.join(&asset.path);
            if let Ok(content) = fs::read(&asset_path).await {
                let hash = compute_checksum(&content);
                checksums.push(format!("{}  {}", hash, asset.path));
            }
        }

        // Write checksums file
        let checksums_content = checksums.join("\n");
        let checksums_path = pack_dir.join("CHECKSUMS.sha256");
        fs::write(&checksums_path, checksums_content).await
            .map_err(|e| BuildError::Io(e))?;

        debug!("Generated checksums file: {}", checksums_path.display());

        Ok(())
    }
}

/// Compute SHA-256 checksum
fn compute_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Build error types
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Manifest not found: {0}")]
    ManifestNotFound(PathBuf),
    
    #[error("Failed to parse manifest: {0}")]
    ManifestParse(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Asset not found: {0}")]
    AssetNotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_build_success() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("source");
        let output = temp_dir.path().join("output");
        
        fs::create_dir(&source).await.unwrap();
        
        // Create manifest
        let manifest = r#"
name = "test-pack"
version = "1.0.0"

[[assets]]
name = "main"
path = "main.py"
type = "python_script"
"#;
        fs::write(source.join("phenotype.toml"), manifest).await.unwrap();
        fs::write(source.join("main.py"), "print('hello')").await.unwrap();
        
        let builder = PackBuilder::new(temp_dir.path());
        let result = builder.build(&source, &output).await.unwrap();
        
        assert!(result.success);
        assert!(output.exists());
        assert!(output.join("phenotype.toml").exists());
        assert!(output.join("main.py").exists());
        assert!(output.join("CHECKSUMS.sha256").exists());
    }

    #[tokio::test]
    async fn test_build_missing_manifest() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("source");
        let output = temp_dir.path().join("output");
        
        fs::create_dir(&source).await.unwrap();
        
        let builder = PackBuilder::new(temp_dir.path());
        let result = builder.build(&source, &output).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BuildError::ManifestNotFound(_)));
    }

    #[tokio::test]
    async fn test_build_missing_asset() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("source");
        let output = temp_dir.path().join("output");
        
        fs::create_dir(&source).await.unwrap();
        
        // Create manifest referencing non-existent asset
        let manifest = r#"
name = "test-pack"
version = "1.0.0"

[[assets]]
name = "missing"
path = "missing.py"
type = "python_script"
"#;
        fs::write(source.join("phenotype.toml"), manifest).await.unwrap();
        
        let builder = PackBuilder::new(temp_dir.path());
        let result = builder.build(&source, &output).await.unwrap();
        
        assert!(!result.success);
        assert!(result.errors.iter().any(|e| e.contains("not found")));
    }

    #[tokio::test]
    async fn test_build_multiple_assets() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("source");
        let output = temp_dir.path().join("output");
        
        fs::create_dir(&source).await.unwrap();
        fs::create_dir(source.join("lib")).await.unwrap();
        
        // Create manifest with multiple assets
        let manifest = r#"
name = "multi-pack"
version = "1.0.0"

[[assets]]
name = "main"
path = "main.py"
type = "python_script"

[[assets]]
name = "utils"
path = "lib/utils.py"
type = "python_script"

[[assets]]
name = "config"
path = "config.json"
type = "config"
"#;
        fs::write(source.join("phenotype.toml"), manifest).await.unwrap();
        fs::write(source.join("main.py"), "# main").await.unwrap();
        fs::write(source.join("lib/utils.py"), "# utils").await.unwrap();
        fs::write(source.join("config.json"), "{}").await.unwrap();
        
        let builder = PackBuilder::new(temp_dir.path());
        let result = builder.build(&source, &output).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.artifacts.len(), 3);
        assert!(output.join("lib/utils.py").exists());
    }

    #[tokio::test]
    async fn test_checksum_computation() {
        let data = b"hello world";
        let checksum = compute_checksum(data);
        
        // SHA-256 of "hello world"
        assert_eq!(checksum, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }

    #[test]
    fn test_build_error_display() {
        let err = BuildError::ManifestNotFound(PathBuf::from("/missing.toml"));
        assert!(err.to_string().contains("Manifest not found"));
        
        let err = BuildError::ManifestParse("invalid TOML".to_string());
        assert!(err.to_string().contains("Failed to parse"));
    }
}
