# ADR-001: Artifact Storage Backend Selection

## Status

Accepted

## Context

The artifacts system needs to store binary data with the following requirements:

1. Must support artifacts from 1KB to 10GB
2. Must be compatible with Git LFS workflow
3. Must allow local development without cloud dependencies
4. Must support future cloud migration without data migration

## Decision

We will implement a **pluggable storage driver interface** with two initial implementations:

1. **Filesystem Driver** (default): Local filesystem with content-addressable storage
2. **S3-compatible Driver**: AWS S3, MinIO, or compatible services

### Storage Layout (Content-Addressable)

```
artifacts/
└── storage/
    └── blobs/
        └── sha256/
            └── ab/
                └── cd1234...ef   # First 4 chars of digest as prefix
```

### Driver Interface

```go
type StorageDriver interface {
    // Read
    Get(ctx context.Context, digest string) (io.ReadCloser, error)
    Stat(ctx context.Context, digest string) (Descriptor, error)
    
    // Write
    Put(ctx context.Context, digest string, r io.Reader) error
    
    // Delete
    Delete(ctx context.Context, digest string) error
    
    // Listing
    Walk(ctx context.Context, fn WalkFn) error
}
```

## Consequences

### Positive

- Local development requires no external services
- Git LFS integration is natural (LFS uses same content-addressable pattern)
- Easy migration path: sync blobs to S3 when ready
- Deduplication is automatic (same content = same digest)

### Negative

- Filesystem driver doesn't scale horizontally
- No built-in CDN for filesystem driver
- S3 driver adds deployment complexity

## Alternatives Considered

| Alternative | Reasoning |
|-------------|-----------|
| PostgreSQL blobs | Would couple storage to database; poor performance for large files |
| Only S3 | Would prevent local development without AWS credentials |
| Custom storage | Would require building what already exists |

## References

- OCI Distribution Spec storage model
- Git LFS internals: https://github.com/git-lfs/git-lfs/blob/main/docs/README.md
