#!/usr/bin/env bash
# Integration tests for agent-wave governance scripts
# Traces to: FR-GOV-001, FR-GOV-002, FR-GOV-003, FR-GOV-004, FR-GOV-005

set -euo pipefail

PASS=0
FAIL=0
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"

pass() { echo "PASS: $1"; PASS=$((PASS+1)); }
fail() { echo "FAIL: $1"; FAIL=$((FAIL+1)); }

# FR-GOV-003: quality-gate.sh exists and is runnable via bash
if [ -f "$PROJECT_ROOT/scripts/quality-gate.sh" ] && bash -n "$PROJECT_ROOT/scripts/quality-gate.sh" 2>/dev/null; then
    pass "FR-GOV-003: quality-gate.sh present and valid bash"
else
    fail "FR-GOV-003: quality-gate.sh missing or invalid"
fi

# FR-GOV-004: security-guard.sh exists and is runnable via bash
if [ -f "$PROJECT_ROOT/scripts/security-guard.sh" ] && bash -n "$PROJECT_ROOT/scripts/security-guard.sh" 2>/dev/null; then
    pass "FR-GOV-004: security-guard.sh present and valid bash"
else
    fail "FR-GOV-004: security-guard.sh missing or invalid"
fi

# FR-GOV-005: policy-gate.sh exists and is runnable via bash
if [ -f "$PROJECT_ROOT/scripts/policy-gate.sh" ] && bash -n "$PROJECT_ROOT/scripts/policy-gate.sh" 2>/dev/null; then
    pass "FR-GOV-005: policy-gate.sh present and valid bash"
else
    fail "FR-GOV-005: policy-gate.sh missing or invalid"
fi

# FR-GOV-002: AGENTS.md and CLAUDE.md at root
[ -f "$PROJECT_ROOT/AGENTS.md" ] && pass "FR-GOV-002: AGENTS.md present" || fail "FR-GOV-002: AGENTS.md missing"
[ -f "$PROJECT_ROOT/CLAUDE.md" ] && pass "FR-GOV-002: CLAUDE.md present" || fail "FR-GOV-002: CLAUDE.md missing"

# FR-GOV-001: .pre-commit-config.yaml exists
[ -f "$PROJECT_ROOT/.pre-commit-config.yaml" ] && pass "FR-GOV-001: .pre-commit-config.yaml present" || fail "FR-GOV-001: .pre-commit-config.yaml missing"

# quality-gate.sh references linting
grep -q "yamllint\|lint" "$PROJECT_ROOT/scripts/quality-gate.sh" 2>/dev/null && pass "FR-GOV-003: quality-gate references lint" || fail "FR-GOV-003: quality-gate missing lint"

# policy-gate.sh references policy enforcement
grep -q "merge\|namespace\|policy" "$PROJECT_ROOT/scripts/policy-gate.sh" 2>/dev/null && pass "FR-GOV-005: policy-gate references policy" || fail "FR-GOV-005: policy-gate missing policy"

# security-guard.sh references security checks
grep -q "security\|pre-commit\|scan" "$PROJECT_ROOT/scripts/security-guard.sh" 2>/dev/null && pass "FR-GOV-004: security-guard references security" || fail "FR-GOV-004: security-guard missing security checks"

# VERSION file non-empty
[ -s "$PROJECT_ROOT/VERSION" ] && pass "FR-GOV: VERSION non-empty" || fail "FR-GOV: VERSION empty/missing"

# Taskfile.yml present
[ -f "$PROJECT_ROOT/Taskfile.yml" ] && pass "FR-GOV: Taskfile.yml present" || fail "FR-GOV: Taskfile.yml missing"

# All scripts have valid bash syntax
for script in quality-gate.sh security-guard.sh policy-gate.sh self-merge-gate.sh; do
    bash -n "$PROJECT_ROOT/scripts/$script" 2>/dev/null && pass "FR-GOV: $script valid bash syntax" || fail "FR-GOV: $script syntax error"
done

echo ""
echo "Results: $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ] && exit 0 || exit 1
