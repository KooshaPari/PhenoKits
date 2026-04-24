# phenotype-infrakit Specification

## 1. Overview

phenotype-infrakit provides infrastructure utilities for the Phenotype ecosystem.

### 1.1 Components

- **phenotype-resources**: System resource monitoring
- **phenotype-path-resolve**: PATH resolution utilities

## 2. Resource Monitoring

### 2.1 Resource Snapshot

```rust
#[derive(Debug, Serialize)]
pub struct ResourceSnapshot {
    pub fd_used: u32,
    pub fd_limit: u32,
    pub mem_rss_mb: f64,
    pub mem_available_mb: f64,
    pub cpu_count: u32,
    pub load_1m: f64,
    pub load_5m: f64,
    pub load_15m: f64,
}
```

### 2.2 Sampling

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

## 3. PATH Resolution

### 3.1 PathResolver

```rust
pub struct PathResolver {
    skip_dirs: Vec<PathBuf>,
}

impl PathResolver {
    pub fn new() -> Self {
        Self { skip_dirs: Vec::new() }
    }
    
    pub fn with_skip_dirs(skip_dirs: Vec<String>) -> Self {
        Self {
            skip_dirs: skip_dirs.iter().map(PathBuf::from).collect(),
        }
    }
    
    pub fn resolve(&self, name: &str) -> Option<String> {
        which(name).ok()
            .filter(|p| !self.is_in_skip_dirs(p))
            .map(|p| p.to_string_lossy().to_string())
    }
    
    pub fn resolve_many(&self, names: &[&str]) -> HashMap<String, Option<String>> {
        names.iter()
            .map(|&n| (n.to_string(), self.resolve(n)))
            .collect()
    }
}

pub fn resolve_binary(name: &str) -> Option<String> {
    PathResolver::new().resolve(name)
}
```

## 4. Usage

```rust
// Resource monitoring
let snapshot = phenotype_resources::sample();
println!("Memory: {} MB", snapshot.mem_rss_mb);

// PATH resolution
let resolver = PathResolver::with_skip_dirs(vec!["/usr/bin".to_string()]);
let path = resolver.resolve("python3");
```

---

*Specification Version: 1.0*
