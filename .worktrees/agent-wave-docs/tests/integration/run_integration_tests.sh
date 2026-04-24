#!/usr/bin/env bash
# Entry point for all agent-wave integration tests
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
echo "=== agent-wave integration tests ==="
bash "$DIR/scripts.test.sh"
