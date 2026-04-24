#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

bash "$repo_root/scripts/validate-docs.sh"

build_file="$repo_root/templates/zig/build.zig"
test -f "$build_file"

if grep -q '\\\\n' "$build_file"; then
  echo "[FAIL] $build_file contains literal '\\n'"
  exit 1
fi

if command -v zig >/dev/null; then
  zig fmt --check "$build_file"
  zig build --build-file "$build_file"
else
  if [[ "${REQUIRE_ZIG:-0}" == "1" ]]; then
    echo "[FAIL] zig is required (REQUIRE_ZIG=1) but not installed"
    exit 1
  fi
  echo "[WARN] zig not found, skipping build checks"
fi

echo "[OK] scaffold smoke passed"
