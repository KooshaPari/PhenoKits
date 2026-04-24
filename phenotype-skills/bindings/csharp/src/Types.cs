using System;
using System.Collections.Generic;
using System.Linq;

namespace Phenotype.Skills;

/// <summary>
/// Unique identifier for a skill
/// </summary>
public readonly struct SkillId : IEquatable<SkillId>
{
    public string Value { get; }

    public SkillId(string value)
    {
        if (string.IsNullOrEmpty(value))
            throw new ArgumentException("SkillId cannot be null or empty", nameof(value));
        Value = value;
    }

    public static SkillId NewId(string value) => new(value);
    public override string ToString() => Value;
    public bool Equals(SkillId other) => Value == other.Value;
    public override bool Equals(object? obj) => obj is SkillId other && Equals(other);
    public override int GetHashCode() => Value.GetHashCode();
    public static bool operator ==(SkillId left, SkillId right) => left.Equals(right);
    public static bool operator !=(SkillId left, SkillId right) => !left.Equals(right);
}

/// <summary>
/// Semantic version for skills
/// </summary>
public readonly struct SemVersion : IComparable<SemVersion>, IEquatable<SemVersion>
{
    public uint Major { get; }
    public uint Minor { get; }
    public uint Patch { get; }
    public string? Prerelease { get; }

    public SemVersion(uint major, uint minor, uint patch, string? prerelease = null)
    {
        Major = major;
        Minor = minor;
        Patch = patch;
        Prerelease = prerelease;
    }

    public static SemVersion ParseVersion(string version)
    {
        var parts = version.Split('.', 3);
        if (parts.Length < 3)
            throw new ArgumentException("Version must be in format MAJOR.MINOR.PATCH", nameof(version));

        return new SemVersion(
            uint.Parse(parts[0]),
            uint.Parse(parts[1]),
            uint.Parse(parts[2].Split('-')[0])
        );
    }

    public int CompareTo(SemVersion other)
    {
        var cmp = Major.CompareTo(other.Major);
        if (cmp != 0) return cmp;
        cmp = Minor.CompareTo(other.Minor);
        if (cmp != 0) return cmp;
        return Patch.CompareTo(other.Patch);
    }

    public bool Equals(SemVersion other) => Major == other.Major && Minor == other.Minor && Patch == other.Patch;
    public override bool Equals(object? obj) => obj is SemVersion other && Equals(other);
    public override int GetHashCode() => HashCode.Combine(Major, Minor, Patch);
    public override string ToString() => Prerelease != null ? $"{Major}.{Minor}.{Patch}-{Prerelease}" : $"{Major}.{Minor}.{Patch}";
    public static bool operator ==(SemVersion left, SemVersion right) => left.Equals(right);
    public static bool operator !=(SemVersion left, SemVersion right) => !left.Equals(right);
    public static bool operator <(SemVersion left, SemVersion right) => left.CompareTo(right) < 0;
    public static bool operator >(SemVersion left, SemVersion right) => left.CompareTo(right) > 0;
}

/// <summary>
/// Runtime environment for skill execution
/// </summary>
public enum SkillRuntimeType
{
    Wasm,
    Python,
    JavaScript,
    Rust,
    CSharp,
    Go,
    Shell,
    Binary,
    Custom
}

/// <summary>
/// Permission for skill execution
/// </summary>
public sealed class SkillPermission
{
    public string Name { get; }
    public string Description { get; }

    public SkillPermission(string name, string description)
    {
        Name = name;
        Description = description;
    }
}

/// <summary>
/// Skill execution priority
/// </summary>
public enum SkillPriorityType
{
    Low = 1,
    Normal = 5,
    High = 10,
    Critical = 20
}

/// <summary>
/// Skill lifecycle status
/// </summary>
public enum SkillStatus
{
    Inactive,
    Active,
    Deprecated,
    Unregistered
}

/// <summary>
/// Skill manifest with all metadata
/// </summary>
public sealed class SkillManifest
{
    public string Name { get; set; } = "";
    public string Version { get; set; } = "0.1.0";
    public string Description { get; set; } = "";
    public string Author { get; set; } = "";
    public SkillRuntimeType Runtime { get; set; } = SkillRuntimeType.Wasm;
    public string EntryPoint { get; set; } = "";
    public SkillPriorityType Priority { get; set; } = SkillPriorityType.Normal;
    public SkillStatus Status { get; set; } = SkillStatus.Inactive;
    public List<SkillDependency> Dependencies { get; set; } = new();
    public List<SkillPermission> Permissions { get; set; } = new();
    public Dictionary<string, string> Metadata { get; set; } = new();
    public Dictionary<string, byte[]> Assets { get; set; } = new();
}

