#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
files=(
  "$repo_root/docs/index.md"
  "$repo_root/docs/UPGRADE.md"
  "$repo_root/docs/BRANCH_PROTECTION.md"
  "$repo_root/README.md"
  "$repo_root/Taskfile.yml"
  "$repo_root/contracts/template.manifest.json"
  "$repo_root/contracts/reconcile.rules.yaml"
)
for f in "${files[@]}"; do
  if [[ ! -f "$f" ]]; then
    echo "[FAIL] missing docs asset $f"
    exit 1
  fi
  if [[ ! -s "$f" ]]; then
    echo "[FAIL] empty docs asset $f"
    exit 1
  fi
 done
 echo "[OK] docs validate"
