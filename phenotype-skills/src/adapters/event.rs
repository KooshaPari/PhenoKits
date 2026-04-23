//! Event adapter implementations

use crate::domain::SkillEvent;
use tracing::{info, debug, warn};

/// Event port implementation using tracing
#[derive(Debug, Clone, Default)]
pub struct TracingEventPort;

impl TracingEventPort {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&self, event: SkillEvent) {
        match event {
            SkillEvent::SkillRegistered { skill_id, name, version, .. } => {
                info!("Skill registered: {} ({}) v{}", name, skill_id, version);
            }
            SkillEvent::SkillUnregistered { skill_id, .. } => {
                info!("Skill unregistered: {}", skill_id);
            }
            SkillEvent::SkillUpdated { skill_id, old_version, new_version, .. } => {
                info!("Skill updated: {} {} -> {}", skill_id, old_version, new_version);
            }
            SkillEvent::SkillExecuted { skill_id, success, duration_ms, .. } => {
                if success {
                    info!("Skill executed successfully: {} in {}ms", skill_id, duration_ms);
                } else {
                    warn!("Skill execution failed: {}", skill_id);
                }
            }
            SkillEvent::DependencyResolved { skill_id, dependency_name, resolved_version, .. } => {
                debug!("Dependency resolved: {} -> {} v{}", skill_id, dependency_name, resolved_version);
            }
        }
    }
}
