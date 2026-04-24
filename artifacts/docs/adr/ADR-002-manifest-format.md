# ADR-002: Manifest Format and Versioning

## Status

Accepted

## Context

We need a format to describe artifact metadata that:

1. Identifies artifact content via cryptographic hash
2. Supports multiple artifact types (binary, archive, report)
3. Enables discovery and search
4. Is human-readable and machine-parseable

## Decision

We will use **JSON-based manifests** inspired by OCI Image Spec, with the following structure:

### Artifact Manifest Schema

```json
{
  "schemaVersion": 1,
  "mediaType": "application/vnd.phenotype.artifact.v1+json",
  "artifact": {
    "id": "pheno-cli-v1.2.0-linux-amd64",
    "name": "pheno-cli",
    "version": "1.2.0",
    "platform": "linux-amd64",
    "kind": "binary",
    "created": "2026-04-03T10:30:00Z"
  },
  "content": {
    "digest": "sha256:abc123...",
    "size": 15234567,
    "mediaType": "application/x-executable"
  },
  "provenance": {
    "source": "git",
    "repository": "phenotype/crates",
    "commit": "def456...",
    "ciRunId": "123456789",
    "builtBy": "github-actions"
  },
  "metadata": {
    "labels": {
      "project": "phenotype",
      "component": "cli"
    },
    "annotations": {
      "description": "Cross-platform CLI tool"
    }
  }
}
```

### Index Manifest

An index tracks all artifacts in a namespace:

```json
{
  "schemaVersion": 1,
  "mediaType": "application/vnd.phenotype.index.v1+json",
  "generated": "2026-04-04T00:00:00Z",
  "artifacts": [
    {"digest": "sha256:abc...", "size": 1234, "platform": "linux-amd64"},
    {"digest": "sha256:def...", "size": 1234, "platform": "darwin-arm64"}
  ]
}
```

## Consequences

### Positive

- Human-readable for debugging
- Cryptographic verification of content
- Provenance tracking for supply chain
- Labels enable flexible categorization

### Negative

- JSON overhead for small manifests
- No built-in compression
- Manual versioning required

## Alternatives Considered

| Alternative | Reasoning |
|-------------|-----------|
| SQLite | Would require database for basic read operations |
| Protocol Buffers | Would add complexity; human-readability is priority |
| YAML | No significant advantage over JSON for this use case |
| Direct OCI | Would be overkill; we don't need container-specific features |

## Versioning Strategy

- `schemaVersion` starts at 1
- Minor changes: add optional fields
- Major changes: increment schemaVersion, maintain backward compatibility for read

## References

- OCI Image Spec: https://github.com/opencontainers/image-spec/blob/main/spec.md
- SLSA Provenance: https://slsa.dev/provenance/v1
