//! Manifest validation

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, warn};
use semver::Version;

use crate::types::{ValidationResult, PackManifest, AssetSpec, DependencySpec};

/// Manifest validator
pub struct ManifestValidator;

impl ManifestValidator {
    /// Create a new manifest validator
    pub fn new() -> Self {
        Self
    }

    /// Validate a pack at the given path
    pub async fn validate(&self, pack_path: impl AsRef<Path>) -> ValidationResult {
        let path = pack_path.as_ref();
        
        debug!("Validating pack at: {}", path.display());

        // Find the manifest
        let manifest_path = if path.is_dir() {
            path.join(PackManifest::file_name())
        } else {
            path.to_path_buf()
        };

        if !manifest_path.exists() {
            return ValidationResult::error(format!(
                "Manifest not found: {}",
                manifest_path.display()
            ));
        }

        // Read and parse the manifest
        let content = match fs::read_to_string(&manifest_path).await {
            Ok(c) => c,
            Err(e) => {
                return ValidationResult::error(format!(
                    "Failed to read manifest: {}",
                    e
                ));
            }
        };

        let manifest: PackManifest = match toml::from_str(&content) {
            Ok(m) => m,
            Err(e) => {
                return ValidationResult::error(format!(
                    "Failed to parse manifest TOML: {}",
                    e
                ));
            }
        };

        // Perform validations
        let mut result = ValidationResult::success();

        // Validate required fields
        self.validate_required_fields(&manifest, &mut result);
        
        // Validate version format
        self.validate_version(&manifest, &mut result);
        
        // Validate asset names are unique
        self.validate_unique_assets(&manifest, &mut result);
        
        // Validate asset paths exist (if pack is a directory)
        if path.is_dir() {
            self.validate_asset_paths(path, &manifest, &mut result).await;
        }
        
        // Validate dependencies
        self.validate_dependencies(&manifest, &mut result);

        if result.valid {
            info!("Manifest validation passed: {}", manifest_path.display());
        } else {
            warn!("Manifest validation failed: {}", manifest_path.display());
        }

        result
    }

    /// Validate required fields
    fn validate_required_fields(&self, manifest: &PackManifest, result: &mut ValidationResult) {
        if manifest.name.is_empty() {
            result.add_error("Pack name is required");
        }

        if manifest.name.len() > 100 {
            result.add_warning("Pack name is very long (>100 characters)");
        }

        // Check for valid characters in name
        let invalid_chars: Vec<char> = manifest.name.chars()
            .filter(|c| !c.is_alphanumeric() && !matches!(c, '-' | '_' | '.'))
            .collect();
        
        if !invalid_chars.is_empty() {
            result.add_error(format!(
                "Pack name contains invalid characters: {:?}",
                invalid_chars
            ));
        }

        if manifest.version.is_empty() {
            result.add_error("Pack version is required");
        }
    }

    /// Validate version format
    fn validate_version(&self, manifest: &PackManifest, result: &mut ValidationResult) {
        match Version::parse(&manifest.version) {
            Ok(_) => {
                // Valid semver
            }
            Err(e) => {
                result.add_error(format!(
                    "Invalid version format '{}': {}. Must follow semantic versioning (e.g., 1.0.0)",
                    manifest.version,
                    e
                ));
            }
        }
    }

    /// Validate unique asset names
    fn validate_unique_assets(&self, manifest: &PackManifest, result: &mut ValidationResult) {
        let mut seen_names = HashSet::new();
        let mut duplicates = Vec::new();

        for asset in &manifest.assets {
            if !seen_names.insert(&asset.name) {
                duplicates.push(asset.name.clone());
            }
        }

        if !duplicates.is_empty() {
            result.add_error(format!(
                "Duplicate asset names found: {}",
                duplicates.join(", ")
            ));
        }
    }

    /// Validate asset paths exist
    async fn validate_asset_paths(
        &self,
        pack_dir: &Path,
        manifest: &PackManifest,
        result: &mut ValidationResult,
    ) {
        for asset in &manifest.assets {
            let asset_path = pack_dir.join(&asset.path);
            
            if !asset_path.exists() {
                result.add_error(format!(
                    "Asset path does not exist: {}",
                    asset.path
                ));
            } else if !asset_path.is_file() {
                result.add_error(format!(
                    "Asset path is not a file: {}",
                    asset.path
                ));
            }
        }
    }

