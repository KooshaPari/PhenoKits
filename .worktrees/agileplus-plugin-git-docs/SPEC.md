# agileplus-plugin-git Specification

## Architecture
```
┌─────────────────────────────────────────────────────┐
│           Plugin Git (Rust)                        │
├─────────────────────────────────────────────────────┤
│  ┌──────────┐    ┌──────────────────────────────┐ │
│  │   git2   │◀───│   Git Operations             │ │
│  │   Rust   │    │  (clone, push, pull, diff)    │ │
│  └──────────┘    └──────────────────────────────┘ │
│                                                │
│  ┌──────────────────────────────────────────┐  │
│  │     Plugin Trait Implementation            │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

## Components

| Component | Responsibility | Public API |
|-----------|----------------|-----------|
| GitAdapter | git2 wrapper | `clone()`, `push()`, `pull()`, `diff()` |
| Repository | Repo lifecycle | `open()`, `close()`, `status()` |
| Branch | Branch operations | `create_branch()`, `merge()`, `rebase()` |

## Data Models

```rust
struct GitConfig {
    url: String,
    branch: String,
    auth: GitAuth,
}

enum GitAuth {
    None,
    SshKey(PathBuf),
    Token(String),
}

struct DiffResult {
    files: Vec<FileDiff>,
    stats: DiffStats,
}
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Clone (shallow) | <30s |
| Diff | <500ms |
| Push/Pull | <60s |
| Concurrent repos | 10 max |