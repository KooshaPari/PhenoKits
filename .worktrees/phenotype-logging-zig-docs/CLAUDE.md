# phenotype-logging-zig

Structured logging library for Zig with minimal allocations. Provides JSON-formatted output and structured log levels.

## Stack
- Language: Zig
- Key deps: Zig build system (`build.zig`)
- Status: Archived (see ARCHIVED.md)

## Structure
- `src/`: Zig library source
  - Structured log entry types
  - JSON serialization with zero/minimal heap allocations
  - Log level filtering

## Key Patterns
- Comptime-based log level filtering (no runtime cost for disabled levels)
- Allocation-free fast path for common log operations
- JSON output formatted for structured log aggregators (Loki, ELK, etc.)

## Adding New Functionality
- This repo is archived; prefer the successor indicated in ARCHIVED.md
- If extending: add new log fields as comptime-known struct fields
- Run `zig build test` to verify
