#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

bash "$repo_root/scripts/validate-docs.sh"

manifest="$repo_root/contracts/template.manifest.json"
if [[ ! -f "$manifest" ]]; then
  echo "[FAIL] missing $manifest"
  exit 1
fi

generated="$repo_root/templates/mojo/main.mojo"
if [[ ! -f "$generated" ]]; then
  echo "[FAIL] missing $generated"
  exit 1
fi

if grep -F "\\n" "$generated" >/dev/null; then
  echo "[FAIL] $generated contains literal '\\n'"
  exit 1
fi

if command -v mojo >/dev/null; then
  mojo --version >/dev/null
else
  if [[ "${REQUIRE_MOJO:-0}" == "1" ]]; then
    echo "[FAIL] mojo is required (REQUIRE_MOJO=1) but not installed"
    exit 1
  fi
  echo "[WARN] mojo not found, skipping mojo run"
fi

echo "[OK] scaffold smoke passed"
