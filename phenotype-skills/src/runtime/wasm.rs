//! WASM runtime for executing WebAssembly skills

use crate::domain::{Skill, Runtime};
use crate::{SkillsError, Result};
use tracing::debug;

/// WASM runtime using wasmtime
#[derive(Debug)]
pub struct WasmRuntime;

impl WasmRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Execute a WASM skill
    pub fn execute(&self, skill: &Skill, input: serde_json::Value) -> Result<serde_json::Value> {
        debug!("Executing WASM skill {} with input {}", skill.id, input);

        // Placeholder - would use wasmtime here
        Err(SkillsError::ExecutionError(
            "WASM runtime not yet implemented - requires wasmtime integration".to_string()
        ))
    }

    /// Check if can execute WASM
    pub fn can_execute(&self, skill: &Skill) -> bool {
        skill.manifest.runtime == Runtime::Wasm
    }
}

impl Default for WasmRuntime {
    fn default() -> Self {
        Self::new()
    }
}
