# Archived: Wrong Content Directory

**Status**: ARCHIVED  
**Date**: 2026-04-02  
**Reason**: Content Mismatch

## Issue

This directory was named `hexagon-ts` but contained `phenotype-cli-core` (a Go CLI library) instead of TypeScript hexagonal template content.

## Resolution

Archived as part of repository cleanup per `plans/20260402-repo-graph-optimization-v1.md`.

The legitimate hexagonal templates are maintained in:
- `template-lang-typescript/` - TypeScript template
- `template-lang-python/` - Python template
- `template-lang-rust/` - Rust template
- `hexagon-zig/` - Zig hexagonal template (if exists)

## Action

This directory should be removed or kept as archive reference only. The real CLI core project should be accessed at its canonical location.
