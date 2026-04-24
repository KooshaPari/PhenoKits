# ADR-003: Git LFS Integration Strategy

## Status

Accepted

## Context

The artifacts repository must work with Git while handling large binary files. We need a strategy that:

1. Keeps the Git repository size reasonable
2. Allows version control of artifact metadata
3. Enables CI/CD workflows to push/pull artifacts
4. Works with GitHub/GitLab out of the box

## Decision

We will use **Git LFS as the primary storage mechanism**, with the following design:

### Repository Structure

```
artifacts/
├── .gitattributes          # LFS patterns
├── manifest.json           # Artifact index (Git-tracked)
├── binaries/               # LFS-tracked binaries
├── releases/               # LFS-tracked release bundles
├── ci/                     # Git-tracked CI metadata (small files)
└── reports/                # LFS-tracked large reports
```

### .gitattributes Configuration

```gitattributes
# Large binaries
binaries/**/* filter=lfs diff=lfs merge=lfs -text
releases/**/* filter=lfs diff=lfs merge=lfs -text
*.tar.gz filter=lfs diff=lfs merge=lfs -text
*.zip filter=lfs diff=lfs merge=lfs -text

# Reports and docs
reports/**/*.pdf filter=lfs diff=lfs merge=lfs -text

# Small files stay in Git
manifest.json text eol=lf
ci/**/*.json text eol=lf
ci/**/*.xml text eol=lf
```

### LFS Pointer Files

Git LFS replaces large files with text pointers:

```
version https://git-lfs.github.com/spec/v1
sha256:abc123...def456
size 15234567
```

The actual content is stored in `.git/lfs/objects/` or on the LFS server.

### Manifest + LFS Integration

```json
{
  "artifacts": [
    {
      "id": "pheno-cli-v1.2.0",
      "path": "binaries/pheno-cli/v1.2.0/linux-amd64",
      "lfs": {
        "oid": "sha256:abc123...",
        "size": 15234567
      },
      "verified": true
    }
  ]
}
```

### CLI Integration

```bash
# Upload artifact
pheno artifacts upload ./pheno-cli --name pheno-cli --version 1.2.0

# Download artifact
pheno artifacts download pheno-cli --version 1.2.0 --platform linux-amd64

# Verify manifest matches LFS
pheno artifacts verify
```

## Consequences

### Positive

- Native GitHub/GitLab support (no custom server needed)
- Bandwidth-efficient (only pulls needed artifacts)
- Version-controlled metadata
- Familiar Git workflow for developers
- Free storage on public repos

### Negative

- Bandwidth limits on free tiers (GitHub: 1GB/month)
- Requires Git LFS client installation
- LFS server is separate from Git server
- No fine-grained access control per artifact

## Alternatives Considered

| Alternative | Reasoning |
|-------------|-----------|
| Custom registry server | Would require building and maintaining infrastructure |
| Git submodules | Would fragment the repository; complex for CI |
| Direct S3 | Would lose Git integration and version control |
| Git Annex | Less widely supported than LFS |
| Self-hosted LFS | Would add operational burden |

## Rate Limits and Quotas

| Provider | Storage | Bandwidth | Notes |
|----------|---------|-----------|-------|
| GitHub Free | 2GB | 1GB/month | Soft limits |
| GitHub Pro | 2GB | Included | Per user |
| GitLab.com | 10GB | 10GB/month | Group limit |
| Self-hosted | Unlimited | Unlimited | Infrastructure cost |

## Migration Path

1. Phase 1: Git LFS only (current)
2. Phase 2: Optional S3 backend for large artifacts
3. Phase 3: Hybrid (LFS for small, S3 for large)

## References

- Git LFS: https://git-lfs.com/
- GitHub LFS docs: https://docs.github.com/en/repositories/working-with-files/managing-large-files
- GitLab LFS docs: https://docs.gitlab.com/ee/topics/git/lfs/
