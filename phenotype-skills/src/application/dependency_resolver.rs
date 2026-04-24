//! Dependency Resolver - Resolves skill dependencies

use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;

use crate::domain::{Skill, SkillId, SkillDependency};
use crate::{SkillsError, Result};

/// Dependency resolver for skill graphs
pub struct DependencyResolver {
    /// Cache of resolved versions
    version_cache: HashMap<String, Vec<Skill>>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            version_cache: HashMap::new(),
        }
    }

    /// Resolve all dependencies for a set of skills
    pub fn resolve(&self, skills: &[Skill]) -> Result<Vec<Skill>> {
        if skills.is_empty() {
            return Ok(vec![]);
        }

        // Build dependency graph
        let mut graph: DiGraph<&SkillId, ()> = DiGraph::new();
        let mut node_indices: HashMap<&SkillId, NodeIndex> = HashMap::new();

        // Add nodes
        for skill in skills {
            let idx = graph.add_node(&skill.id);
            node_indices.insert(&skill.id, idx);
        }

        // Add edges based on dependencies
        for skill in skills {
            let Some(&from_idx) = node_indices.get(&skill.id) else {
                continue;
            };

            for dep in &skill.manifest.dependencies {
                let dep_id = SkillId::new(&dep.name);

                // Find the skill that provides this dependency
                if let Some(&to_idx) = node_indices.get(&dep_id) {
                    graph.add_edge(from_idx, to_idx, ());
                }
            }
        }

        // Topological sort
        let sorted = toposort(&graph, None)
            .map_err(|_| SkillsError::DependencyError("Circular dependency detected".to_string()))?;

        // Build result in dependency order (reverse to get dependencies first)
        let mut result = Vec::new();
        for idx in sorted {
            if let Some(skill_id) = graph.node_weight(idx) {
                if let Some(skill) = skills.iter().find(|s| s.id == **skill_id) {
                    result.push(skill.clone());
                }
            }
        }

        // Reverse to get proper dependency order (dependencies before dependents)
        result.reverse();

        Ok(result)
    }

    /// Check for circular dependencies
    pub fn check_circular(&self, skills: &[Skill]) -> Result<()> {
        let mut visited: HashSet<&SkillId> = HashSet::new();
        let mut recursion_stack: HashSet<&SkillId> = HashSet::new();

        for skill in skills {
            if self.has_circular_dep(skill, skills, &mut visited, &mut recursion_stack)? {
                return Err(SkillsError::DependencyError(
                    format!("Circular dependency found involving skill: {}", skill.id)
                ));
            }
        }

        Ok(())
    }

    fn has_circular_dep<'a>(
        &self,
        skill: &'a Skill,
        all_skills: &'a [Skill],
        visited: &mut HashSet<&'a SkillId>,
        recursion_stack: &mut HashSet<&'a SkillId>,
    ) -> crate::Result<bool> {
        if recursion_stack.contains(&skill.id) {
            return Ok(true);
        }

        if visited.contains(&skill.id) {
            return Ok(false);
        }

        visited.insert(&skill.id);
        recursion_stack.insert(&skill.id);

        for dep in &skill.manifest.dependencies {
            let dep_id = SkillId::new(&dep.name);

            if let Some(dep_skill) = all_skills.iter().find(|s| s.id == dep_id) {
                if self.has_circular_dep(dep_skill, all_skills, visited, recursion_stack)? {
                    return Ok(true);
                }
            }
        }

        recursion_stack.remove(&skill.id);
        Ok(false)
    }

    /// Get all dependencies for a skill (recursively)
    pub fn get_all_dependencies(&self, skill: &Skill, all_skills: &[Skill]) -> Vec<Skill> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        self.collect_deps_recursive(skill, all_skills, &mut visited, &mut result);

        result
    }

    fn collect_deps_recursive(
        &self,
        skill: &Skill,
        all_skills: &[Skill],
        visited: &mut HashSet<SkillId>,
        result: &mut Vec<Skill>,
    ) {
        for dep in &skill.manifest.dependencies {
            let dep_id = SkillId::new(&dep.name);

            if visited.contains(&dep_id) {
                continue;
            }

            if let Some(dep_skill) = all_skills.iter().find(|s| s.id == dep_id) {
                visited.insert(dep_id.clone());
                result.push(dep_skill.clone());

                // Recursively collect transitive dependencies
                self.collect_deps_recursive(dep_skill, all_skills, visited, result);
            }
        }
    }

    /// Generate a DOT graph representation
    pub fn to_dot(&self, skills: &[Skill]) -> String {
        let mut graph = String::from("digraph dependencies {\n");
        graph.push_str("  rankdir=BT;\n"); // Bottom to top for dependency flow

        // Add nodes
        for skill in skills {
            graph.push_str(&format!(
                "  \"{}\" [label=\"{} v{}\"];\n",
                skill.id.as_str(),
                skill.id.as_str(),
                skill.version
            ));
        }

        // Add edges
        for skill in skills {
            for dep in &skill.manifest.dependencies {
                // Edge from dependency to the skill that depends on it
                graph.push_str(&format!(
                    "  \"{}\" -> \"{}\";\n",
                    dep.name,
                    skill.id.as_str()
                ));
            }
        }

        graph.push_str("}\n");
        graph
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_skill(name: &str, deps: Vec<SkillDependency>) -> Skill {
        let manifest = crate::domain::SkillManifest {
            name: name.to_string(),
            version: crate::domain::Version::new(1, 0, 0),
            description: None,
            author: None,
            runtime: crate::domain::Runtime::Wasm,
            entry_point: "main.wasm".to_string(),
            permissions: vec![],
            dependencies: deps,
            config: serde_json::json!({}),
        };
        crate::domain::Skill::from_manifest(manifest, None)
    }

    #[test]
    fn test_resolve_linear() {
        let resolver = DependencyResolver::new();

        let a = make_skill("a", vec![]);
        let b = make_skill("b", vec![SkillDependency::new("a", "^1.0.0")]);
        let c = make_skill("c", vec![SkillDependency::new("b", "^1.0.0")]);

        let resolved = resolver.resolve(&[c.clone(), b.clone(), a.clone()]).unwrap();

        // Should be in dependency order (a, b, c)
        assert_eq!(resolved[0].id.as_str(), "a");
        assert_eq!(resolved[1].id.as_str(), "b");
        assert_eq!(resolved[2].id.as_str(), "c");
    }

    #[test]
    fn test_circular_detection() {
        let resolver = DependencyResolver::new();

        let a = make_skill("a", vec![SkillDependency::new("b", "^1.0.0")]);
        let b = make_skill("b", vec![SkillDependency::new("a", "^1.0.0")]);

        let result = resolver.check_circular(&[a, b]);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_dependencies() {
        let resolver = DependencyResolver::new();

        let a = make_skill("a", vec![]);
        let b = make_skill("b", vec![SkillDependency::new("a", "^1.0.0")]);
        let c = make_skill("c", vec![SkillDependency::new("b", "^1.0.0")]);

        let all_skills = vec![a.clone(), b.clone(), c.clone()];
        let deps = resolver.get_all_dependencies(&c, &all_skills);

        // Should have both a and b
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn test_dot_generation() {
        let resolver = DependencyResolver::new();

        let a = make_skill("a", vec![]);
        let b = make_skill("b", vec![SkillDependency::new("a", "^1.0.0")]);

        let dot = resolver.to_dot(&[a, b]);

        assert!(dot.contains("digraph"));
        // Edge goes from dependency (a) to dependent (b)
        assert!(dot.contains("\"a\" -> \"b\""));
    }
}