    /// Validate dependencies
    fn validate_dependencies(&self, manifest: &PackManifest, result: &mut ValidationResult) {
        let mut seen_deps = HashSet::new();

        for dep in &manifest.dependencies {
            // Check for duplicate dependencies
            if !seen_deps.insert(&dep.name) {
                result.add_warning(format!(
                    "Duplicate dependency: {}",
                    dep.name
                ));
            }

            // Validate version constraint (basic check)
            if dep.version_constraint.is_empty() {
                result.add_warning(format!(
                    "Empty version constraint for dependency: {}",
                    dep.name
                ));
            }

            // Check for wildcard-only constraints (generally not recommended)
            if dep.version_constraint.trim() == "*" {
                result.add_warning(format!(
                    "Wildcard version constraint for {} may cause issues",
                    dep.name
                ));
            }
        }

        // Warn if there are many dependencies
        if manifest.dependencies.len() > 50 {
            result.add_warning("Pack has a large number of dependencies (>50)");
        }
    }
}

impl Default for ManifestValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_validate_valid_manifest() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test-pack"
version = "1.0.0"
author = "Author"
description = "A test pack"

[[assets]]
name = "main"
path = "main.py"
type = "python_script"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        fs::write(temp_dir.path().join("main.py"), "# main").await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        assert!(result.valid, "Validation failed: {:?}", result.errors);
    }

    #[tokio::test]
    async fn test_validate_missing_name() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
version = "1.0.0"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        assert!(!result.valid);
        // The TOML deserialization might fail, or the name might be empty
        // Either way, there should be an error about the name
        assert!(
            result.errors.iter().any(|e| e.contains("Pack name is required")) ||
            result.errors.iter().any(|e| e.contains("TOML")),
            "Expected error about name. Got: {:?}", result.errors
        );
    }

    #[tokio::test]
    async fn test_validate_invalid_version() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test-pack"
version = "not-a-version"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("version")));
    }

    #[tokio::test]
    async fn test_validate_duplicate_assets() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test-pack"
version = "1.0.0"

[[assets]]
name = "asset1"
path = "a.py"
type = "python_script"

[[assets]]
name = "asset1"
path = "b.py"
type = "python_script"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Duplicate")));
    }

    #[tokio::test]
    async fn test_validate_missing_asset_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test-pack"
version = "1.0.0"

[[assets]]
name = "missing"
path = "nonexistent.py"
type = "python_script"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("does not exist")));
    }

    #[tokio::test]
    async fn test_validate_manifest_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test-pack"
version = "1.0.0"
"#;
        let manifest_path = temp_dir.path().join("phenotype.toml");
        fs::write(&manifest_path, manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(&manifest_path).await;
        
        // When passing the file directly, asset path validation is skipped
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_validate_invalid_toml() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        fs::write(temp_dir.path().join("phenotype.toml"), "not valid toml!!!").await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("TOML")));
    }

    #[tokio::test]
    async fn test_validate_invalid_characters_in_name() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test@pack!"
version = "1.0.0"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("invalid characters")));
    }

    #[tokio::test]
    async fn test_validate_long_name_warning() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let long_name = "a".repeat(150);
        let manifest = format!(r#"
name = "{}"
version = "1.0.0"
"#, long_name);
        
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        // Should have warning but still be valid
        assert!(result.warnings.iter().any(|w| w.contains("long")));
    }

    #[tokio::test]
    async fn test_validate_duplicate_dependencies() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test-pack"
version = "1.0.0"

[[dependencies]]
name = "base"
version_constraint = ">=1.0.0"

[[dependencies]]
name = "base"
version_constraint = ">=2.0.0"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        // Should have warning but still be valid
        assert!(result.valid);
        assert!(result.warnings.iter().any(|w| w.contains("Duplicate dependency")));
    }

    #[tokio::test]
    async fn test_validate_wildcard_dependency() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test-pack"
version = "1.0.0"

[[dependencies]]
name = "any"
version_constraint = "*"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let validator = ManifestValidator::new();
        let result = validator.validate(temp_dir.path()).await;
        
        assert!(result.valid);
        assert!(result.warnings.iter().any(|w| w.contains("Wildcard")));
    }
}
