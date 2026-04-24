# xDD Lib Core — Testing Utilities

## Overview

Cross-cutting xDD utilities library for Rust projects.

## Features

### Testing Patterns

1. **Property Testing** — Reusable strategies for proptest/quickcheck
2. **Contract Testing** — Port/Adapter verification framework
3. **Mutation Testing** — Coverage tracking utilities
4. **SpecDD** — Specification parsing and validation

## Requirements

- FR-001: Provide common Arbitrary implementations
- FR-002: Contract testing framework for ports
- FR-003: Mutation testing support
- FR-004: SpecDD specification parser
- FR-005: Zero-cost abstractions

## Architecture

```
src/
├── lib.rs              # Public API
├── property/           # Property testing strategies
├── contract/           # Contract testing framework
├── mutation/           # Mutation testing utilities
└── specdd/             # SpecDD parser
```
