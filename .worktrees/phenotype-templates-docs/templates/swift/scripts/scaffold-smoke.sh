#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

bash "$repo_root/scripts/validate-docs.sh"

files=(
  "$repo_root/contracts/template.manifest.json"
  "$repo_root/contracts/reconcile.rules.yaml"
  "$repo_root/templates/swift/Package.swift"
  "$repo_root/templates/swift/Sources/App/main.swift"
  "$repo_root/templates/swift/Tests/Unit/UnitTests.swift"
  "$repo_root/templates/swift/Tests/Integration/IntegrationTests.swift"
  "$repo_root/Taskfile.yml"
)

for f in "${files[@]}"; do
  test -f "$f" || { echo "[FAIL] missing $f"; exit 1; }
done

package_file="$repo_root/templates/swift/Package.swift"

if grep -q '\\\\n' "$package_file"; then
  echo "[FAIL] $package_file contains literal '\\n'"
  exit 1
fi

if command -v swift >/dev/null; then
  swift package dump-package --package-path "$repo_root/templates/swift"
else
  if [[ "${REQUIRE_SWIFT:-0}" == "1" ]]; then
    echo "[FAIL] swift is required (REQUIRE_SWIFT=1) but not installed"
    exit 1
  fi
  echo "[WARN] swift not found, skipping package validation"
fi

echo "[OK] scaffold smoke passed"
