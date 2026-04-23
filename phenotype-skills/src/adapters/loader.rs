//! Manifest loader adapters

use std::path::PathBuf;
use crate::domain::{SkillManifest, Runtime, Version, SkillDependency};
use crate::{SkillsError, Result};
use tracing::{debug, error};

/// TOML-based manifest loader
#[derive(Debug, Clone, Default)]
pub struct TomlLoader;

impl TomlLoader {
    pub fn new() -> Self {
        Self
    }

    /// Load a manifest from a TOML file
    pub fn load(&self, path: &PathBuf) -> Result<SkillManifest> {
        debug!("Loading manifest from {:?}", path);

        let content = std::fs::read_to_string(path)
            .map_err(|e| SkillsError::LoaderError(format!("Failed to read file: {}", e)))?;

        self.load_from_string(&content)
    }

    /// Load a manifest from a TOML string
    pub fn load_from_string(&self, content: &str) -> Result<SkillManifest> {
        let toml: TomlSkillManifest = toml::from_str(content)
            .map_err(|e| SkillsError::LoaderError(format!("Failed to parse TOML: {}", e)))?;

        let version = Version::parse(&toml.version)
            .map_err(|e| SkillsError::LoaderError(format!("Invalid version: {}", e)))?;

        let runtime = match toml.runtime.as_deref() {
            Some("wasm") => Runtime::Wasm,
            Some("python") => Runtime::Python,
            Some("javascript") | Some("js") => Runtime::JavaScript,
            Some("rust") => Runtime::Rust,
            Some("csharp") | Some("c#") => Runtime::CSharp,
            Some("go") => Runtime::Go,
            Some("shell") => Runtime::Shell,
            Some("binary") => Runtime::Binary,
            Some("custom") => Runtime::Custom,
            None => Runtime::default(),
            Some(r) => {
                return Err(SkillsError::LoaderError(format!("Unknown runtime: {}", r)));
            }
        };

        let dependencies = toml.dependencies
            .unwrap_or_default()
            .into_iter()
            .map(|d| SkillDependency {
                name: d.name,
                version_constraint: d.version,
                optional: d.optional.unwrap_or(false),
            })
            .collect();

        let permissions = toml.permissions
            .unwrap_or_default()
            .into_iter()
            .map(|p| crate::domain::Permission::new(p, ""))
            .collect();

        Ok(SkillManifest {
            name: toml.name,
            version,
            description: toml.description,
            author: toml.author,
            runtime,
            entry_point: toml.entry_point,
            permissions,
            dependencies,
            config: toml.config,
        })
    }

    /// Load multiple manifests from a directory
    pub fn load_all(&self, _dir: &PathBuf) -> Vec<SkillManifest> {
        // Implementation would scan directory and load all manifests
        vec![]
    }
}

/// TOML representation of a skill manifest
#[derive(serde::Deserialize)]
struct TomlSkillManifest {
    name: String,
    version: String,
    description: Option<String>,
    author: Option<String>,
    runtime: Option<String>,
    entry_point: String,
    permissions: Option<Vec<String>>,
    dependencies: Option<Vec<TomlDependency>>,
    #[serde(default)]
    config: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct TomlDependency {
    name: String,
    version: String,
    optional: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Runtime;

    #[test]
    fn test_load_toml() {
        let loader = TomlLoader::new();

        let toml = r#"
            name = "test-skill"
            version = "1.2.3"
            description = "A test skill"
            author = "Test Author"
            runtime = "wasm"
            entry_point = "main.wasm"
            permissions = ["filesystem", "network"]
        "#;

        let manifest = loader.load_from_string(toml).unwrap();

        assert_eq!(manifest.name, "test-skill");
        assert_eq!(manifest.version.to_string(), "1.2.3");
        assert_eq!(manifest.runtime, Runtime::Wasm);
    }

    #[test]
    fn test_load_toml_with_deps() {
        let loader = TomlLoader::new();

        let toml = r#"
            name = "test-skill"
            version = "1.2.3"
            entry_point = "main.wasm"

            [[dependencies]]
            name = "auth"
            version = "^1.0.0"

            [[dependencies]]
            name = "logging"
            version = "~2.0.0"
            optional = true
        "#;

        let manifest = loader.load_from_string(toml).unwrap();

        assert_eq!(manifest.dependencies.len(), 2);
        assert_eq!(manifest.dependencies[0].name, "auth");
        assert!(manifest.dependencies[1].optional);
    }
}
