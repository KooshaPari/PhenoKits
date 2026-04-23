/**
 * Core types for the skills system
 */

/**
 * Runtime environment for skill execution
 */
export enum Runtime {
  WASM = 'wasm',
  PYTHON = 'python',
  JAVASCRIPT = 'javascript',
  RUST = 'rust',
  CSHARP = 'csharp',
  GO = 'go',
  SHELL = 'shell',
  BINARY = 'binary',
  CUSTOM = 'custom',
}

/**
 * Unique identifier for a skill
 */
export class SkillId {
  private constructor(public readonly value: string) {}

  static new(value: string): SkillId {
    return new SkillId(value);
  }

  toString(): string {
    return this.value;
  }

  equals(other: SkillId): boolean {
    return this.value === other.value;
  }
}

/**
 * Semantic version
 */
export class Version {
  constructor(
    public readonly major: number,
    public readonly minor: number,
    public readonly patch: number,
    public readonly prerelease?: string
  ) {}

  static parse(version: string): Version {
    const parts = version.split('.');
    if (parts.length !== 3) {
      throw new Error(`Invalid version: ${version}`);
    }
    return new Version(
      parseInt(parts[0], 10),
      parseInt(parts[1], 10),
      parseInt(parts[2], 10)
    );
  }

  static new(major: number, minor: number, patch: number): Version {
    return new Version(major, minor, patch);
  }

  satisfies(constraint: string): boolean {
    if (constraint.startsWith('^')) {
      const base = Version.parse(constraint.slice(1));
      return this.major === base.major && this.compareTo(base) >= 0;
    } else if (constraint.startsWith('~')) {
      const base = Version.parse(constraint.slice(1));
      return (
        this.major === base.major &&
        this.minor === base.minor &&
        this.compareTo(base) >= 0
      );
    } else if (constraint.startsWith('>=')) {
      const base = Version.parse(constraint.slice(2));
      return this.compareTo(base) >= 0;
    } else {
      const base = Version.parse(constraint);
      return this.equals(base);
    }
  }

  compareTo(other: Version): number {
    const cmp = this.major - other.major;
    if (cmp !== 0) return cmp;

    const cmp2 = this.minor - other.minor;
    if (cmp2 !== 0) return cmp2;

    return this.patch - other.patch;
  }

  equals(other: Version): boolean {
    return (
      this.major === other.major &&
      this.minor === other.minor &&
      this.patch === other.patch
    );
  }

  toString(): string {
    if (this.prerelease) {
      return `${this.major}.${this.minor}.${this.patch}-${this.prerelease}`;
    }
    return `${this.major}.${this.minor}.${this.patch}`;
  }
}

/**
 * Permission for skill execution
 */
export class Permission {
  constructor(
    public readonly name: string,
    public readonly description: string = ''
  ) {}

  toString(): string {
    return this.name;
  }
}

/**
 * Skill dependency specification
 */
export class SkillDependency {
  constructor(
    public readonly name: string,
    public readonly versionConstraint: string,
    public readonly optional: boolean = false
  ) {}

  static new(name: string, versionConstraint: string): SkillDependency {
    return new SkillDependency(name, versionConstraint);
  }

  static optional(name: string, versionConstraint: string): SkillDependency {
    return new SkillDependency(name, versionConstraint, true);
  }

  toString(): string {
    return `${this.name}@${this.versionConstraint}`;
  }
}

/**
 * Skill manifest options
 */
export interface SkillManifestOptions {
  name: string;
  version: string | Version;
  runtime: Runtime;
  entryPoint: string;
  description?: string;
  author?: string;
  permissions?: Permission[];
  dependencies?: SkillDependency[];
  config?: Record<string, unknown>;
}

/**
 * Skill manifest - definition of a skill
 */
export class SkillManifest {
  name: string;
  version: Version;
  runtime: Runtime;
  entryPoint: string;
  description?: string;
  author?: string;
  permissions: Permission[];
  dependencies: SkillDependency[];
  config: Record<string, unknown>;

  constructor(options: SkillManifestOptions) {
    this.name = options.name;
    this.version =
      options.version instanceof Version
        ? options.version
        : Version.parse(options.version);
    this.runtime = options.runtime;
    this.entryPoint = options.entryPoint;
    this.description = options.description;
    this.author = options.author;
    this.permissions = options.permissions ?? [];
    this.dependencies = options.dependencies ?? [];
    this.config = options.config ?? {};
  }

  isCompatibleWith(other: SkillManifest): boolean {
    return this.runtime === other.runtime;
  }
}
