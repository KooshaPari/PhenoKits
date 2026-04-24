using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;

namespace Phenotype.Skills;

/// <summary>
/// Main registry for managing skills
/// </summary>
public sealed class SkillRegistry : IDisposable
{
    private readonly Dictionary<SkillId, Skill> _skills = new();
    private readonly ReaderWriterLockSlim _lock = new();
    private bool _disposed;

    public SkillRegistry()
    {
    }

    /// <summary>
    /// Register a new skill
    /// </summary>
    public void Register(Skill skill)
    {
        if (skill == null)
            throw new ArgumentNullException(nameof(skill));

        _lock.EnterWriteLock();
        try
        {
            if (_skills.ContainsKey(skill.Id))
            {
                throw new InvalidOperationException($"Skill {skill.Id} already registered");
            }

            skill.Manifest.Status = SkillStatus.Active;
            _skills[skill.Id] = skill;
        }
        finally
        {
            _lock.ExitWriteLock();
        }
    }

    /// <summary>
    /// Unregister a skill
    /// </summary>
    public bool Unregister(SkillId id)
    {
        _lock.EnterWriteLock();
        try
        {
            if (_skills.TryGetValue(id, out var skill))
            {
                skill.Manifest.Status = SkillStatus.Unregistered;
                return _skills.Remove(id);
            }
            return false;
        }
        finally
        {
            _lock.ExitWriteLock();
        }
    }

    /// <summary>
    /// Get a skill by ID
    /// </summary>
    public Skill? Get(SkillId id)
    {
        _lock.EnterReadLock();
        try
        {
            _skills.TryGetValue(id, out var skill);
            return skill;
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Get all registered skills
    /// </summary>
    public IReadOnlyList<Skill> GetAll()
    {
        _lock.EnterReadLock();
        try
        {
            return _skills.Values.ToList();
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Check if a skill is registered
    /// </summary>
    public bool IsRegistered(SkillId id)
    {
        _lock.EnterReadLock();
        try
        {
            return _skills.ContainsKey(id);
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Get skills by name (can have multiple versions)
    /// </summary>
    public IReadOnlyList<Skill> GetByName(string name)
    {
        _lock.EnterReadLock();
        try
        {
            return _skills.Values
                .Where(s => s.Manifest.Name == name)
                .ToList();
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Get the latest version of a skill by name
    /// </summary>
    public Skill? GetLatest(string name)
    {
        _lock.EnterReadLock();
        try
        {
            return _skills.Values
                .Where(s => s.Manifest.Name == name)
                .OrderByDescending(s => s.Manifest.Version)
                .FirstOrDefault();
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Count of registered skills
    /// </summary>
    public int Count
    {
        get
        {
            _lock.EnterReadLock();
            try
            {
                return _skills.Count;
            }
            finally
            {
                _lock.ExitReadLock();
            }
        }
    }

    public void Dispose()
    {
        if (_disposed) return;
        _lock.Dispose();
        _disposed = true;
    }
}
