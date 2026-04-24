//! Runtime module - Multi-language skill execution

use crate::domain::{Skill, Runtime};
use crate::Result;

mod wasm;
mod python;
mod csharp;

pub use wasm::WasmRuntime;
pub use python::PythonRuntime;
pub use csharp::CSharpRuntime;

/// Unified skill runtime dispatcher
#[derive(Debug)]
pub struct SkillRuntime {
    wasm_runtime: WasmRuntime,
    python_runtime: PythonRuntime,
    csharp_runtime: CSharpRuntime,
}

impl SkillRuntime {
    pub fn new() -> Self {
        Self {
            wasm_runtime: WasmRuntime::new(),
            python_runtime: PythonRuntime::new(),
            csharp_runtime: CSharpRuntime::new(),
        }
    }

    /// Execute a skill based on its runtime type
    pub fn execute(&self, skill: &Skill, input: serde_json::Value) -> Result<serde_json::Value> {
        match skill.manifest.runtime {
            Runtime::Wasm => self.wasm_runtime.execute(skill, input),
            Runtime::Python => self.python_runtime.execute(skill, input),
            Runtime::CSharp => self.csharp_runtime.execute(skill, input),
            _ => Err(crate::SkillsError::ExecutionError(
                format!("Runtime {:?} not yet supported", skill.manifest.runtime)
            )),
        }
    }

    /// Check if this runtime can execute the skill
    pub fn can_execute(&self, skill: &Skill) -> bool {
        match skill.manifest.runtime {
            Runtime::Wasm => self.wasm_runtime.can_execute(skill),
            Runtime::Python => self.python_runtime.can_execute(skill),
            Runtime::CSharp => self.csharp_runtime.can_execute(skill),
            _ => false,
        }
    }
}

impl Default for SkillRuntime {
    fn default() -> Self {
        Self::new()
    }
}
