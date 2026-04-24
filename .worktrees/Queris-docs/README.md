# Queris

> Query system for distributed data access

## Overview

Queris provides a unified query interface for accessing and aggregating data across distributed systems.

## Features

- **Query Builder**: Type-safe query construction
- **Data Federation**: Query across multiple sources
- **Caching**: Intelligent query result caching
- **Optimization**: Query plan optimization

## Quick Start

```bash
# Install
pip install queris

# Query data
queris query --source postgres --sql "SELECT * FROM users"
```

## Documentation

- [Specification](SPEC.md) - Technical details
- [Implementation Plan](PLAN.md) - Development roadmap

## License

MIT
