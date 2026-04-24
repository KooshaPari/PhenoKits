# Dep Guard Core — Supply Chain Security

## Overview

Malicious dependency analysis and supply chain security guard.

## Features

### Security Operations

1. **Dependency Analysis** — High-velocity multi-source dependency resolution
2. **Static Triage** — Heuristic and AST parsing for malicious code detection
3. **LLM Analysis** — Agentic LLM deep analysis integration
4. **Reporting** — Alerting and reporting mechanisms

## Requirements

- FR-001: Parse Cargo.lock and dependency trees
- FR-002: AST parsing for .pth/setup.py scanning
- FR-003: LLM integration for deep analysis
- FR-004: Report generation and alerting
- FR-005: Configurable thresholds

## Architecture

```
src/
├── lib.rs              # Public API
├── analyzer.rs         # Dependency analyzer
├── parser.rs           # AST and static analysis
├── llm.rs              # LLM integration
├── reporter.rs         # Alert and report generation
└── scanner.rs          # Vulnerability scanner
```
