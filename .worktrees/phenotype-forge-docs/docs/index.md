---
layout: home
title: phenotype-forge - CLI Task Runner and Build Orchestrator
titleTemplate: false
---

# phenotype-forge

CLI task runner and build orchestrator for the Phenotype ecosystem.

## Overview

`phenotype-forge` is a Rust CLI tool providing parallel task execution with automatic topological sorting, dependency graph resolution, and hot reload capabilities.

## Features

- **Parallel Execution**: Run tasks concurrently with automatic topological sort
- **Dependency Graph**: Automatic resolution of task dependencies
- **Hot Reload**: Watch files and restart on changes
- **Plugin System**: Extend with custom task definitions

## Quick Start

```bash
# Run tasks
forge run build --parallel

# Watch mode
forge watch --pattern "src/**/*.rs"

# Show dependency graph
forge graph
```

## Links

- [Repository](https://github.com/KooshaPari/phenotype-forge)
