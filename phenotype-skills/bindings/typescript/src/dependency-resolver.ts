/**
 * Dependency resolver for skill graphs
 */

import { Skill } from './skill';

/**
 * Dependency resolver for skill graphs
 */
export class DependencyResolver {
  /**
   * Resolve all dependencies for a set of skills
   * @returns Skills in dependency order (dependencies first)
   */
  resolve(skills: Skill[]): Skill[] {
    if (skills.length === 0) return [];

    // Build dependency graph
    const graph = new Map<string, string[]>();
    const skillMap = new Map<string, Skill>();

    for (const skill of skills) {
      const id = skill.id.toString();
      skillMap.set(id, skill);
      const deps: string[] = [];
      for (const dep of skill.manifest.dependencies) {
        // Find skill that provides this dependency
        const provider = skills.find((s) => s.id.value === dep.name);
        if (provider) {
          deps.push(provider.id.toString());
        }
      }
      graph.set(id, deps);
    }

    // Topological sort
    const result: Skill[] = [];
    const visited = new Set<string>();
    const temp = new Set<string>();

    const visit = (id: string) => {
      if (temp.has(id)) {
        throw new Error(`Circular dependency detected involving: ${id}`);
      }
      if (visited.has(id)) return;

      temp.add(id);
      const deps = graph.get(id) ?? [];
      for (const depId of deps) {
        visit(depId);
      }
      temp.delete(id);
      visited.add(id);
      const skill = skillMap.get(id);
      if (skill) result.push(skill);
    };

    for (const skill of skills) {
      visit(skill.id.toString());
    }

    // Reverse to get dependency order (dependencies before dependents)
    result.reverse();
    return result;
  }

  /**
   * Check for circular dependencies
   * @throws Error if circular dependencies are detected
   */
  checkCircular(skills: Skill[]): void {
    // Just run resolve - it will throw on circular dependencies
    this.resolve(skills);
  }

  /**
   * Get all dependencies for a skill (recursively)
   */
  getAllDependencies(skill: Skill, allSkills: Skill[]): Skill[] {
    const result: Skill[] = [];
    const visited = new Set<string>();

    const collectDeps = (s: Skill) => {
      for (const dep of s.manifest.dependencies) {
        if (visited.has(dep.name)) continue;
        const depSkill = allSkills.find((x) => x.id.value === dep.name);
        if (depSkill) {
          visited.add(dep.name);
          result.push(depSkill);
          collectDeps(depSkill);
        }
      }
    };

    collectDeps(skill);
    return result;
  }

  /**
   * Generate a DOT graph representation
   */
  toDot(skills: Skill[]): string {
    const lines = ['digraph dependencies {', '  rankdir=BT;'];

    // Add nodes
    for (const skill of skills) {
      lines.push(`  "${skill.id}" [label="${skill.id} v${skill.version}"];`);
    }

    // Add edges
    for (const skill of skills) {
      for (const dep of skill.manifest.dependencies) {
        lines.push(`  "${dep.name}" -> "${skill.id}";`);
      }
    }

    lines.push('}');
    return lines.join('\n');
  }
}
