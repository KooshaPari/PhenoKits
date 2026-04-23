# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Archived] - 2026-03-25

### Migration
- Repository archived and migrated to `libs/logging-zig`
- All future development continues in the new location

## [1.0.0] - 2026-03-25

### Added
- Structured logging with JSON output
- Compile-time log level filtering
- Stderr adapter with ANSI colors
- File adapter with rotation support
- Zero-allocation hot path
- Comptime adapter interface
- Integration tests

### Features
- Standard log levels: trace, debug, info, warn, err, fatal
- Key-value structured fields
- Human-readable and JSON formats
- Pluggable transport adapters

## [0.1.0] - 2026-03-01

### Added
- Initial project setup
- Core logging interface
- Basic stderr adapter
- Zig 0.15 compatibility
