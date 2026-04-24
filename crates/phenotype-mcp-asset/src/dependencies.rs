//! Dependency resolution

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, warn};
use semver::{Version, VersionReq};

use crate::types::{DependencyResolution, ResolvedDependency, DependencyConflict, PackManifest, DependencySpec};

/// Dependency resolver
pub struct DependencyResolver {
    /// Cache of resolved dependencies
    cache: HashMap<String, ResolvedDependency>,
    /// Registry URLs to search
    registries: Vec<String>,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            registries: vec!["https://registry.phenotype.dev".to_string()],
        }
    }

    /// Add a registry URL
    pub fn add_registry(mut self, url: impl Into<String>) -> Self {
        self.registries.push(url.into());
        self
    }

    /// Resolve dependencies for a pack
    pub async fn resolve(
        &self,
        pack_path: impl AsRef<Path>,
    ) -> Result<DependencyResolution, DependencyError> {
        let path = pack_path.as_ref();
        
        debug!("Resolving dependencies for: {}", path.display());

        // Read the manifest
        let manifest_path = if path.is_dir() {
            path.join(PackManifest::file_name())
        } else {
            path.to_path_buf()
        };

        if !manifest_path.exists() {
            return Err(DependencyError::ManifestNotFound(manifest_path));
        }

        let content = fs::read_to_string(&manifest_path).await
            .map_err(|e| DependencyError::Io(e))?;
        
        let manifest: PackManifest = toml::from_str(&content)
            .map_err(|e| DependencyError::ManifestParse(e.to_string()))?;

        self.resolve_manifest(&manifest).await
    }

    /// Resolve dependencies from a manifest
    pub async fn resolve_manifest(
        &self,
        manifest: &PackManifest,
    ) -> Result<DependencyResolution, DependencyError> {
        let mut resolution = DependencyResolution::new();

        info!("Resolving {} dependencies", manifest.dependencies.len());

        // Check for conflicts first
        let conflicts = self.find_conflicts(&manifest.dependencies);
        for conflict in conflicts {
            resolution.conflicts.push(conflict);
        }

        // Resolve each dependency
        for dep in &manifest.dependencies {
            match self.resolve_single(dep).await {
                Ok(resolved) => {
                    debug!("Resolved {} -> {}", dep.name, resolved.version);
                    resolution.add_resolved(resolved);
                }
                Err(e) => {
                    warn!("Failed to resolve {}: {}", dep.name, e);
                    resolution.add_unresolved(&dep.name);
                }
            }
        }

        Ok(resolution)
    }

    /// Resolve a single dependency
    async fn resolve_single(
        &self,
        dep: &DependencySpec,
    ) -> Result<ResolvedDependency, DependencyError> {
        // Check cache first
        if let Some(cached) = self.cache.get(&dep.name) {
            if self.satisfies_constraint(&cached.version, &dep.version_constraint)? {
                return Ok(cached.clone());
            }
        }

        // Try to resolve from registries
        for registry in &self.registries {
            match self.resolve_from_registry(dep, registry).await {
                Ok(resolved) => return Ok(resolved),
                Err(_) => continue,
            }
        }

        // Try local resolution
        if let Ok(resolved) = self.resolve_local(dep).await {
            return Ok(resolved);
        }

        Err(DependencyError::NotFound(dep.name.clone()))
    }

    /// Resolve from a registry
    async fn resolve_from_registry(
        &self,
        dep: &DependencySpec,
        _registry: &str,
    ) -> Result<ResolvedDependency, DependencyError> {
        // This would make an HTTP request to the registry
        // For now, return a mock resolution
        
        // Parse version constraint to extract a version
        if let Ok(version) = self.extract_version_from_constraint(&dep.version_constraint) {
            Ok(ResolvedDependency {
                name: dep.name.clone(),
                version: version.to_string(),
                path: None,
                source: "registry".to_string(),
            })
        } else {
            Err(DependencyError::VersionMismatch {
                name: dep.name.clone(),
                requested: dep.version_constraint.clone(),
                found: "none".to_string(),
            })
        }
    }

    /// Try to resolve locally
    async fn resolve_local(
        &self,
        dep: &DependencySpec,
    ) -> Result<ResolvedDependency, DependencyError> {
        // Look for a local pack with the right name
        // This would search in a local packs directory
        
        // For now, just check if we can parse the constraint
        if let Ok(version) = self.extract_version_from_constraint(&dep.version_constraint) {
            Ok(ResolvedDependency {
                name: dep.name.clone(),
                version: version.to_string(),
                path: Some(PathBuf::from(format!("./packs/{}", dep.name))),
                source: "local".to_string(),
            })
        } else {
            Err(DependencyError::NotFound(dep.name.clone()))
        }
    }

    /// Find conflicts between dependencies
    fn find_conflicts(&self, deps: &[DependencySpec]) -> Vec<DependencyConflict> {
        let mut conflicts = Vec::new();
        let mut by_name: HashMap<String, Vec<&DependencySpec>> = HashMap::new();

        // Group by name
        for dep in deps {
            by_name.entry(dep.name.clone()).or_default().push(dep);
        }

        // Check for version constraint conflicts
        for (name, versions) in by_name {
            if versions.len() > 1 {
                let constraints: Vec<String> = versions
                    .iter()
                    .map(|v| v.version_constraint.clone())
                    .collect();

                // Check if constraints are compatible
                if !self.constraints_compatible(&constraints) {
                    conflicts.push(DependencyConflict {
                        name,
                        constraints: constraints.clone(),
                        description: format!(
                            "Incompatible version constraints: {:?}",
                            constraints
                        ),
                    });
                }
            }
        }

        conflicts
    }

    /// Check if version constraints are compatible
    fn constraints_compatible(&self, constraints: &[String]) -> bool {
        // Simplified check - in reality this would use semver intersection
        // For now, just check if they have the same major version requirement
        
        let majors: Vec<Option<u64>> = constraints
            .iter()
            .map(|c| self.extract_major_version(c))
            .collect();

        // If all have same major version requirement, assume compatible
        if majors.len() > 1 {
            let first = &majors[0];
            majors.iter().all(|m| m == first)
        } else {
            true
        }
    }

    /// Check if a version satisfies a constraint
    fn satisfies_constraint(
        &self,
        version: &str,
        constraint: &str,
    ) -> Result<bool, DependencyError> {
        let ver = Version::parse(version)
            .map_err(|e| DependencyError::InvalidVersion(e.to_string()))?;
        let req = VersionReq::parse(constraint)
            .map_err(|e| DependencyError::InvalidConstraint(e.to_string()))?;
        
        Ok(req.matches(&ver))
    }

    /// Extract version from constraint (simplified)
    fn extract_version_from_constraint(&self, constraint: &str) -> Result<Version, DependencyError> {
        // Try to extract a version from the constraint
        // This is a simplified version - real implementation would be more complex
        
        let trimmed = constraint.trim_start_matches(|c| c == '=' || c == '>' || c == '<' || c == '~' || c == '^' || c == '*');
        
        if trimmed == "*" || trimmed.is_empty() {
            // Return a default version for wildcards
            Ok(Version::parse("1.0.0").unwrap())
        } else {
            Version::parse(trimmed)
                .map_err(|e| DependencyError::InvalidConstraint(e.to_string()))
        }
    }

    /// Extract major version from constraint
    fn extract_major_version(&self, constraint: &str) -> Option<u64> {
        if let Ok(version) = self.extract_version_from_constraint(constraint) {
            Some(version.major)
        } else {
            None
        }
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Dependency resolution errors
#[derive(Debug, thiserror::Error)]
pub enum DependencyError {
    #[error("Manifest not found: {0}")]
    ManifestNotFound(PathBuf),
    
    #[error("Failed to parse manifest: {0}")]
    ManifestParse(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Dependency not found: {0}")]
    NotFound(String),
    
    #[error("Version mismatch for {name}: requested {requested}, found {found}")]
    VersionMismatch { name: String, requested: String, found: String },
    
    #[error("Invalid version: {0}")]
    InvalidVersion(String),
    
    #[error("Invalid version constraint: {0}")]
    InvalidConstraint(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_resolve_empty_dependencies() {
        let resolver = DependencyResolver::new();
        
        let manifest = PackManifest::new("test", "1.0.0");
        let resolution = resolver.resolve_manifest(&manifest).await.unwrap();
        
        assert!(resolution.fully_resolved());
        assert!(resolution.resolved.is_empty());
    }

    #[tokio::test]
    async fn test_resolve_single_dependency() {
        let resolver = DependencyResolver::new();
        
        let manifest = PackManifest::new("test", "1.0.0")
            .add_dependency(DependencySpec::new("base", ">=1.0.0"));
        
        let resolution = resolver.resolve_manifest(&manifest).await.unwrap();
        
        assert_eq!(resolution.resolved.len(), 1);
        assert_eq!(resolution.resolved[0].name, "base");
    }

    #[tokio::test]
    async fn test_resolve_multiple_dependencies() {
        let resolver = DependencyResolver::new();
        
        let manifest = PackManifest::new("test", "1.0.0")
            .add_dependency(DependencySpec::new("dep1", ">=1.0.0"))
            .add_dependency(DependencySpec::new("dep2", "^2.0.0"));
        
        let resolution = resolver.resolve_manifest(&manifest).await.unwrap();
        
        assert_eq!(resolution.resolved.len(), 2);
        assert!(resolution.fully_resolved());
    }

    #[tokio::test]
    async fn test_find_conflicts() {
        let resolver = DependencyResolver::new();
        
        let deps = vec![
            DependencySpec::new("base", ">=1.0.0"),
            DependencySpec::new("base", ">=2.0.0"),
        ];
        
        let conflicts = resolver.find_conflicts(&deps);
        
        // Should detect conflict between >=1.0.0 and >=2.0.0
        assert!(!conflicts.is_empty());
    }

    #[tokio::test]
    async fn test_satisfies_constraint() {
        let resolver = DependencyResolver::new();
        
        assert!(resolver.satisfies_constraint("1.5.0", ">=1.0.0").unwrap());
        assert!(resolver.satisfies_constraint("2.0.0", ">=1.0.0").unwrap());
        assert!(!resolver.satisfies_constraint("0.9.0", ">=1.0.0").unwrap());
        assert!(resolver.satisfies_constraint("1.2.3", "~1.2.0").unwrap());
        assert!(!resolver.satisfies_constraint("1.3.0", "~1.2.0").unwrap());
    }

    #[tokio::test]
    async fn test_extract_version_from_constraint() {
        let resolver = DependencyResolver::new();
        
        let v = resolver.extract_version_from_constraint(">=1.0.0").unwrap();
        assert_eq!(v, Version::parse("1.0.0").unwrap());
        
        let v = resolver.extract_version_from_constraint("^2.5.0").unwrap();
        assert_eq!(v, Version::parse("2.5.0").unwrap());
        
        // Wildcard returns default
        let v = resolver.extract_version_from_constraint("*").unwrap();
        assert_eq!(v, Version::parse("1.0.0").unwrap());
    }

    #[tokio::test]
    async fn test_resolve_manifest_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let manifest = r#"
name = "test-pack"
version = "1.0.0"

[[dependencies]]
name = "library"
version_constraint = ">=1.0.0"
"#;
        fs::write(temp_dir.path().join("phenotype.toml"), manifest).await.unwrap();
        
        let resolver = DependencyResolver::new();
        let resolution = resolver.resolve(temp_dir.path()).await.unwrap();
        
        assert_eq!(resolution.resolved.len(), 1);
        assert_eq!(resolution.resolved[0].name, "library");
    }

    #[test]
    fn test_dependency_error_display() {
        let err = DependencyError::NotFound("my-dep".to_string());
        assert!(err.to_string().contains("not found"));
        
        let err = DependencyError::InvalidVersion("bad".to_string());
        assert!(err.to_string().contains("Invalid version"));
        
        let err = DependencyError::InvalidConstraint("bad".to_string());
        assert!(err.to_string().contains("Invalid version constraint"));
    }
}
