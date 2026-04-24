# SPEC: Artifacts

## Overview

The Artifacts system provides versioned storage for build outputs, CI artifacts, and release assets for the Phenotype ecosystem. It implements a content-addressable storage layer with Git LFS integration, cryptographic integrity verification, and a pluggable backend architecture.

**Goals:**
- Store build artifacts with integrity guarantees
- Enable reproducible builds through content verification
- Support local development and CI/CD workflows
- Provide a migration path to cloud-scale storage

**Non-Goals:**
- Real-time artifact streaming
- Complex access control (rely on Git permissions)
- Built-in artifact execution
- General-purpose object storage

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           ARTIFACT SYSTEM                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────┐   │
│  │                        INGESTION LAYER                              │   │
│  │                                                                      │   │
│  │   ┌────────────┐    ┌────────────┐    ┌────────────┐               │   │
│  │   │   CI/CD    │    │   CLI      │    │   Manual   │               │   │
│  │   │  GitHub    │───▶│  Upload    │    │   Upload   │               │   │
│  │   │  Actions   │    │  (API)     │    │   (Git)    │               │   │
│  │   └────────────┘    └────────────┘    └────────────┘               │   │
│  │                            │                                        │   │
│  └────────────────────────────┼────────────────────────────────────────┘   │
│                               │                                            │
│                               ▼                                            │
│  ┌────────────────────────────────────────────────────────────────────┐   │
│  │                      VALIDATION LAYER                               │   │
│  │                                                                      │   │
│  │   ┌────────────┐    ┌────────────┐    ┌────────────┐               │   │
│  │   │   SHA-256  │    │   Size     │    │  Content   │               │   │
│  │   │   Hash     │    │   Check    │    │   Type     │               │   │
│  │   │ Verification│    │            │    │ Validation │               │   │
│  │   └────────────┘    └────────────┘    └────────────┘               │   │
│  │                                                                      │   │
│  └────────────────────────────┼────────────────────────────────────────┘   │
│                               │                                            │
│                               ▼                                            │
│  ┌────────────────────────────────────────────────────────────────────┐   │
│  │                      STORAGE LAYER                                  │   │
│  │                                                                      │   │
│  │   ┌────────────┐    ┌────────────┐    ┌────────────┐               │   │
│  │   │  binaries/ │    │  releases/ │    │   ci/      │               │   │
│  │   │            │    │            │    │            │               │   │
│  │   │  • Linux   │    │  • v1.0.0  │    │  • Test    │               │   │
│  │   │  • macOS   │    │  • v1.1.0  │    │    results │               │   │
│  │   │  • Windows │    │  • v2.0.0  │    │  • Coverage│               │   │
│  │   │            │    │            │    │  • Logs    │               │   │
│  │   └────────────┘    └────────────┘    └────────────┘               │   │
│  │                                                                      │   │
│  │   ┌────────────┐    ┌────────────┐    ┌────────────┐               │   │
│  │   │  reports/  │    │   docs/    │    │  staging/  │               │   │
│  │   │            │    │            │    │  (temp)    │               │   │
│  │   │  • Audit   │    │  • HTML    │    │            │               │   │
│  │   │  • Metrics │    │  • PDF     │    │            │               │   │
│  │   │  • SARIF   │    │  • API     │    │            │               │   │
│  │   └────────────┘    └────────────┘    └────────────┘               │   │
│  │                                                                      │   │
│  └────────────────────────────┼────────────────────────────────────────┘   │
│                               │                                            │
│                               ▼                                            │
│  ┌────────────────────────────────────────────────────────────────────┐   │
│  │                      RETRIEVAL LAYER                                │   │
│  │                                                                      │   │
│  │   ┌────────────┐    ┌────────────┐    ┌────────────┐               │   │
│  │   │   HTTP     │    │    Git     │    │   CLI      │               │   │
│  │   │   GET      │    │   Clone    │    │   Fetch    │               │   │
│  │   └────────────┘    └────────────┘    └────────────┘               │   │
│  │                                                                      │   │
│  └────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Components Table

| Component | Type | Purpose | Storage |
|-----------|------|---------|---------|
| `binaries/` | Directory | Cross-platform executables | Git LFS |
| `releases/` | Directory | Versioned release bundles | Git LFS |
| `ci/` | Directory | CI outputs (test results, logs) | Git |
| `reports/` | Directory | Generated reports (audit, metrics) | Git LFS |
| `docs/` | Directory | Built documentation | Git |
| `staging/` | Directory | Temporary upload area | Git (gitignored) |
| `manifest.json` | File | Artifact index with checksums | Git |
| `.github/workflows/` | CI | Automated artifact generation | N/A |
| `storage/` | Directory | Content-addressable blob store | Git LFS/FS |

---

## Data Models

### Artifact Manifest

The primary data structure describing an artifact:

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
    "digest": "sha256:a1b2c3d4e5f6...",
    "size": 15234567,
    "mediaType": "application/x-executable"
  },
  "provenance": {
    "source": "git",
    "repository": "phenotype/crates",
    "commit": "abc123def456",
    "ciRunId": "123456789",
    "builtBy": "github-actions",
    "builderUrl": "https://github.com/phenotype/crates/actions/runs/123456789"
  },
  "metadata": {
    "labels": {
      "project": "phenotype",
      "component": "cli",
      "stage": "release"
    },
    "annotations": {
      "description": "Cross-platform CLI tool for Phenotype ecosystem",
      "license": "MIT",
      "homepage": "https://github.com/phenotype/crates"
    }
  },
  "retention": {
    "expiresAt": null,
    "keepForever": true
  }
}
```

### Schema Definition

```typescript
interface ArtifactManifest {
  schemaVersion: number;
  mediaType: string;
  artifact: {
    id: string;              // Unique identifier
    name: string;            // Human-readable name
    version: string;         // Semantic version
    platform: string;        // target triple: os-arch
    kind: ArtifactKind;      // binary|archive|report|document
    created: string;         // ISO 8601 timestamp
  };
  content: {
    digest: string;          // sha256:hash
    size: number;            // bytes
    mediaType: string;       // MIME type
  };
  provenance: {
    source: string;          // git|manual|api
    repository?: string;     // Source repository
    commit?: string;         // Git commit hash
    ciRunId?: string;        // CI run identifier
    builtBy?: string;        // Builder identity
    builderUrl?: string;     // Link to build logs
  };
  metadata: {
    labels: Record<string, string>;      // Searchable key-value pairs
    annotations: Record<string, string>; // Non-searchable metadata
  };
  retention: {
    expiresAt: string | null;  // ISO 8601 or null for permanent
    keepForever: boolean;      // Override expiration
  };
}

type ArtifactKind = 
  | "binary"      // Executable binary
  | "archive"     // tar.gz, zip, etc.
  | "report"      // Test results, coverage, etc.
  | "document"    // Documentation builds
  | "package";     // Language-specific packages
```

### CI Artifact Record

For CI-specific artifacts with build context:

```yaml
artifact:
  id: "ci-20260403-abc123"
  schemaVersion: 1
  source:
    type: "ci"
    repository: "phenotype/crates"
    workflow: "ci.yml"
    runId: 123456789
    runUrl: "https://github.com/phenotype/crates/actions/runs/123456789"
    commit: "abc123def456"
    branch: "main"
    tag: null
    prNumber: null
    triggeredBy: "push"
    
  environment:
    os: "ubuntu-22.04"
    arch: "x86_64"
    runner: "github-hosted"
    toolchain:
      rust: "1.78.0"
      cargo: "1.78.0"
    
  files:
    - path: "test-results.xml"
      digest: "sha256:def789..."
      size: 1024000
      mediaType: "application/xml+xunit"
      
    - path: "coverage.lcov"
      digest: "sha256:ghi012..."
      size: 2048000
      mediaType: "text/plain+lcov"
      
    - path: "benchmarks.json"
      digest: "sha256:jkl345..."
      size: 51200
      mediaType: "application/json"
      
  testSummary:
    total: 1523
    passed: 1520
    failed: 3
    skipped: 0
    duration: 45.2
    
  coverageSummary:
    lines: 89.5
    functions: 87.2
    branches: 84.1
    
  retention:
    expiresAt: "2026-05-03T00:00:00Z"
    keepForever: false
