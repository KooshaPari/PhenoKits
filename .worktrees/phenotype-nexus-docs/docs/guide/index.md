---
type: howto
evidence_bundle: guide-index
---

<EvidenceBundle bundle="guide-index" title="Evidence Bundle: Getting Started Guide" />

# Guide

Follow these steps to install and configure `nexus` in your Rust project.

## Installation

```toml
[dependencies]
nexus = { git = "https://github.com/KooshaPari/nexus" }
```

## Registering Services

```rust
let registry = Registry::new();
registry.register(Service::new("user-svc", "localhost:8080")).await?;
```

## Discovering Services

```rust
let services = registry.discover("user-svc").await?;
```
