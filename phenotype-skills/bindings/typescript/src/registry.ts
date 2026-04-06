/**
 * Skill registry implementation
 */

import { SkillId, SkillManifest } from './types';
import { Skill } from './skill';

/**
 * Main registry for managing skills
 */
export class SkillRegistry {
  private readonly skills: Map<string, Skill> = new Map();

  /**
   * Register a new skill
   * @throws Error if skill is already registered
   */
  register(skill: Skill): void {
    const key = skill.id.toString();
    if (this.skills.has(key)) {
      throw new Error(`Skill already registered: ${skill.id}`);
    }
    this.skills.set(key, skill);
  }

  /**
   * Unregister a skill
   * @throws Error if skill not found
   */
  unregister(skillId: SkillId): Skill {
    const key = skillId.toString();
    const skill = this.skills.get(key);
    if (!skill) {
      throw new Error(`Skill not found: ${skillId}`);
    }
    this.skills.delete(key);
    return skill;
  }

  /**
   * Get a skill by ID
   * @throws Error if skill not found
   */
  get(skillId: SkillId): Skill {
    const key = skillId.toString();
    const skill = this.skills.get(key);
    if (!skill) {
      throw new Error(`Skill not found: ${skillId}`);
    }
    return skill;
  }

  /**
   * Try to get a skill by ID
   */
  tryGet(skillId: SkillId): Skill | undefined {
    return this.skills.get(skillId.toString());
  }

  /**
   * List all registered skills
   */
  list(): Skill[] {
    return Array.from(this.skills.values());
  }

  /**
   * Update a skill's manifest
   * @throws Error if skill not found
   */
  update(skillId: SkillId, manifest: SkillManifest): Skill {
    const key = skillId.toString();
    const skill = this.skills.get(key);
    if (!skill) {
      throw new Error(`Skill not found: ${skillId}`);
    }
    skill.updateManifest(manifest);
    return skill;
  }

  /**
   * Check if a skill exists
   */
  exists(skillId: SkillId): boolean {
    return this.skills.has(skillId.toString());
  }

  /**
   * Get the count of registered skills
   */
  get count(): number {
    return this.skills.size;
  }
}

/**
 * Builder for creating configured SkillRegistry instances
 */
export class SkillRegistryBuilder {
  /**
   * Build the registry
   */
  build(): SkillRegistry {
    return new SkillRegistry();
  }
}