```

### Index Manifest

The master index tracking all artifacts:

```json
{
  "schemaVersion": 1,
  "mediaType": "application/vnd.phenotype.index.v1+json",
  "generatedAt": "2026-04-04T00:00:00Z",
  "generatedBy": "pheno-artifacts-v1.2.0",
  "repository": "phenotype/artifacts",
  "commit": "xyz789...",
  
  "stats": {
    "totalArtifacts": 247,
    "totalSize": 12584937472,
    "byKind": {
      "binary": 45,
      "archive": 23,
      "report": 156,
      "document": 23
    },
    "byProject": {
      "phenotype": 180,
      "helios": 42,
      "agileplus": 25
    }
  },
  
  "artifacts": [
    {
      "digest": "sha256:abc123...",
      "manifestPath": "manifests/abc123...json",
      "name": "pheno-cli",
      "version": "1.2.0",
      "platform": "linux-amd64",
      "kind": "binary",
      "created": "2026-04-03T10:30:00Z",
      "size": 15234567
    }
  ],
  
  "indices": {
    "byName": "indices/by-name.json",
    "byProject": "indices/by-project.json",
    "byDate": "indices/by-date.json"
  }
}
```

### Directory Structure

```
artifacts/
├── .git/                          # Git repository
│   └── lfs/                       # LFS object storage
│       └── objects/
│           └── ...
│
├── .gitattributes                 # LFS configuration
├── .gitignore                     # Ignore patterns
│
├── manifest.json                  # Master index (Git-tracked)
├── manifest.lock                  # Lock file for atomic updates
│
├── manifests/                     # Individual artifact manifests
│   ├── sha256-abc123.json
│   ├── sha256-def456.json
│   └── ...
│
├── indices/                       # Secondary indices
│   ├── by-name.json              # Index by artifact name
│   ├── by-project.json           # Index by project
│   ├── by-date.json              # Index by creation date
│   └── by-kind.json              # Index by artifact kind
│
├── binaries/                      # Executable binaries
│   ├── pheno-cli/
│   │   ├── latest -> v1.2.0/     # Symlink to latest
│   │   ├── v1.1.0/
│   │   │   ├── pheno-cli-linux-amd64       # LFS
│   │   │   ├── pheno-cli-linux-arm64       # LFS
│   │   │   ├── pheno-cli-darwin-amd64      # LFS
│   │   │   ├── pheno-cli-darwin-arm64      # LFS
│   │   │   └── pheno-cli-windows-amd64.exe # LFS
│   │   └── v1.2.0/
│   │       └── ...
│   └── kitty-specs/
│       └── ...
│
├── releases/                      # Release bundles
│   ├── v1.0.0/
│   │   ├── CHANGELOG.md           # Git-tracked
│   │   ├── checksums.sha256       # Git-tracked
│   │   └── bundle.tar.gz          # LFS
│   ├── v1.1.0/
│   │   └── ...
│   └── latest -> v1.1.0/          # Symlink
│
├── ci/                            # CI artifacts
│   └── phenotype/
│       └── crates/
│           ├── 2026-04-03/        # Date-based organization
│           │   ├── abc123/
│           │   │   ├── artifact.yaml      # Git-tracked
│           │   │   ├── test-results.xml   # Git-tracked (small)
│           │   │   ├── coverage.lcov      # Git-tracked (small)
│   │   │   └── benchmarks.json    # Git-tracked (small)
│   │   └── def456/
│   │       └── ...
│           └── 2026-04-04/
│               └── ...
│
├── reports/                       # Generated reports
│   ├── audit/
│   │   ├── 2026-Q1-security-audit.pdf     # LFS
│   │   └── 2026-Q1-security-audit.json     # Git-tracked
│   └── metrics/
│       ├── weekly-performance-report.html  # LFS
│       └── weekly-performance-report.json # Git-tracked
│
├── docs/                          # Built documentation
│   └── api-reference/
│       └── v1.2.0/
│           ├── index.html         # LFS
│           └── assets/
│               └── ...
│
└── staging/                       # Temporary upload area
    └── .gitkeep                   # Gitignored, directory only
```

---

## Storage Drivers

### Driver Interface

```go
// StorageDriver defines the interface for artifact storage backends
type StorageDriver interface {
    // Name returns the driver identifier
    Name() string
    
    // Read operations
    Get(ctx context.Context, digest string) (io.ReadCloser, error)
    Stat(ctx context.Context, digest string) (Descriptor, error)
    Exists(ctx context.Context, digest string) (bool, error)
    
    // Write operations
    Put(ctx context.Context, digest string, r io.Reader) error
    PutStream(ctx context.Context, digest string) (io.WriteCloser, error)
    
    // Delete operations
    Delete(ctx context.Context, digest string) error
    
    // Listing
    Walk(ctx context.Context, fn WalkFunc) error
    List(ctx context.Context, prefix string) ([]string, error)
    
    // Maintenance
    Verify(ctx context.Context, digest string) error
    Stats() DriverStats
}

// Descriptor describes content
type Descriptor struct {
    Digest    string // sha256:hash
    Size      int64
    MediaType string
    CreatedAt time.Time
}

// DriverStats provides driver metrics
type DriverStats struct {
    TotalBlobs  int64
    TotalSize   int64
    DriverName  string
}
```

### Filesystem Driver

The default driver using local filesystem with content-addressable storage:

```go
type FilesystemDriver struct {
    root string // Storage root directory
}

// Blob storage layout:
// <root>/blobs/sha256/
//   └── ab/
//       └── cd1234...ef   # 2-char prefix for sharding
//   └── 12/
//       └── 345678...90

func (d *FilesystemDriver) blobPath(digest string) string {
    // digest format: sha256:abcdef1234...
    parts := strings.SplitN(digest, ":", 2)
    if len(parts) != 2 {
        return ""
    }
    algorithm := parts[0]
    hash := parts[1]
    
    // Use 2-char prefix for directory sharding
    prefix := hash[:2]
    return filepath.Join(d.root, "blobs", algorithm, prefix, hash)
}

func (d *FilesystemDriver) Put(ctx context.Context, digest string, r io.Reader) error {
    path := d.blobPath(digest)
    
    // Ensure directory exists
    if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
        return err
    }
    
    // Write to temp file first
    tmpPath := path + ".tmp"
    f, err := os.Create(tmpPath)
    if err != nil {
        return err
    }
    defer os.Remove(tmpPath) // Cleanup on failure
    
    // Copy with digest verification
    hasher := sha256.New()
    tee := io.MultiWriter(f, hasher)
    
    if _, err := io.Copy(tee, r); err != nil {
        f.Close()
        return err
    }
    
    f.Close()
    
    // Verify digest
    computedDigest := "sha256:" + hex.EncodeToString(hasher.Sum(nil))
    if computedDigest != digest {
        return fmt.Errorf("digest mismatch: expected %s, got %s", digest, computedDigest)
    }
    
    // Atomic rename
    return os.Rename(tmpPath, path)
}

func (d *FilesystemDriver) Get(ctx context.Context, digest string) (io.ReadCloser, error) {
    path := d.blobPath(digest)
    return os.Open(path)
}

