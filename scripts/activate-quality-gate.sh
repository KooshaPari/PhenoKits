#!/bin/bash
# Activate Phenotype Quality Gate

REPOS_DIR="$(cd "$(dirname "$0")" && pwd)"
AGILEPLUS_DIR="$REPOS_DIR/AgilePlus"

echo "Activating Quality Gate..."

# Install binaries
cd "$AGILEPLUS_DIR" && ./install-quality-gate.sh 2>/dev/null || echo "Install manually: chmod +x bin/*"

echo "Activation complete!"
