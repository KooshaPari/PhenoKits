//! Sandbox adapters for skill execution

use crate::domain::{Skill, Runtime};

/// WASM sandbox adapter (Tier 1)
#[derive(Debug, Clone, Default)]
pub struct WasmSandbox;

impl WasmSandbox {
    pub fn new() -> Self {
        Self
    }

    /// Execute a skill in the WASM sandbox
    /// 
    /// This requires wasmtime to be installed:
    ///   cargo install wasmtime-cli
    pub fn execute(&self, skill: &Skill, input: serde_json::Value) -> crate::Result<serde_json::Value> {
        Err(crate::SkillsError::SandboxError(
            format!("WASM execution requires wasmtime. Install with: cargo install wasmtime-cli\n\nSkill: {}", skill.id)
        ))
    }

    /// Check if this sandbox can execute the skill
    pub fn can_execute(&self, skill: &Skill) -> bool {
        matches!(skill.manifest.runtime, Runtime::Wasm)
    }
}

/// gVisor sandbox adapter (Tier 2)
#[derive(Debug, Clone, Default)]
pub struct GVisorSandbox;

impl GVisorSandbox {
    pub fn new() -> Self {
        Self
    }

    /// Execute a skill in the gVisor sandbox
    ///
    /// This requires runsc to be installed:
    ///   https://gvisor.dev/docs/user_guide/install/
    pub fn execute(&self, skill: &Skill, input: serde_json::Value) -> crate::Result<serde_json::Value> {
        Err(crate::SkillsError::SandboxError(
            format!("gVisor execution requires 'runsc' binary.\n\nInstall: https://gvisor.dev/docs/user_guide/install/\n\nSkill: {}", skill.id)
        ))
    }

    /// Check if this sandbox can execute the skill
    pub fn can_execute(&self, skill: &Skill) -> bool {
        matches!(
            skill.manifest.runtime,
            Runtime::Python | Runtime::JavaScript | Runtime::Shell
        )
    }
}

/// Firecracker sandbox adapter (Tier 3)
#[derive(Debug, Clone, Default)]
pub struct FirecrackerSandbox;

impl FirecrackerSandbox {
    pub fn new() -> Self {
        Self
    }

    /// Execute a skill in the Firecracker sandbox
    ///
    /// This requires the Firecracker binary to be installed:
    ///   https://github.com/firecracker-microvm/firecracker/releases
    pub fn execute(&self, skill: &Skill, input: serde_json::Value) -> crate::Result<serde_json::Value> {
        Err(crate::SkillsError::SandboxError(
            format!("Firecracker execution requires 'firecracker' binary.\n\nInstall: https://github.com/firecracker-microvm/firecracker/releases\n\nSkill: {}", skill.id)
        ))
    }

    /// Check if this sandbox can execute any skill
    pub fn can_execute(&self, _skill: &Skill) -> bool {
        true
    }
}

/// Unified sandbox that selects the appropriate tier
#[derive(Debug, Clone, Default)]
pub struct UnifiedSandbox {
    wasm: WasmSandbox,
    gvisor: GVisorSandbox,
    firecracker: FirecrackerSandbox,
}

impl UnifiedSandbox {
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute a skill using the most appropriate sandbox
    pub fn execute(&self, skill: &Skill, input: serde_json::Value) -> crate::Result<serde_json::Value> {
        // Select sandbox based on runtime requirements
        if self.wasm.can_execute(skill) {
            self.wasm.execute(skill, input)
        } else if self.gvisor.can_execute(skill) {
            self.gvisor.execute(skill, input)
        } else {
            // Fall back to Firecracker for unknown runtimes
            self.firecracker.execute(skill, input)
        }
    }

    /// Check if any sandbox can execute this skill
    pub fn can_execute(&self, skill: &Skill) -> bool {
        self.wasm.can_execute(skill) 
            || self.gvisor.can_execute(skill) 
            || self.firecracker.can_execute(skill)
    }

    /// Get the recommended sandbox for a skill
    pub fn recommended_tier(&self, skill: &Skill) -> &'static str {
        if self.wasm.can_execute(skill) {
            "Tier 1 (WASM - Fastest, ~1ms startup)"
        } else if self.gvisor.can_execute(skill) {
            "Tier 2 (gVisor - ~90ms startup)"
        } else {
            "Tier 3 (Firecracker - ~125ms startup, full isolation)"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SkillManifest;

    fn make_skill(runtime: Runtime) -> Skill {
        let manifest = SkillManifest {
            name: "test".to_string(),
            version: crate::domain::Version::new(1, 0, 0),
            description: None,
            author: None,
            runtime,
            entry_point: "main".to_string(),
            permissions: vec![],
            dependencies: vec![],
            config: serde_json::json!({}),
        };
        crate::domain::Skill::from_manifest(manifest, None)
    }

    #[test]
    fn test_wasm_sandbox() {
        let sandbox = WasmSandbox::new();
        let skill = make_skill(Runtime::Wasm);

        assert!(sandbox.can_execute(&skill));
    }

    #[test]
    fn test_wasm_sandbox_rejects_python() {
        let sandbox = WasmSandbox::new();
        let skill = make_skill(Runtime::Python);

        assert!(!sandbox.can_execute(&skill));
    }

    #[test]
    fn test_gvisor_sandbox() {
        let sandbox = GVisorSandbox::new();
        let skill = make_skill(Runtime::Python);

        assert!(sandbox.can_execute(&skill));
    }

    #[test]
    fn test_firecracker_sandbox_universal() {
        let sandbox = FirecrackerSandbox::new();
        let skill = make_skill(Runtime::Custom);

        assert!(sandbox.can_execute(&skill));
    }

    #[test]
    fn test_unified_sandbox_selection() {
        let unified = UnifiedSandbox::new();

        let wasm_skill = make_skill(Runtime::Wasm);
        let python_skill = make_skill(Runtime::Python);
        let rust_skill = make_skill(Runtime::Rust);

        // WASM skill -> Tier 1
        assert!(unified.wasm.can_execute(&wasm_skill));
        assert_eq!(unified.recommended_tier(&wasm_skill), "Tier 1 (WASM - Fastest, ~1ms startup)");

        // Python skill -> Tier 2
        assert!(unified.gvisor.can_execute(&python_skill));
        assert_eq!(unified.recommended_tier(&python_skill), "Tier 2 (gVisor - ~90ms startup)");

        // Rust/unknown -> Tier 3
        assert!(!unified.wasm.can_execute(&rust_skill));
        assert!(!unified.gvisor.can_execute(&rust_skill));
        assert!(unified.firecracker.can_execute(&rust_skill));
        assert_eq!(unified.recommended_tier(&rust_skill), "Tier 3 (Firecracker - ~125ms startup, full isolation)");
    }
}