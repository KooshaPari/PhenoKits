# AGENTS.md - Agent Guidelines for phenotype-logging-zig

## Agent Working Context

**Project**: phenotype-logging-zig
**Type**: Zig library (ARCHIVED)
**Primary Language**: Zig
**Status**: Archived - Migrated to libs/logging-zig

## Important Notice

This repository is **ARCHIVED** as of 2026-03-25. The code has been migrated to `libs/logging-zig`.

## Archive Status

- **Source**: phenotype-logging-zig
- **Target**: libs/logging-zig
- **Migration Date**: 2026-03-25

## Development Environment (Historical)

### Prerequisites

- Zig 0.15.x

### Build Commands

```bash
# Run tests
zig build test

# Build library
zig build
```

## Project Structure

```
phenotype-logging-zig/
├── src/
│   ├── lib.zig           # Core logging
│   ├── adapters/
│   │   ├── stderr.zig    # Stderr output
│   │   └── file.zig      # File output
│   └── interface.zig     # Adapter interface
├── build.zig             # Build configuration
└── build.zig.zon         # Package manifest
```

## Agent Instructions

**Do not modify this repository.** All changes should be made to the successor:
- **New Location**: `libs/logging-zig`
- **Repository**: Use the migrated version instead

## Historical Context

This was a structured logging library for Zig providing:
- Zero-allocation logging
- Compile-time log level filtering
- JSON and ANSI output
- Pluggable adapters

## License

MIT
