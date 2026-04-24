# API Reference

`nexus` is a library for service registry and discovery.

## Core Surface

- `Registry` for register/deregister/discover operations
- `Service` for address, tags, and health metadata
- Load balancing strategies for round-robin, random, and consistent hash

## Architectural Notes

- In-memory only
- Lock-free hot path with `dashmap`
- No binary surface; embed the library in the calling process
