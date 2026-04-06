#!/bin/bash
#
# Security Guard Script
# Runs security audits: trufflehog, secret pattern detection
# Usage: ./scripts/security-guard.sh audit

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

detect_secrets_patterns() {
    log_info "Scanning for hardcoded secrets..."

    local found_secrets=0
    local secret_patterns=(
        "password\s*=\s*['\"][^'\"]*['\"]"
        "api[_-]?key\s*=\s*['\"][^'\"]*['\"]"
        "secret\s*=\s*['\"][^'\"]*['\"]"
        "token\s*=\s*['\"][^'\"]*['\"]"
        "AWS_SECRET_ACCESS_KEY"
        "PRIVATE_KEY"
        "BEGIN RSA PRIVATE KEY"
    )

    # Search for patterns in tracked files (exclude common safe paths)
    for pattern in "${secret_patterns[@]}"; do
        if git grep -iE "$pattern" -- \
            ':!*.md' ':!*.txt' ':!*.json.example' ':!*.env.example' \
            ':!LICENSE' ':!CHANGELOG.md' 2>/dev/null; then
            found_secrets=$((found_secrets + 1))
        fi
    done

    if [ $found_secrets -gt 0 ]; then
        log_error "Found potential secrets in tracked files"
        return 1
    else
        log_info "No hardcoded secrets detected"
        return 0
    fi
}

run_trufflehog() {
    log_info "Running trufflehog scan..."

    if command -v trufflehog &> /dev/null; then
        if trufflehog git file://. --since-commit HEAD --only-verified --fail 2>/dev/null; then
            log_info "trufflehog scan completed"
            return 0
        else
            log_warn "trufflehog found verified secrets"
            return 1
        fi
    else
        log_warn "trufflehog not installed, skipping secret scan"
        return 0
    fi
}

audit_security() {
    log_info "Running security audit..."

    local exit_code=0

    # Run pattern-based secret detection
    if detect_secrets_patterns; then
        log_info "Secret pattern detection passed"
    else
        log_warn "Secret pattern detection found issues"
        exit_code=1
    fi

    # Run gitleaks if available
    if run_trufflehog; then
        log_info "trufflehog scan passed"
    else
        log_warn "trufflehog scan found issues"
        exit_code=$((exit_code + 1))
    fi

    if [ $exit_code -eq 0 ]; then
        log_info "Security audit completed successfully"
        return 0
    else
        log_error "Security audit found issues"
        return 1
    fi
}

main() {
    case "${1:-}" in
        audit)
            audit_security
            ;;
        *)
            log_error "Usage: $0 audit"
            exit 1
            ;;
    esac
}

main "$@"