func (d *FilesystemDriver) Stat(ctx context.Context, digest string) (Descriptor, error) {
    path := d.blobPath(digest)
    info, err := os.Stat(path)
    if err != nil {
        return Descriptor{}, err
    }
    
    return Descriptor{
        Digest:    digest,
        Size:      info.Size(),
        CreatedAt: info.ModTime(),
    }, nil
}
```

### S3 Driver

Cloud-compatible driver for AWS S3 and compatible services:

```go
type S3Driver struct {
    client *s3.Client
    bucket string
    prefix string
}

// Blob storage layout:
// s3://<bucket>/<prefix>/blobs/sha256/ab/cd1234...ef

func (d *S3Driver) Put(ctx context.Context, digest string, r io.Reader) error {
    key := d.blobKey(digest)
    
    // S3 supports streaming upload with multipart for large files
    uploader := manager.NewUploader(d.client)
    
    _, err := uploader.Upload(ctx, &s3.PutObjectInput{
        Bucket:            aws.String(d.bucket),
        Key:               aws.String(key),
        Body:              r,
        ChecksumAlgorithm: types.ChecksumAlgorithmSha256,
    })
    
    return err
}

func (d *S3Driver) Get(ctx context.Context, digest string) (io.ReadCloser, error) {
    key := d.blobKey(digest)
    
    result, err := d.client.GetObject(ctx, &s3.GetObjectInput{
        Bucket: aws.String(d.bucket),
        Key:    aws.String(key),
    })
    if err != nil {
        return nil, err
    }
    
    return result.Body, nil
}

func (d *S3Driver) blobKey(digest string) string {
    parts := strings.SplitN(digest, ":", 2)
    if len(parts) != 2 {
        return ""
    }
    algorithm := parts[0]
    hash := parts[1]
    prefix := hash[:2]
    
    return filepath.Join(d.prefix, "blobs", algorithm, prefix, hash)
}
```

### Git LFS Driver

Driver that integrates with Git LFS for version-controlled storage:

```go
type GitLFSDriver struct {
    repoRoot   string
    remoteURL  string
    batchSize  int
}

// LFS Pointer Format:
// version https://git-lfs.github.com/spec/v1
// oid sha256:abc123...
// size 15234567

type LFSPointer struct {
    Version string
    OID     string // sha256:hash
    Size    int64
}

func (d *GitLFSDriver) Put(ctx context.Context, digest string, r io.Reader) error {
    // 1. Write to LFS storage
    oid := strings.TrimPrefix(digest, "sha256:")
    
    // Ensure .git/lfs/objects directory exists
    objDir := filepath.Join(d.repoRoot, ".git", "lfs", "objects", oid[:2], oid[2:4])
    if err := os.MkdirAll(objDir, 0755); err != nil {
        return err
    }
    
    objPath := filepath.Join(objDir, oid)
    
    // Write content
    f, err := os.Create(objPath)
    if err != nil {
        return err
    }
    defer f.Close()
    
    hasher := sha256.New()
    tee := io.MultiWriter(f, hasher)
    size, err := io.Copy(tee, r)
    if err != nil {
        return err
    }
    
    f.Close()
    
    // Verify digest
    computedOID := hex.EncodeToString(hasher.Sum(nil))
    if computedOID != oid {
        os.Remove(objPath)
        return fmt.Errorf("digest mismatch: expected %s, got %s", oid, computedOID)
    }
    
    // 2. Create pointer file content
    pointer := fmt.Sprintf("version https://git-lfs.github.com/spec/v1\noid sha256:%s\nsize %d\n", oid, size)
    
    // 3. Write pointer file to working tree
    // Path determined by artifact metadata
    
    return nil
}

func (d *GitLFSDriver) Get(ctx context.Context, digest string) (io.ReadCloser, error) {
    oid := strings.TrimPrefix(digest, "sha256:")
    
    // Try local first
    localPath := filepath.Join(d.repoRoot, ".git", "lfs", "objects", oid[:2], oid[2:4], oid)
    if f, err := os.Open(localPath); err == nil {
        return f, nil
    }
    
    // Fetch from remote if not local
    if err := d.fetchFromRemote(ctx, oid); err != nil {
        return nil, err
    }
    
    return os.Open(localPath)
}

func (d *GitLFSDriver) fetchFromRemote(ctx context.Context, oid string) error {
    // Use git-lfs command or implement batch API
    cmd := exec.CommandContext(ctx, "git", "lfs", "fetch", "--include", oid)
    cmd.Dir = d.repoRoot
    return cmd.Run()
}
```

---

## API Specification

### REST API

#### Authentication

All API requests require authentication via:
- `Authorization: Bearer <token>` header
- Token obtained from GitHub/GitLab or CLI login

#### Endpoints

##### List Artifacts

```
GET /api/v1/artifacts
```

Query Parameters:
| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | string | Filter by artifact name |
| `version` | string | Filter by version |
| `kind` | string | Filter by artifact kind |
| `platform` | string | Filter by platform |
| `project` | string | Filter by project label |
| `limit` | integer | Max results (default: 20, max: 100) |
| `offset` | integer | Pagination offset |

Response:
```json
{
  "artifacts": [
    {
      "id": "pheno-cli-v1.2.0-linux-amd64",
      "name": "pheno-cli",
      "version": "1.2.0",
      "platform": "linux-amd64",
      "kind": "binary",
      "digest": "sha256:abc123...",
      "size": 15234567,
      "created": "2026-04-03T10:30:00Z",
      "labels": {
        "project": "phenotype"
      }
    }
  ],
  "pagination": {
    "total": 247,
    "limit": 20,
    "offset": 0,
    "hasMore": true
  }
}
```

##### Get Artifact

```
GET /api/v1/artifacts/{id}
```

Response:
```json
{
  "schemaVersion": 1,
  "mediaType": "application/vnd.phenotype.artifact.v1+json",
  "artifact": { ... },
  "content": { ... },
  "provenance": { ... },
  "metadata": { ... }
}
```

##### Download Artifact Content

```
GET /api/v1/artifacts/{id}/download
```

Response: Binary content with `Content-Type` and `Content-Length` headers.

Headers:
| Header | Value |
|--------|-------|
| `Content-Type` | Artifact mediaType |
| `Content-Length` | Artifact size |
| `Content-Digest` | sha256:digest |
| `X-Artifact-Name` | Artifact name |
| `X-Artifact-Version` | Artifact version |

##### Upload Artifact

```
POST /api/v1/artifacts
```

Request Body (multipart/form-data):
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | file | Yes | Artifact binary |
| `name` | string | Yes | Artifact name |
| `version` | string | Yes | Semantic version |
| `platform` | string | Yes | Target platform |
| `kind` | string | Yes | Artifact kind |
| `labels` | json | No | Key-value labels |
| `annotations` | json | No | Key-value annotations |

Response:
```json
{
  "id": "pheno-cli-v1.3.0-linux-amd64",
  "digest": "sha256:def456...",
  "size": 16384000,
  "uploadedAt": "2026-04-04T12:00:00Z"
}
```

##### Delete Artifact

```
DELETE /api/v1/artifacts/{id}
```

Response: `204 No Content`

##### Verify Artifact

```
POST /api/v1/artifacts/{id}/verify
```

Verifies that the stored content matches the recorded digest.

Response:
```json
{
  "valid": true,
  "digest": "sha256:abc123...",
  "computed": "sha256:abc123...",
  "checkedAt": "2026-04-04T12:00:00Z"
}
```

##### Get Index

```
GET /api/v1/index
```

Returns the complete artifact index.

Response: Index manifest JSON

##### Regenerate Index

```
POST /api/v1/index/regenerate
```

Regenerates the index from stored manifests.

Response:
```json
{
  "generatedAt": "2026-04-04T12:00:00Z",
  "artifactsIndexed": 247,
  "duration": 1.23
}
```

### CLI API

```bash
# List artifacts
pheno artifacts list [--name NAME] [--version VERSION] [--kind KIND]

