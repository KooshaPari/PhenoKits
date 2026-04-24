# Contributing to phenotype-logging-zig

## Archived Notice

This repository has been **archived** as of 2026-03-25.

## New Location

All development has moved to:
- **Repository**: `libs/logging-zig`
- **Package**: `logging-zig`

## Historical Information

This was a structured logging library for Zig providing zero-allocation logging with compile-time filtering.

### Original Development Setup

```bash
# Install Zig 0.15.x
# Clone the repository
git clone <repo-url>
cd phenotype-logging-zig

# Run tests
zig build test

# Build library
zig build
```

### Project Structure

```
src/
├── lib.zig           # Core logging types
├── adapters/
│   ├── stderr.zig    # Stderr output
│   └── file.zig      # File output
└── interface.zig     # Adapter interface
```

## Contributing to Successor

To contribute to the active project:

1. Visit `libs/logging-zig`
2. Follow contribution guidelines there
3. Reference this archive for historical context

## License

MIT
