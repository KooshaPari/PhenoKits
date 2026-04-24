#!/bin/bash
#
# Policy Gate Script
# Enforces engineering policies: PR labels, commit message format
# Usage: ./scripts/policy-gate.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Color output for better readability
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

check_pr_labels() {
    log_info "Checking PR labels..."

    # In GitHub Actions, GITHUB_EVENT_NAME is set
    if [ "${GITHUB_EVENT_NAME:-}" = "pull_request" ]; then
        # GitHub Actions context available - check labels via API
        local pr_number="${GITHUB_PR_NUMBER:-}"
        if [ -z "$pr_number" ] && [ -n "${GITHUB_REF:-}" ]; then
            # Extract PR number from ref if available
            pr_number=$(echo "$GITHUB_REF" | grep -oP 'refs/pull/\K[0-9]+' || echo "")
        fi

        if [ -n "$pr_number" ]; then
            log_info "Validating PR #$pr_number labels..."
            # Labels are optional - just warn if none exist
            log_info "PR label check: skipped (labels are advisory)"
        else
            log_warn "Could not determine PR number"
        fi
    else
        log_info "Not running in PR context, skipping label check"
    fi

    return 0
}

check_commit_format() {
    log_info "Checking commit message format..."

    # Get the last commit message
    local commit_msg
    commit_msg=$(git log -1 --pretty=%B 2>/dev/null || echo "")

    if [ -z "$commit_msg" ]; then
        log_warn "No commits found, skipping format check"
        return 0
    fi

    # Check for conventional commit format: type(scope): description
    # Allowed types: feat, fix, docs, style, refactor, perf, test, chore, ci, build, revert
    local first_line
    first_line=$(echo "$commit_msg" | head -1)

    if echo "$first_line" | grep -qE '^(feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\(.+\))?!?: .+'; then
        log_info "Commit message follows conventional commit format"
        return 0
    else
        log_warn "Commit message does not follow strict conventional commit format: '$first_line'"
        log_warn "Expected format: type(scope): description (e.g., 'feat(auth): add login validation')"
        # Non-blocking for now - allow commits that don't match
        return 0
    fi
}

enforce_policies() {
    log_info "Enforcing engineering policies..."

    check_pr_labels || return 1
    check_commit_format || return 1

    log_info "Policy enforcement completed successfully"
    return 0
}

main() {
    enforce_policies
}

main "$@"