# Get artifact info
pheno artifacts info <id-or-name> [--version VERSION] [--platform PLATFORM]

# Download artifact
pheno artifacts download <id-or-name> [--version VERSION] [--platform PLATFORM] [--output PATH]

# Upload artifact
pheno artifacts upload <file> --name NAME --version VERSION --platform PLATFORM [--label KEY=VALUE]

# Delete artifact
pheno artifacts delete <id>

# Verify artifact integrity
pheno artifacts verify [id]

# Search artifacts
pheno artifacts search <query>

# Sync with remote
pheno artifacts sync [--push] [--pull]

# Cleanup expired artifacts
pheno artifacts cleanup [--dry-run]

# Show storage stats
pheno artifacts stats
```

---

## CI/CD Integration

### GitHub Actions

```yaml
# .github/workflows/artifacts.yml
name: Upload Artifacts

on:
  push:
    tags: ['v*']
  workflow_run:
    workflows: ["Build"]
    types: [completed]

jobs:
  upload:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
        with:
          lfs: true
          
      - name: Build
        run: cargo build --release
        
      - name: Upload Artifact
        uses: phenotype/artifacts-action@v1
        with:
          name: pheno-cli
          version: ${{ github.ref_name }}
          files: |
            target/release/pheno-cli:linux-amd64
          labels: |
            project=phenotype
            component=cli
            
      - name: Upload CI Results
        uses: phenotype/artifacts-action@v1
        with:
          kind: report
          files: |
            test-results.xml
            coverage.lcov
          retention: 30
```

### GitLab CI

```yaml
# .gitlab-ci.yml
stages:
  - build
  - upload

upload:artifacts:
  stage: upload
  image: phenotype/artifacts-cli:latest
  script:
    - pheno artifacts upload target/release/pheno-cli
        --name pheno-cli
        --version $CI_COMMIT_TAG
        --platform linux-amd64
        --label project=phenotype
  only:
    - tags
```

### Local Development

```bash
# Configure artifacts CLI
pheno artifacts config set storage.driver filesystem
pheno artifacts config set storage.root ~/.local/share/phenotype/artifacts

# Upload local build
pheno artifacts upload ./target/release/pheno-cli \
  --name pheno-cli \
  --version $(git describe --tags) \
  --platform $(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)

# Download for testing
pheno artifacts download pheno-cli --version latest --platform linux-amd64
```

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Upload time (10MB) | < 5s | End-to-end upload |
| Download time (10MB) | < 3s | End-to-end download |
| Manifest lookup | < 10ms | JSON parse + search |
| Index regeneration | < 5s | For 1000 artifacts |
| Verify artifact | < 2s | SHA-256 verification |
| CLI startup | < 100ms | Cold start time |
| CI artifact retention | 30 days | Configurable |
| Release artifact retention | Forever | Permanent storage |
| Storage deduplication | 80%+ | Identical file detection |
| Git LFS bandwidth | < 1GB/month | Per project |

### Size Limits

| Artifact Type | Max Size | Notes |
|--------------|----------|-------|
| Binary | 100MB | Compressed if larger |
| CI output | 50MB | Auto-pruned after retention |
| Report | 10MB | PDF/HTML |
| Release bundle | 500MB | Includes all platforms |
| Documentation | 100MB | Static site builds |

---

## Security

### Content Integrity

All artifacts are verified using SHA-256 digests:

```
Digest = SHA-256(Content)
Stored = "sha256:" + Hex(Digest)
```

Verification happens:
- On upload (computed vs provided)
- On download (computed vs stored)
- Periodically via `pheno artifacts verify`

### Authentication

Authentication via Git provider tokens:
- GitHub: Personal Access Token or GITHUB_TOKEN
- GitLab: Personal Access Token or CI_JOB_TOKEN

### Authorization

Access control via Git repository permissions:
- Read: Repository read access
- Write: Repository write access
- Admin: Repository admin access

### Audit Trail

All operations are logged:
```json
{
  "timestamp": "2026-04-04T12:00:00Z",
  "actor": "github:username",
  "action": "upload",
  "artifact": "pheno-cli-v1.2.0",
  "digest": "sha256:abc123...",
  "source": "cli",
  "ip": "192.168.1.1"
}
```

### Content Types

Media types are validated against a whitelist:
```
application/x-executable
application/x-mach-binary
application/x-elf
application/zip
application/gzip
application/x-tar
application/json
application/xml
text/plain
text/html
text/markdown
```

---

## Implementation Phases

### Phase 1: Core (MVP)

- Filesystem driver
- JSON manifest format
- CLI upload/download
- Basic index
- Git LFS integration

### Phase 2: CI/CD

- GitHub Actions integration
- GitLab CI integration
- CI artifact records
- Retention policies

### Phase 3: Cloud

- S3 driver
- Azure Blob driver
- GCS driver
- Hybrid storage

### Phase 4: Advanced

- Content signing (cosign)
- SBOM generation
- Vulnerability scanning hooks
- Federation

---

## Error Handling

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `ARTIFACT_NOT_FOUND` | 404 | Artifact does not exist |
| `ARTIFACT_EXISTS` | 409 | Artifact already exists |
| `DIGEST_MISMATCH` | 400 | Content digest verification failed |
| `INVALID_MANIFEST` | 400 | Manifest schema validation failed |
| `STORAGE_ERROR` | 500 | Storage backend error |
| `RATE_LIMITED` | 429 | Too many requests |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |

### Error Response Format

```json
{
  "error": {
    "code": "ARTIFACT_NOT_FOUND",
    "message": "Artifact 'pheno-cli-v1.0.0' not found",
    "details": {
      "artifactId": "pheno-cli-v1.0.0",
      "suggestions": ["pheno-cli-v1.1.0", "pheno-cli-v1.2.0"]
    },
    "requestId": "req-abc123",
    "timestamp": "2026-04-04T12:00:00Z"
  }
}
```

---

## Testing Strategy

### Unit Tests

- Driver implementations
- Manifest validation
- Digest computation
- Path generation

### Integration Tests

- End-to-end upload/download
- Driver switching
- Git LFS workflows
- CLI commands

### Performance Tests

- Upload benchmarks (1MB, 10MB, 100MB)
- Download benchmarks
- Index regeneration at scale
- Concurrent operations

### Compatibility Tests

- Git LFS versions
- Git providers (GitHub, GitLab)
- Storage backends
- Platform support

---

## Monitoring

### Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `artifacts.total` | Gauge | Total artifacts stored |
| `artifacts.uploaded` | Counter | Artifacts uploaded |
| `artifacts.downloaded` | Counter | Artifacts downloaded |
| `artifacts.deleted` | Counter | Artifacts deleted |
| `artifacts.size` | Histogram | Artifact size distribution |
| `storage.used_bytes` | Gauge | Total storage used |
| `operation.duration` | Histogram | Operation latency |
| `operation.errors` | Counter | Operation errors |

### Health Checks

- Storage backend connectivity
- Git LFS availability
- Index consistency
- Disk space

### Alerts

- Storage > 80% capacity
- Error rate > 1%
- Latency > 5s (p99)

---

## References

- ADR-001: Storage Backend Selection
- ADR-002: Manifest Format and Versioning
- ADR-003: Git LFS Integration Strategy
- OCI Distribution Spec: https://github.com/opencontainers/distribution-spec
- OCI Image Spec: https://github.com/opencontainers/image-spec
- Git LFS: https://git-lfs.github.com/
- SLSA: https://slsa.dev/

---

## Protocol Specification

### Content Delivery Protocol

The artifacts system uses a content-addressable protocol for all blob transfers. This ensures integrity and enables deduplication.

#### Upload Protocol

```
1. Client computes SHA-256 digest of content
2. Client checks if content exists (HEAD request)
3. If not exists, client initiates upload (POST)
4. Server returns upload URL
5. Client streams content (PUT/PATCH)
6. Server verifies digest and commits
7. Server returns artifact manifest
```

#### Download Protocol

```
1. Client requests artifact by ID
2. Server returns manifest with digest
3. Client checks local cache by digest
4. If not cached, client requests blob
5. Server returns content with digest header
6. Client verifies digest
7. Client caches content by digest
```

#### Resumable Uploads

For large artifacts (>100MB), support resumable uploads:

```
1. Client initiates multipart upload
2. Server returns upload ID
3. Client uploads chunks with ETags
4. Server stores chunks
5. Client completes upload with manifest
6. Server assembles and verifies
```

### Caching Strategy

#### Client-Side Caching

```
Cache Location: ~/.cache/phenotype/artifacts/
Layout: sha256/ab/cd1234...ef

