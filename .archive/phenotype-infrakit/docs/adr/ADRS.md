# phenotype-infrakit ADRs

## ADR 001: Native Resource Sampling Without Subprocess Spawns

### Status: Accepted

### Context

Resource monitoring often relies on subprocess calls (lsof, vm_stat, ps) which are slow and resource-intensive. We need efficient native sampling.

Options:
- Subprocess calls: Portable but slow
- /proc filesystem: Linux-only but fast
- libc calls: Portable across Unix systems
- Platform APIs: Native but platform-specific

### Decision

Use libc calls and /proc filesystem on Linux, fall back to subprocess on macOS:

```rust
pub fn sample() -> ResourceSnapshot {
    let (fd_used, fd_limit) = get_fd_usage();
    let (mem_rss_mb, mem_available_mb) = get_memory_mb();
    let cpu_count = get_cpu_count();
    let (load_1m, load_5m, load_15m) = get_load_avg();
    
    ResourceSnapshot {
        fd_used,
        fd_limit,
        mem_rss_mb,
        mem_available_mb,
        cpu_count,
        load_1m,
        load_5m,
        load_15m,
    }
}
```

### Consequences

#### Positive

1. **Performance**: No subprocess overhead
2. **Reliability**: Fewer failure modes
3. **Security**: No shell injection risks
4. **Precision**: Raw kernel values

#### Negative

1. **Platform differences**: Linux vs macOS
2. **Complexity**: Multiple code paths
3. **Maintenance**: Platform-specific testing

---

## ADR 002: Skip Directory Support for PATH Resolution

### Status: Accepted

### Context

Multiple versions of tools may exist in PATH. Users need to exclude specific directories (e.g., system paths with outdated versions).

### Decision

Implement PATH resolver with skip directory support:

```rust
pub struct PathResolver {
    skip_dirs: Vec<PathBuf>,
}

impl PathResolver {
    pub fn resolve(&self, name: &str) -> Option<String> {
        let path = which(name).ok()?;
        if self.is_in_skip_dirs(&path) {
            None
        } else {
            Some(path.to_string_lossy().to_string())
        }
    }
}
```

### Consequences

#### Positive

1. **Control**: Exclude unwanted paths
2. **Security**: Avoid system directories
3. **Flexibility**: Custom resolution logic

#### Negative

1. **Configuration**: Requires setup
2. **Performance**: Extra path checks

---

*ADRs phenotype-infrakit - Version 1.0*
