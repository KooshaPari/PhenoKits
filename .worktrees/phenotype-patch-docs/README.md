# patch

Unified diff and patch library for Rust. Parse, create, and apply patches.

## Features

- **Parse**: Unified, context, and side-by-side diffs
- **Create**: Generate diffs from text or structured data
- **Apply**: Apply patches with conflict detection
- **Merge**: Three-way merge with conflict markers

## Installation

```toml
[dependencies]
patch = { git = "https://github.com/KooshaPari/patch" }
```

## Usage

```rust
use patch::{diff, apply};

let old = "hello world";
let new = "hello rust";

let diff = diff(old, new)?;
apply(old, &diff)?;
```

## License

MIT