Cache Operations:
- GET: Check digest in cache first
- PUT: Store by digest after download
- CLEAN: LRU eviction at 90% capacity
```

#### Cache Headers

| Header | Value | Description |
|--------|-------|-------------|
| `Cache-Control` | `immutable` | Content never changes |
| `ETag` | `"sha256:abc123"` | Digest as ETag |
| `Last-Modified` | ISO 8601 | Creation timestamp |
| `Expires` | Far future | Content-addressable immutability |

### Integrity Verification

#### SHA-256 Verification

```go
func VerifyContent(r io.Reader, expectedDigest string) error {
    hasher := sha256.New()
    if _, err := io.Copy(hasher, r); err != nil {
        return err
    }
    
    computed := "sha256:" + hex.EncodeToString(hasher.Sum(nil))
    if computed != expectedDigest {
        return fmt.Errorf("digest mismatch: expected %s, got %s", 
            expectedDigest, computed)
    }
    return nil
}
```

#### Streaming Verification

```go
type VerifyingReader struct {
    r      io.Reader
    hasher hash.Hash
    expected string
}

func (v *VerifyingReader) Read(p []byte) (n int, err error) {
    n, err = v.r.Read(p)
    if n > 0 {
        v.hasher.Write(p[:n])
    }
    if err == io.EOF {
        computed := "sha256:" + hex.EncodeToString(v.hasher.Sum(nil))
        if computed != v.expected {
            return n, fmt.Errorf("digest mismatch: expected %s, got %s",
                v.expected, computed)
        }
    }
    return n, err
}
```

---

## Configuration System

### Configuration Hierarchy

Configuration is loaded from multiple sources (later overrides earlier):

1. Built-in defaults
2. System config (`/etc/phenotype/artifacts.yaml`)
3. User config (`~/.config/phenotype/artifacts.yaml`)
4. Project config (`./.phenotype/artifacts.yaml`)
5. Environment variables (`PHENO_ARTIFACTS_*`)
6. Command-line flags

### Configuration Schema

```yaml
# artifacts.yaml
version: 1

# Storage configuration
storage:
  driver: filesystem  # filesystem, s3, azure, gcs
  
  # Filesystem driver options
  filesystem:
    root: ~/.local/share/phenotype/artifacts
    
  # S3 driver options
  s3:
    bucket: phenotype-artifacts
    region: us-east-1
    prefix: artifacts
    endpoint: null  # For MinIO, set to http://localhost:9000
    credentials:
      accessKeyId: ${AWS_ACCESS_KEY_ID}
      secretAccessKey: ${AWS_SECRET_ACCESS_KEY}
      sessionToken: ${AWS_SESSION_TOKEN}
      
  # Azure Blob driver options
  azure:
    accountName: ${AZURE_STORAGE_ACCOUNT}
    accountKey: ${AZURE_STORAGE_KEY}
    container: artifacts
    prefix: artifacts
    
  # GCS driver options
  gcs:
    bucket: phenotype-artifacts
    prefix: artifacts
    credentials: ${GOOGLE_APPLICATION_CREDENTIALS}

# Git LFS configuration
git:
  lfs:
    enabled: true
    remote: origin
    fetchRecent: true
    fetchRecentRefsDays: 7
    fetchRecentCommitsDays: 0
    pruneOffsetDays: 30

# API configuration
api:
  endpoint: https://artifacts.phenotype.dev
  timeout: 30s
  retries: 3
  retryBackoff: exponential

# Authentication
auth:
  type: token  # token, oauth, iam
  token:
    source: git  # git, file, env
    gitProvider: github  # github, gitlab
  oauth:
    clientId: ${OAUTH_CLIENT_ID}
    clientSecret: ${OAUTH_CLIENT_SECRET}
    scopes:
      - read:artifacts
      - write:artifacts

# Cache configuration
cache:
  enabled: true
  directory: ~/.cache/phenotype/artifacts
  maxSize: 10GB
  ttl: 30d
  cleanupInterval: 24h

# Index configuration
index:
  autoGenerate: true
  generateOnUpload: true
  generateInterval: 1h
  indices:
    - by-name
    - by-project
    - by-date
    - by-kind

# Retention configuration
retention:
  defaultTTL: 30d
  ciArtifacts: 30d
  reportArtifacts: 90d
  binaryArtifacts: 0  # Never expire
  maxVersions: 10  # Keep last N versions per artifact

# Logging
logging:
  level: info  # debug, info, warn, error
  format: json  # json, text
  output: stderr  # stderr, stdout, file
  file: ~/.local/share/phenotype/artifacts.log
  
# Metrics
metrics:
  enabled: true
  backend: prometheus
  endpoint: /metrics
  port: 9090

# Security
security:
  verifyDownloads: true
  verifyUploads: true
  allowedMediaTypes:
    - application/x-executable
    - application/zip
    - application/gzip
    - application/json
    - text/plain
  blockedExtensions:
    - .exe
    - .dll
    - .so
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PHENO_ARTIFACTS_STORAGE_DRIVER` | Storage driver | `filesystem` |
| `PHENO_ARTIFACTS_STORAGE_ROOT` | Filesystem root | `~/.local/share/phenotype/artifacts` |
| `PHENO_ARTIFACTS_S3_BUCKET` | S3 bucket | - |
| `PHENO_ARTIFACTS_API_ENDPOINT` | API endpoint | `https://artifacts.phenotype.dev` |
| `PHENO_ARTIFACTS_TOKEN` | Auth token | - |
| `PHENO_ARTIFACTS_CACHE_DIR` | Cache directory | `~/.cache/phenotype/artifacts` |
| `PHENO_ARTIFACTS_LOG_LEVEL` | Log level | `info` |
| `PHENO_ARTIFACTS_VERIFY` | Verify transfers | `true` |

---

## Implementation Details

### Manifest Indexing System

The index system provides efficient artifact discovery without scanning all manifests.

#### Index Types

```go
// IndexType defines available indices
type IndexType string

const (
    IndexByName     IndexType = "by-name"
    IndexByProject    IndexType = "by-project"
    IndexByDate       IndexType = "by-date"
    IndexByKind       IndexType = "by-kind"
    IndexByPlatform   IndexType = "by-platform"
    IndexByDigest     IndexType = "by-digest"
)

// IndexEntry represents a single index entry
type IndexEntry struct {
    Digest    string    `json:"digest"`
    Name      string    `json:"name"`
    Version   string    `json:"version"`
    Platform  string    `json:"platform"`
    Kind      string    `json:"kind"`
    CreatedAt time.Time `json:"createdAt"`
    Size      int64     `json:"size"`
    Labels    map[string]string `json:"labels"`
}
```

