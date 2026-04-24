/**
 * Skill and event types
 */

import { SkillId, Version, SkillManifest } from './types';

/**
 * A registered skill with its metadata
 */
export class Skill {
  id: SkillId;
  name: string;
  version: Version;
  manifest: SkillManifest;
  path?: string;
  readonly createdAt: Date;
  updatedAt: Date;

  private constructor(
    id: SkillId,
    name: string,
    version: Version,
    manifest: SkillManifest,
    path?: string
  ) {
    this.id = id;
    this.name = name;
    this.version = version;
    this.manifest = manifest;
    this.path = path;
    this.createdAt = new Date();
    this.updatedAt = new Date();
  }

  /**
   * Create a skill from a manifest
   */
  static fromManifest(manifest: SkillManifest, path?: string): Skill {
    const id = SkillId.new(manifest.name);
    return new Skill(id, manifest.name, manifest.version, manifest, path);
  }

  /**
   * Check if this skill is compatible with another
   */
  isCompatibleWith(other: Skill): boolean {
    return this.manifest.isCompatibleWith(other.manifest);
  }

  /**
   * Check if the skill requires a specific permission
   */
  requiresPermission(permission: string): boolean {
    return this.manifest.permissions.some((p) => p.name === permission);
  }

  /**
   * Update the manifest
   */
  updateManifest(manifest: SkillManifest): void {
    this.manifest = manifest;
    this.updatedAt = new Date();
  }

  toString(): string {
    return `${this.name}@${this.version}`;
  }
}

/**
 * Base class for skill events
 */
export abstract class SkillEvent {
  readonly timestamp: Date;

  constructor() {
    this.timestamp = new Date();
  }
}

/**
 * Event emitted when a skill is registered
 */
export class SkillRegistered extends SkillEvent {
  constructor(
    public readonly skillId: SkillId,
    public readonly name: string,
    public readonly version: string
  ) {
    super();
  }
}

/**
 * Event emitted when a skill is unregistered
 */
export class SkillUnregistered extends SkillEvent {
  constructor(public readonly skillId: SkillId) {
    super();
  }
}

/**
 * Event emitted when a skill is updated
 */
export class SkillUpdated extends SkillEvent {
  constructor(
    public readonly skillId: SkillId,
    public readonly oldVersion: string,
    public readonly newVersion: string
  ) {
    super();
  }
}

/**
 * Event emitted when a skill is executed
 */
export class SkillExecuted extends SkillEvent {
  constructor(
    public readonly skillId: SkillId,
    public readonly success: boolean,
    public readonly durationMs: number
  ) {
    super();
  }
}

/**
 * Event emitted when a dependency is resolved
 */
export class DependencyResolved extends SkillEvent {
  constructor(
    public readonly skillId: SkillId,
    public readonly dependencyName: string,
    public readonly resolvedVersion: string
  ) {
    super();
  }
}
