//! Value objects for the skills domain

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

/// Semantic version
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
        }
    }

    pub fn parse(s: &str) -> crate::Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(crate::SkillsError::InvalidVersion(format!(
                "Expected format MAJOR.MINOR.PATCH, got: {}",
                s
            )));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| crate::SkillsError::InvalidVersion(format!("Invalid major version in: {}", s)))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| crate::SkillsError::InvalidVersion(format!("Invalid minor version in: {}", s)))?;
        let patch = parts[2]
            .parse()
            .map_err(|_| crate::SkillsError::InvalidVersion(format!("Invalid patch version in: {}", s)))?;

        Ok(Self {
            major,
            minor,
            patch,
            pre: None,
        })
    }

    pub fn satisfies(&self, constraint: &str) -> bool {
        // Simplified constraint checking
        if constraint.starts_with('^') {
            // Caret: compatible with major version
            let base = &constraint[1..];
            if let Ok(base_ver) = Version::parse(base) {
                return self.major == base_ver.major && self >= &base_ver;
            }
        } else if constraint.starts_with('~') {
            // Tilde: compatible with minor version
            let base = &constraint[1..];
            if let Ok(base_ver) = Version::parse(base) {
                return self.major == base_ver.major
                    && self.minor == base_ver.minor
                    && self >= &base_ver;
            }
        } else if constraint.starts_with(">=") {
            // Greater than or equal
            let base = &constraint[2..];
            if let Ok(base_ver) = Version::parse(base) {
                return self >= &base_ver;
            }
        }
        // Exact match or other constraints
        if let Ok(req) = Version::parse(constraint) {
            return self == &req;
        }
        false
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
    }
}

/// Skill dependency specification
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SkillDependency {
    pub name: String,
    pub version_constraint: String,
    pub optional: bool,
}

impl SkillDependency {
    pub fn new(name: impl Into<String>, version_constraint: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version_constraint: version_constraint.into(),
            optional: false,
        }
    }

    pub fn optional(name: impl Into<String>, version_constraint: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version_constraint: version_constraint.into(),
            optional: true,
        }
    }
}

/// Runtime environment for skill execution
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Runtime {
    Wasm,
    Python,
    JavaScript,
    Rust,
    CSharp,
    Go,
    Shell,
    Binary,
    Custom,
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime::Wasm
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Runtime::Wasm => "wasm",
            Runtime::Python => "python",
            Runtime::JavaScript => "javascript",
            Runtime::Rust => "rust",
            Runtime::CSharp => "csharp",
            Runtime::Go => "go",
            Runtime::Shell => "shell",
            Runtime::Binary => "binary",
            Runtime::Custom => "custom",
        };
        write!(f, "{}", s)
    }
}

/// Permission for skill execution
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Permission {
    pub name: String,
    pub description: String,
}

impl Permission {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

/// Skill manifest - definition of a skill
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SkillManifest {
    pub name: String,
    pub version: Version,
    pub description: Option<String>,
    pub author: Option<String>,
    pub runtime: Runtime,
    pub entry_point: String,
    pub permissions: Vec<Permission>,
    pub dependencies: Vec<SkillDependency>,
    pub config: serde_json::Value,
}

impl SkillManifest {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        runtime: Runtime,
        entry_point: impl Into<String>,
    ) -> Self {
        let version = Version::parse(&version.into()).unwrap_or_else(|_| Version::new(0, 0, 0));

        Self {
            name: name.into(),
            version,
            description: None,
            author: None,
            runtime,
            entry_point: entry_point.into(),
            permissions: vec![],
            dependencies: vec![],
            config: serde_json::json!({}),
        }
    }

    pub fn is_compatible_with(&self, other: &SkillManifest) -> bool {
        self.runtime == other.runtime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parse() {
        let v = Version::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 2, 0);
        let v3 = Version::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

    #[test]
    fn test_version_satisfies_caret() {
        let v = Version::parse("1.2.3").unwrap();
        assert!(v.satisfies("^1.0.0"));
        assert!(!v.satisfies("^2.0.0"));
    }

    #[test]
    fn test_version_satisfies_tilde() {
        let v = Version::parse("1.2.3").unwrap();
        assert!(v.satisfies("~1.2.0"));
        assert!(!v.satisfies("~1.1.0"));
    }

    #[test]
    fn test_version_satisfies_gte() {
        let v = Version::parse("1.2.3").unwrap();
        assert!(v.satisfies(">=1.0.0"));
        assert!(v.satisfies(">=1.2.3"));
        assert!(!v.satisfies(">=2.0.0"));
    }

    #[test]
    fn test_version_to_string() {
        let v = Version::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_dependency() {
        let dep = SkillDependency::new("auth", "^1.0.0");
        assert_eq!(dep.name, "auth");
        assert!(!dep.optional);

        let optional_dep = SkillDependency::optional("logging", "~1.0.0");
        assert!(optional_dep.optional);
    }

    #[test]
    fn test_runtime_display() {
        assert_eq!(Runtime::Wasm.to_string(), "wasm");
        assert_eq!(Runtime::Python.to_string(), "python");
    }

    #[test]
    fn test_manifest_new() {
        let manifest = SkillManifest::new("test-skill", "1.0.0", Runtime::Wasm, "main.wasm");
        assert_eq!(manifest.name, "test-skill");
        assert_eq!(manifest.version.to_string(), "1.0.0");
    }
}