#### By-Name Index

```json
{
  "indexType": "by-name",
  "generatedAt": "2026-04-04T00:00:00Z",
  "entries": {
    "pheno-cli": [
      {
        "digest": "sha256:abc123...",
        "name": "pheno-cli",
        "version": "1.2.0",
        "platform": "linux-amd64",
        "kind": "binary",
        "createdAt": "2026-04-03T10:30:00Z",
        "size": 15234567,
        "labels": {"project": "phenotype"}
      },
      {
        "digest": "sha256:def456...",
        "name": "pheno-cli",
        "version": "1.2.0",
        "platform": "darwin-arm64",
        "kind": "binary",
        "createdAt": "2026-04-03T10:30:00Z",
        "size": 14892345,
        "labels": {"project": "phenotype"}
      }
    ]
  }
}
```

#### By-Project Index

```json
{
  "indexType": "by-project",
  "generatedAt": "2026-04-04T00:00:00Z",
  "entries": {
    "phenotype": [
      {"digest": "sha256:abc123...", "name": "pheno-cli", "version": "1.2.0"},
      {"digest": "sha256:ghi012...", "name": "helios", "version": "2.1.0"}
    ],
    "agileplus": [
      {"digest": "sha256:jkl345...", "name": "agileplus-cli", "version": "0.5.0"}
    ]
  }
}
```

#### By-Date Index

```json
{
  "indexType": "by-date",
  "generatedAt": "2026-04-04T00:00:00Z",
  "entries": {
    "2026-04-03": ["sha256:abc123...", "sha256:def456..."],
    "2026-04-02": ["sha256:ghi012...", "sha256:jkl345..."]
  }
}
```

### Query System

#### Query Language

The artifacts system supports a simple query language for searching:

```
# Search by name
name:pheno-cli

# Search by project
project:phenotype

# Search by kind
kind:binary

# Search by platform
platform:linux-amd64

# Combined queries
name:pheno-cli AND version:1.2.0
project:phenotype AND kind:binary
kind:report AND created:>2026-04-01

# Wildcards
name:pheno-*
name:*-cli

# Range queries
size:1MB..10MB
created:2026-04-01..2026-04-30
```

#### Query Implementation

```go
func (idx *Index) Query(ctx context.Context, q Query) ([]IndexEntry, error) {
    // Use appropriate index based on query
    if q.Name != "" {
        return idx.queryByName(q.Name, q)
    }
    if q.Project != "" {
        return idx.queryByProject(q.Project, q)
    }
    // Fallback to full scan with filters
    return idx.queryScan(q)
}

func (idx *Index) queryByName(name string, q Query) ([]IndexEntry, error) {
    entries, ok := idx.byName[name]
    if !ok {
        return nil, nil
    }
    
    // Apply additional filters
    var results []IndexEntry
    for _, e := range entries {
        if matches(e, q) {
            results = append(results, e)
        }
    }
    return results, nil
}
```

### Retention Policy Engine

#### Policy Types

```go
type RetentionPolicy struct {
    Name        string
    Description string
    Rules       []RetentionRule
}

type RetentionRule struct {
    Type       RuleType
    Conditions []Condition
    Action     Action
}

type RuleType string

const (
    RuleTypeAge      RuleType = "age"
    RuleTypeVersions RuleType = "versions"
    RuleTypeSize     RuleType = "size"
    RuleTypeUnused   RuleType = "unused"
)
```

#### Default Policies

```yaml
policies:
  ci-artifacts:
    description: "Remove CI artifacts after 30 days"
    rules:
      - type: age
        conditions:
          - field: createdAt
            operator: gt
            value: 30d
        action: delete
        
  report-artifacts:
    description: "Remove old reports after 90 days"
    rules:
      - type: age
        conditions:
          - field: createdAt
            operator: gt
            value: 90d
        action: delete
        
  max-versions:
    description: "Keep only last 10 versions per artifact"
    rules:
      - type: versions
        conditions:
          - field: version
            operator: count
            value: 10
        action: delete
        exclude:
          - kind: release
```

#### Policy Evaluation

```go
func (pe *PolicyEngine) Evaluate(ctx context.Context, artifact Artifact) (Action, error) {
    for _, policy := range pe.policies {
        for _, rule := range policy.Rules {
            if matchesRule(artifact, rule) {
                return rule.Action, nil
            }
        }
    }
    return ActionKeep, nil
}

func matchesRule(artifact Artifact, rule RetentionRule) bool {
    for _, cond := range rule.Conditions {
        if !evaluateCondition(artifact, cond) {
            return false
        }
    }
    return true
}
```

---

## Platform Support

### Supported Platforms

| Platform | GOOS | GOARCH | Supported |
|----------|------|--------|-----------|
| Linux AMD64 | linux | amd64 | Yes |
| Linux ARM64 | linux | arm64 | Yes |
| macOS AMD64 | darwin | amd64 | Yes |
| macOS ARM64 | darwin | arm64 | Yes |
| Windows AMD64 | windows | amd64 | Yes |
| FreeBSD AMD64 | freebsd | amd64 | Planned |
| FreeBSD ARM64 | freebsd | arm64 | Planned |

### Platform Detection

```go
func DetectPlatform() string {
    goos := runtime.GOOS
    goarch := runtime.GOARCH
    
    // Normalize GOARCH
    switch goarch {
    case "amd64":
        goarch = "amd64"
    case "arm64":
        goarch = "arm64"
    case "386":
        goarch = "386"
    }
    
    return fmt.Sprintf("%s-%s", goos, goarch)
}
```

### Cross-Compilation

```bash
# Build for all platforms
GOOS=linux GOARCH=amd64 go build -o bin/pheno-cli-linux-amd64
GOOS=linux GOARCH=arm64 go build -o bin/pheno-cli-linux-arm64
GOOS=darwin GOARCH=amd64 go build -o bin/pheno-cli-darwin-amd64
GOOS=darwin GOARCH=arm64 go build -o bin/pheno-cli-darwin-arm64
GOOS=windows GOARCH=amd64 go build -o bin/pheno-cli-windows-amd64.exe

# Upload all
for platform in linux-amd64 linux-arm64 darwin-amd64 darwin-arm64 windows-amd64; do
    pheno artifacts upload bin/pheno-cli-$platform \
        --name pheno-cli \
        --version $(git describe --tags) \
        --platform $platform
done
```

---

## Deployment Scenarios

### Scenario 1: Single Developer

Setup for individual developer with local storage:

```yaml
# ~/.config/phenotype/artifacts.yaml
storage:
  driver: filesystem
  filesystem:
    root: ~/.local/share/phenotype/artifacts

git:
  lfs:
    enabled: true
```

### Scenario 2: Small Team

Setup for team with shared Git LFS:

```yaml
# .phenotype/artifacts.yaml (committed to repo)
storage:
  driver: git-lfs

git:
  lfs:
    enabled: true
    remote: origin

retention:
  ciArtifacts: 30d
  maxVersions: 10
```

### Scenario 3: CI/CD Heavy

Setup for CI/CD with S3 backend:

```yaml
# CI configuration
storage:
  driver: s3
  s3:
    bucket: phenotype-artifacts
    region: us-east-1
    prefix: "ci/${CI_RUN_ID}"

retention:
  defaultTTL: 7d
  ciArtifacts: 7d
```

### Scenario 4: Enterprise

Setup for enterprise with hybrid storage:

