# Test Coverage Matrix

**Project**: phenotype-patch  
**Document Version**: 1.1  
**Last Updated**: 2026-04-02

---

## Coverage Summary

| Metric | Value |
|--------|-------|
| Functional Requirements | 25 (FR-PATCH-INTEGRATION-001 to 025) |
| Test Files | 2 (src/*.rs:6, tests/filesystem_integration.rs:24) |
| Test Functions | 30 (6 unit + 24 integration) |
| Coverage Target | 80% |
| Current Coverage | ~80% (estimated) |

---

## Test Categories

### Unit Tests
- **Location**: `src/*.rs` (#[cfg(test)] modules)
- **Purpose**: Test individual components in isolation
- **Count**: 6 tests
- **Coverage Target**: 90%

### Integration Tests
- **Location**: `tests/filesystem_integration.rs`
- **Purpose**: Test filesystem operations and patch application
- **Count**: 24 tests
- **Coverage Target**: 75%

---

## FR to Test Coverage Mapping

| FR ID | Description | Test Location | Status |
|-------|-------------|--------------|--------|
| FR-PATCH-001 | Parse unified diff | src/parse.rs | Covered |
| FR-PATCH-002 | Apply diff to content | src/apply.rs | Covered |
| FR-PATCH-003 | Error handling | src/apply.rs | Covered |
| FR-PATCH-004 | Create diff from content | src/create.rs | Covered |
| FR-PATCH-005 | Merge diffs | src/merge.rs | N/A |
| FR-PATCH-INTEGRATION-001 | Integration test suite | tests/filesystem_integration.rs:1 | Covered |
| FR-PATCH-INTEGRATION-002 | Round-trip patch | tests/filesystem_integration.rs:33 | Covered |
| FR-PATCH-INTEGRATION-003 | Multiple files | tests/filesystem_integration.rs:66 | Covered |
| FR-PATCH-INTEGRATION-004 | Context lines | tests/filesystem_integration.rs:98 | Covered |
| FR-PATCH-INTEGRATION-005 | Addition at end | tests/filesystem_integration.rs:126 | Covered |
| FR-PATCH-INTEGRATION-006 | Deletion at end | tests/filesystem_integration.rs:160 | Covered |
| FR-PATCH-INTEGRATION-007 | Parse diff format | tests/filesystem_integration.rs:188 | Covered |
| FR-PATCH-INTEGRATION-008 | Hunk structure | tests/filesystem_integration.rs:205 | Covered |
| FR-PATCH-INTEGRATION-009 | Context mismatch | tests/filesystem_integration.rs:229 | Covered |
| FR-PATCH-INTEGRATION-010 | Invalid diff format | tests/filesystem_integration.rs:256 | Covered |
| FR-PATCH-INTEGRATION-011 | File permissions | tests/filesystem_integration.rs:275 | Covered |
| FR-PATCH-INTEGRATION-012 | Binary file handling | tests/filesystem_integration.rs:303 | Covered |
| FR-PATCH-INTEGRATION-013 | Large file | tests/filesystem_integration.rs:346 | Covered |
| FR-PATCH-INTEGRATION-014 | Empty file | tests/filesystem_integration.rs:384 | Covered |
| FR-PATCH-INTEGRATION-015 | Unicode content | tests/filesystem_integration.rs:405 | Covered |
| FR-PATCH-INTEGRATION-016 | Nested directories | tests/filesystem_integration.rs:427 | Covered |
| FR-PATCH-INTEGRATION-017 | Backup creation | tests/filesystem_integration.rs:456 | Covered |
| FR-PATCH-INTEGRATION-018 | Error handling | tests/filesystem_integration.rs:509 | Covered |
| FR-PATCH-INTEGRATION-019 | Concurrent files | tests/filesystem_integration.rs:540 | Covered |
| FR-PATCH-INTEGRATION-020 | Real diff format | tests/filesystem_integration.rs:580 | Covered |
| FR-PATCH-INTEGRATION-021 | Single line file | tests/filesystem_integration.rs:597 | Covered |
| FR-PATCH-INTEGRATION-022 | Trailing newline | tests/filesystem_integration.rs:617 | Covered |
| FR-PATCH-INTEGRATION-023 | Windows line endings | tests/filesystem_integration.rs:637 | Covered |
| FR-PATCH-INTEGRATION-024 | Symlink handling | tests/filesystem_integration.rs:659 | Covered |
| FR-PATCH-INTEGRATION-025 | Long lines | tests/filesystem_integration.rs:681 | Covered |

---

## Coverage Gaps

### Critical Gaps
1. Merge functionality (src/merge.rs)
2. Binary diff generation

### Partial Coverage
1. Large file edge cases (100+ hunks)
2. Real-world diff format compatibility

---

## Recommendations

1. Add tests for merge functionality
2. Add property-based tests for diff generation
3. Add compatibility tests with real diff formats
