namespace Phenotype.Skills;

using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using System.Text.Json.Serialization;

/// <summary>
/// Skill manifest loader
/// </summary>
public class SkillLoader
{
    private readonly JsonSerializerOptions _jsonOptions;

    public SkillLoader()
    {
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
            WriteIndented = true
        };
    }

    /// <summary>
    /// Load a skill manifest from a TOML file
    /// </summary>
    public SkillManifest LoadFromToml(string path)
    {
        var toml = File.ReadAllText(path);
        return ParseToml(toml);
    }

    /// <summary>
    /// Parse a skill manifest from a TOML string
    /// </summary>
    public SkillManifest ParseToml(string toml)
    {
        // Simplified TOML parsing - in production use a proper TOML library
        var manifest = new SkillManifest();

        var lines = toml.Split('\n');
        var section = "";

        foreach (var line in lines)
        {
            var trimmed = line.Trim();
            if (string.IsNullOrEmpty(trimmed) || trimmed.StartsWith("#"))
                continue;

            if (trimmed.StartsWith("[") && trimmed.EndsWith("]"))
            {
                section = trimmed[1..^1];
                continue;
            }

            var parts = trimmed.Split('=', 2);
            if (parts.Length != 2) continue;

            var key = parts[0].Trim();
            var value = parts[1].Trim().Trim('"', '\'');

            switch (section)
            {
                case "skill":
                    switch (key)
                    {
                        case "name": manifest.Name = value; break;
                        case "version": manifest.Version = value; break;
                        case "description": manifest.Description = value; break;
                        case "author": manifest.Author = value; break;
                    }
                    break;
            }
        }

        return manifest;
    }

    /// <summary>
    /// Save a manifest to a TOML file
    /// </summary>
    public void SaveToToml(SkillManifest manifest, string path)
    {
        var toml = $@"[skill]
name = ""{manifest.Name}""
version = ""{manifest.Version}""
description = ""{manifest.Description}""
author = ""{manifest.Author}""
";

        File.WriteAllText(path, toml);
    }

    /// <summary>
    /// Serialize a manifest to JSON
    /// </summary>
    public string ToJson(SkillManifest manifest)
    {
        return JsonSerializer.Serialize(manifest, _jsonOptions);
    }

    /// <summary>
    /// Deserialize a manifest from JSON
    /// </summary>
    public SkillManifest FromJson(string json)
    {
        return JsonSerializer.Deserialize<SkillManifest>(json, _jsonOptions)
            ?? throw new InvalidOperationException("Failed to deserialize manifest");
    }
}