```yaml
storage:
  driver: hybrid
  hybrid:
    default: s3
    rules:
      - condition: size < 1MB
        driver: git-lfs
      - condition: kind = release
        driver: s3
        bucket: phenotype-releases
      - condition: kind = ci
        driver: s3
        bucket: phenotype-ci
        prefix: "ci/"

auth:
  type: oauth
  oauth:
    clientId: ${OAUTH_CLIENT_ID}
    scopes:
      - read:artifacts
      - write:artifacts
```

---

## Migration Guide

### Migrating from Raw Git LFS

If you already use Git LFS for artifacts:

```bash
# 1. Initialize manifest
pheno artifacts init --migrate

# 2. Scan existing LFS files
pheno artifacts scan --import-existing

# 3. Generate initial index
pheno artifacts index regenerate

# 4. Verify integrity
pheno artifacts verify --all
```

### Migrating to S3

```bash
# 1. Configure S3 driver
pheno artifacts config set storage.driver s3
pheno artifacts config set storage.s3.bucket my-bucket

# 2. Sync local to S3
pheno artifacts sync --push

# 3. Verify sync
pheno artifacts verify --all

# 4. Update remote config
# Edit .phenotype/artifacts.yaml and commit
```

### Migrating Between Backends

```bash
# Export from source
pheno artifacts export --driver filesystem --output ./export/

# Import to destination
pheno artifacts import --driver s3 --input ./export/
```

---

## Troubleshooting Guide

### Common Issues

#### Issue: LFS smudge filter error

```
Error: smudge filter lfs failed
```

Solution:
```bash
# Ensure LFS is installed
git lfs install

# Re-fetch LFS objects
git lfs fetch
git lfs checkout
```

#### Issue: Digest mismatch on upload

```
Error: digest mismatch: expected sha256:abc..., got sha256:def...
```

Causes:
- File was modified during upload
- Incorrect digest provided
- Encoding issue

Solution:
```bash
# Re-compute and verify
sha256sum <file>

# Upload with correct digest
pheno artifacts upload <file> --digest sha256:<correct>
```

#### Issue: Rate limited by GitHub

```
Error: API rate limit exceeded
```

Solution:
```bash
# Use authenticated requests
export GITHUB_TOKEN=<token>
pheno artifacts config set auth.token.source env

# Or wait and retry
sleep 60
```

#### Issue: Storage full

```
Error: no space left on device
```

Solution:
```bash
# Check storage usage
pheno artifacts stats

# Cleanup old artifacts
pheno artifacts cleanup --before 30d

# Prune LFS cache
git lfs prune
```

### Debug Mode

Enable debug logging:

```bash
export PHENO_ARTIFACTS_LOG_LEVEL=debug
pheno artifacts upload <file> --verbose
```

### Diagnostic Commands

```bash
# Check system health
pheno artifacts doctor

# Verify storage integrity
pheno artifacts verify --all

# Check configuration
pheno artifacts config show

# List broken references
pheno artifacts gc --dry-run
```

---

## Development Guide

### Building from Source

```bash
# Clone repository
git clone https://github.com/phenotype/artifacts.git
cd artifacts

# Build CLI
cargo build --release

# Run tests
cargo test

# Build for multiple platforms
make cross-compile
```

### Adding a New Storage Driver

1. Create driver file:
```go
// drivers/gcs/driver.go
package gcs

type Driver struct {
    client *storage.Client
    bucket string
    prefix string
}

func New(config Config) (*Driver, error) {
    // Initialize GCS client
}

func (d *Driver) Name() string {
    return "gcs"
}

// Implement StorageDriver interface...
```

2. Register driver:
```go
// drivers/registry.go
func init() {
    Register("gcs", gcs.New)
}
```

3. Add configuration:
```yaml
storage:
  driver: gcs
  gcs:
    bucket: my-bucket
    credentials: ${GOOGLE_APPLICATION_CREDENTIALS}
```

### Testing Storage Drivers

```go
func TestDriver(t *testing.T) {
    tests := []struct {
        name   string
        driver StorageDriver
    }{
        {"filesystem", NewFilesystemDriver(t.TempDir())},
        {"s3", NewS3Driver(testBucket)},
    }
    
    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            testDriver(t, tt.driver)
        })
    }
}

func testDriver(t *testing.T, d StorageDriver) {
    ctx := context.Background()
    content := []byte("test content")
    digest := computeDigest(content)
    
    // Test Put
    err := d.Put(ctx, digest, bytes.NewReader(content))
    require.NoError(t, err)
    
    // Test Get
    r, err := d.Get(ctx, digest)
    require.NoError(t, err)
    got, _ := io.ReadAll(r)
    assert.Equal(t, content, got)
    
    // Test Stat
    desc, err := d.Stat(ctx, digest)
    require.NoError(t, err)
    assert.Equal(t, int64(len(content)), desc.Size)
}
```

---

## Appendix

### A. Media Type Registry

| Media Type | Extension | Description |
|------------|-----------|-------------|
| `application/vnd.phenotype.artifact.v1+json` | .json | Artifact manifest |
| `application/vnd.phenotype.index.v1+json` | .json | Index manifest |
| `application/vnd.phenotype.ci.v1+json` | .json | CI artifact record |
| `application/x-executable` | - | ELF/Mach-O/PE binary |
| `application/x-mach-binary` | - | macOS binary |
| `application/x-elf` | - | Linux binary |
| `application/x-pe` | .exe | Windows binary |
| `application/zip` | .zip | ZIP archive |
| `application/gzip` | .gz | Gzip compressed |
| `application/x-tar` | .tar | Tar archive |
| `application/x-compressed-tar` | .tar.gz | Gzipped tar |
| `application/json` | .json | JSON data |
| `application/xml` | .xml | XML data |
| `text/plain` | .txt | Plain text |
| `text/html` | .html | HTML document |
| `text/markdown` | .md | Markdown document |
| `text/plain+lcov` | .lcov | LCOV coverage |
| `application/xml+xunit` | .xml | XUnit test results |
| `application/json+sarif` | .json | SARIF report |
| `application/pdf` | .pdf | PDF document |

### B. Digest Algorithms

| Algorithm | Prefix | Status |
|-----------|--------|--------|
| SHA-256 | `sha256:` | Required |
| SHA-512 | `sha512:` | Optional |
| Blake3 | `blake3:` | Experimental |

### C. Version Compatibility

| Schema Version | Supported Since | Breaking Changes |
|----------------|-----------------|------------------|
| 1 | Initial release | - |

### D. HTTP Status Codes

| Status | Meaning |
|--------|---------|
| 200 OK | Success |
| 201 Created | Artifact created |
| 204 No Content | Success, no body |
| 304 Not Modified | Content unchanged |
| 400 Bad Request | Invalid request |
| 401 Unauthorized | Authentication required |
| 403 Forbidden | Insufficient permissions |
| 404 Not Found | Artifact not found |
| 409 Conflict | Artifact already exists |
| 410 Gone | Artifact permanently removed |
| 413 Payload Too Large | Exceeds size limit |
| 422 Unprocessable | Validation failed |
| 429 Too Many Requests | Rate limited |
| 500 Internal Error | Server error |
| 503 Service Unavailable | Maintenance |

### E. CLI Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid usage |
| 3 | Artifact not found |
| 4 | Permission denied |
| 5 | Network error |
| 6 | Verification failed |
| 7 | Storage full |
| 8 | Rate limited |
| 9 | Timeout |
| 10 | Interrupted |

### F. Related Specifications

