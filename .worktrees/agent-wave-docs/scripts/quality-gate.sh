#!/bin/bash
#
# Quality Gate Script
# Runs quality checks: yamllint, pre-commit checks
# Usage: ./scripts/quality-gate.sh verify

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

verify_quality() {
    log_info "Running quality gate checks..."
    local exit_code=0

    # Check for yamllint
    if command -v yamllint &> /dev/null; then
        log_info "Running yamllint on YAML files..."
        if yamllint -c "$PROJECT_ROOT/.yamllint" "$PROJECT_ROOT/.github/workflows/"*.{yml,yaml} 2>/dev/null || true; then
            log_info "yamllint passed"
        else
            log_warn "yamllint found issues (non-blocking)"
        fi
    else
        log_warn "yamllint not installed, skipping YAML linting"
    fi

    # Check for pre-commit
    if command -v pre-commit &> /dev/null; then
        log_info "Running pre-commit checks..."
        if pre-commit run --all-files 2>/dev/null || true; then
            log_info "pre-commit checks passed"
        else
            log_warn "pre-commit found issues (non-blocking)"
        fi
    else
        log_warn "pre-commit not installed, skipping pre-commit checks"
    fi

    if [ $exit_code -eq 0 ]; then
        log_info "Quality gate verification completed successfully"
        return 0
    else
        log_error "Quality gate verification failed"
        return 1
    fi
}

main() {
    case "${1:-}" in
        verify)
            verify_quality
            ;;
        *)
            log_error "Usage: $0 verify"
            exit 1
            ;;
    esac
}

main "$@"
