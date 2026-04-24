//! C# runtime for executing .NET skills

use crate::domain::{Skill, Runtime};
use crate::{SkillsError, Result};
use tracing::debug;

/// C# runtime using .NET hosting
#[derive(Debug)]
pub struct CSharpRuntime;

impl CSharpRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Execute a C# skill
    pub fn execute(&self, skill: &Skill, input: serde_json::Value) -> Result<serde_json::Value> {
        debug!("Executing C# skill {} with input {}", skill.id, input);

        // Placeholder - would use nethost here
        Err(SkillsError::ExecutionError(
            "C# runtime not yet implemented - requires nethost integration".to_string()
        ))
    }

    /// Check if can execute C#
    pub fn can_execute(&self, skill: &Skill) -> bool {
        skill.manifest.runtime == Runtime::CSharp
    }
}

impl Default for CSharpRuntime {
    fn default() -> Self {
        Self::new()
    }
}