/// <summary>
/// Dependency on another skill
/// </summary>
public sealed class SkillDependency
{
    public string Name { get; }
    public string VersionConstraint { get; }
    public bool IsOptional { get; }

    public SkillDependency(string name, string versionConstraint, bool optional = false)
    {
        Name = name;
        VersionConstraint = versionConstraint;
        IsOptional = optional;
    }

    public static SkillDependency Create(string name, string versionConstraint) => new(name, versionConstraint);
    public static SkillDependency CreateOptional(string name, string versionConstraint) => new(name, versionConstraint, optional: true);
    public override string ToString() => $"{Name}@{VersionConstraint}";
}

/// <summary>
/// Skill entity
/// </summary>
public sealed class Skill
{
    public SkillId Id { get; }
    public SkillManifest Manifest { get; }
    public string BinaryPath { get; }

    public Skill(SkillId id, SkillManifest manifest, string binaryPath)
    {
        Id = id;
        Manifest = manifest;
        BinaryPath = binaryPath;
    }

    public static Skill Create(SkillId id, SkillManifest manifest, string binaryPath) => new(id, manifest, binaryPath);
    public override string ToString() => $"{Manifest.Name}@{Manifest.Version} [{Id}]";
}

/// <summary>
/// Result of a dependency resolution
/// </summary>
public sealed class DependencyResolutionResult
{
    public bool IsSuccess { get; }
    public IReadOnlyList<Skill> ResolvedOrder { get; }
    public IReadOnlyList<string> Errors { get; }

    private DependencyResolutionResult(bool success, IReadOnlyList<Skill> order, IReadOnlyList<string> errors)
    {
        IsSuccess = success;
        ResolvedOrder = order;
        Errors = errors;
    }

    public static DependencyResolutionResult Success(IReadOnlyList<Skill> order) => new(true, order, new List<string>());
    public static DependencyResolutionResult Failure(IReadOnlyList<string> errors) => new(false, new List<Skill>(), errors);
}

/// <summary>
/// Event types for skill lifecycle
/// </summary>
public enum SkillEventType
{
    Registered,
    Loaded,
    Updated,
    Unregistered,
    Error
}

/// <summary>
/// Skill lifecycle event
/// </summary>
public sealed class SkillEvent
{
    public SkillEventType EventType { get; }
    public SkillId SkillId { get; }
    public DateTime Timestamp { get; }
    public string? ErrorMessage { get; }

    private SkillEvent(SkillEventType eventType, SkillId skillId, DateTime timestamp, string? errorMessage = null)
    {
        EventType = eventType;
        SkillId = skillId;
        Timestamp = timestamp;
        ErrorMessage = errorMessage;
    }

    public static SkillEvent Registered(SkillId skillId) => new(SkillEventType.Registered, skillId, DateTime.UtcNow);
    public static SkillEvent Loaded(SkillId skillId) => new(SkillEventType.Loaded, skillId, DateTime.UtcNow);
    public static SkillEvent Updated(SkillId skillId) => new(SkillEventType.Updated, skillId, DateTime.UtcNow);
    public static SkillEvent Unregistered(SkillId skillId) => new(SkillEventType.Unregistered, skillId, DateTime.UtcNow);
    public static SkillEvent Error(SkillId skillId, string message) => new(SkillEventType.Error, skillId, DateTime.UtcNow, message);
}

/// <summary>
/// Error types for skill operations
/// </summary>
public enum SkillErrorKind
{
    InvalidManifest,
    DependencyNotFound,
    VersionConflict,
    LoadingFailed,
    ExecutionFailed,
    PermissionDenied,
    SandboxError,
    RegistryLocked,
    AlreadyRegistered,
    NotFound
}

/// <summary>
/// Skill registry error
/// </summary>
public sealed class SkillRegistryError
{
    public SkillErrorKind Kind { get; }
    public string Message { get; }

    public SkillRegistryError(SkillErrorKind kind, string message)
    {
        Kind = kind;
        Message = message;
    }
}