- [OCI Distribution Spec](https://github.com/opencontainers/distribution-spec)
- [OCI Image Spec](https://github.com/opencontainers/image-spec)
- [Git LFS Specification](https://github.com/git-lfs/git-lfs/blob/main/docs/README.md)
- [SLSA Provenance](https://slsa.dev/provenance/v1)
- [Sigstore](https://www.sigstore.dev/)
- [SPDX](https://spdx.dev/)
- [CycloneDX](https://cyclonedx.org/)

### G. Authentication Flows

#### GitHub Token Flow

```
1. User: gh auth login
2. GitHub: Redirect to browser for OAuth
3. Browser: User authorizes application
4. GitHub: Redirect back with code
5. CLI: Exchange code for token
6. CLI: Store token in keychain/credential store
7. CLI: Use token for API requests
```

#### GitLab Token Flow

```
1. User: Create Personal Access Token in GitLab UI
2. User: glab auth login --token <token>
3. CLI: Store token securely
4. CLI: Use token with PRIVATE-TOKEN header
```

#### CI/CD Token Flow

```
# GitHub Actions
1. Workflow runs with GITHUB_TOKEN
2. Action: Read token from environment
3. Action: Use token for artifact upload

# GitLab CI
1. Job runs with CI_JOB_TOKEN
2. Runner: Provide token to CLI
3. CLI: Use token for API calls
```

### H. Rate Limiting

#### Rate Limit Headers

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Maximum requests allowed |
| `X-RateLimit-Remaining` | Requests remaining |
| `X-RateLimit-Reset` | Unix timestamp when limit resets |
| `X-RateLimit-Used` | Requests used in current window |
| `Retry-After` | Seconds to wait before retry |

#### Rate Limit Tiers

| Tier | Limit | Window |
|------|-------|--------|
| Anonymous | 60 | hour |
| Authenticated | 1000 | hour |
| CI/CD | 5000 | hour |
| Enterprise | 10000 | hour |

#### Rate Limit Response

```json
{
  "message": "API rate limit exceeded",
  "documentation_url": "https://docs.phenotype.dev/rate-limiting",
  "rate": {
    "limit": 1000,
    "remaining": 0,
    "reset": 1712239200,
    "used": 1000
  }
}
```

### I. Webhook Events

#### Event Types

| Event | Description |
|-------|-------------|
| `artifact.uploaded` | Artifact uploaded |
| `artifact.downloaded` | Artifact downloaded |
| `artifact.deleted` | Artifact deleted |
| `artifact.verified` | Artifact verified |
| `index.updated` | Index regenerated |
| `cleanup.started` | Retention cleanup started |
| `cleanup.completed` | Retention cleanup finished |

#### Webhook Payload

```json
{
  "event": "artifact.uploaded",
  "timestamp": "2026-04-04T12:00:00Z",
  "id": "evt-abc123",
  "artifact": {
    "id": "pheno-cli-v1.2.0-linux-amd64",
    "name": "pheno-cli",
    "version": "1.2.0",
    "platform": "linux-amd64",
    "digest": "sha256:abc123...",
    "size": 15234567,
    "uploadedBy": "github:username",
    "uploadedAt": "2026-04-04T12:00:00Z"
  },
  "repository": {
    "name": "phenotype/artifacts",
    "url": "https://github.com/phenotype/artifacts"
  }
}
```

#### Webhook Delivery

```
POST /webhook/endpoint HTTP/1.1
Host: example.com
X-Phenotype-Event: artifact.uploaded
X-Phenotype-Delivery: evt-abc123
X-Phenotype-Signature: sha256=...
Content-Type: application/json

{payload}
```

#### Webhook Verification

```python
import hmac
import hashlib

def verify_webhook(payload, signature, secret):
    expected = 'sha256=' + hmac.new(
        secret.encode(),
        payload.encode(),
        hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(expected, signature)
```

### J. Pagination

#### Offset Pagination

```
GET /api/v1/artifacts?limit=20&offset=40
```

Response:
```json
{
  "artifacts": [...],
  "pagination": {
    "total": 247,
    "limit": 20,
    "offset": 40,
    "hasMore": true,
    "nextOffset": 60,
    "prevOffset": 20
  }
}
```

#### Cursor Pagination

```
GET /api/v1/artifacts?limit=20&cursor=eyJpZCI6MTIzfQ
```

Response:
```json
{
  "artifacts": [...],
  "pagination": {
    "cursor": "eyJpZCI6MTQ1fQ",
    "hasMore": true,
    "nextCursor": "eyJpZCI6MTY1fQ",
    "prevCursor": "eyJpZCI6MTIzfQ"
  }
}
```

### K. Sorting

#### Sort Parameters

| Parameter | Description |
|-----------|-------------|
| `sort=name` | Sort by name (ascending) |
| `sort=-name` | Sort by name (descending) |
| `sort=created` | Sort by creation date (oldest first) |
| `sort=-created` | Sort by creation date (newest first) |
| `sort=size` | Sort by size (smallest first) |
| `sort=-size` | Sort by size (largest first) |

### L. Filtering

#### Filter Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `eq` | Equal | `name=pheno-cli` |
| `ne` | Not equal | `platform!=windows` |
| `gt` | Greater than | `size>1000000` |
| `gte` | Greater or equal | `created>=2026-04-01` |
| `lt` | Less than | `size<10000000` |
| `lte` | Less or equal | `created<=2026-04-30` |
| `in` | In list | `platform=in:linux-amd64,linux-arm64` |
| `nin` | Not in list | `kind=nin:ci,report` |
| `like` | Pattern match | `name=like:pheno-*` |
| `exists` | Field exists | `labels.project=exists` |

### M. Batch Operations

#### Batch Upload

```
POST /api/v1/artifacts/batch
Content-Type: multipart/form-data

--boundary
Content-Disposition: form-data; name="artifacts"; filename="artifact1.tar.gz"
...
--boundary
Content-Disposition: form-data; name="artifacts"; filename="artifact2.tar.gz"
...
--boundary--
```

#### Batch Delete

```
POST /api/v1/artifacts/batch-delete
Content-Type: application/json

{
  "ids": [
    "pheno-cli-v1.0.0",
    "pheno-cli-v1.1.0"
  ],
  "confirm": true
}
```

### N. Export/Import

#### Export Format

```
export/
├── manifest.json
├── blobs/
│   └── sha256/
│       └── ab/
│           └── cd1234...
└── metadata/
    └── artifacts/
        └── pheno-cli-v1.2.0.json
```

#### Export Command

```bash
pheno artifacts export \
  --format tarball \
  --include blobs \
  --include manifests \
  --filter "project=phenotype" \
  --output export.tar.gz
```

#### Import Command

```bash
pheno artifacts import \
  --input export.tar.gz \
  --strategy merge \
  --verify
```

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-04-04 | Initial specification |

---

## Glossary

| Term | Definition |
|------|------------|
| **Artifact** | A named, versioned binary object with metadata |
| **Blob** | Content-addressable binary data |
| **Content-addressable** | Identified by hash of content, not location |
| **Digest** | Cryptographic hash (SHA-256) of content |
| **Driver** | Pluggable storage backend implementation |
| **Index** | Master list of all artifacts |
| **Kind** | Artifact category (binary, archive, report) |
| **LFS** | Git Large File Storage |
| **Manifest** | JSON document describing an artifact |
| **Platform** | Target operating system and architecture |
| **Provenance** | Metadata about artifact origin and build |
| **Retention** | Policy for artifact lifecycle |
| **Tag** | Human-readable label for version |
| **Webhook** | HTTP callback for events |
| **Cursor** | Pagination continuation token |
| **Batch** | Multi-item operation |
| **Rate Limit** | Request throttling mechanism |
| **Export** | Data extraction for migration |
| **Import** | Data ingestion from external source |

---

*Specification Version: 1.0*
*Last Updated: 2026-04-04*
*Status: Draft*
