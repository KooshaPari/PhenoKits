#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

# ggshield requires GITGUARDIAN_API_KEY; skip gracefully in CI when not configured
if [ -z "${GITGUARDIAN_API_KEY:-}" ]; then
  echo "[security-guard] GITGUARDIAN_API_KEY not set — skipping ggshield secret scan" >&2
else
  if command -v ggshield >/dev/null 2>&1; then
    GGSHIELD=(ggshield)
  elif command -v uvx >/dev/null 2>&1; then
    GGSHIELD=(uvx ggshield)
  elif command -v uv >/dev/null 2>&1; then
    GGSHIELD=(uv tool run ggshield)
  else
    echo "[security-guard] ggshield not installed — skipping secret scan" >&2
    GGSHIELD=()
  fi

  if [ ${#GGSHIELD[@]} -gt 0 ]; then
    echo "[security-guard] Running ggshield secret scan"
    "${GGSHIELD[@]}" secret scan pre-commit
  fi
fi

if command -v codespell >/dev/null 2>&1; then
  changed_files=$(git diff --cached --name-only --diff-filter=ACM || true)
  if [ -z "${changed_files}" ]; then
    changed_files=$(git diff --name-only HEAD~1..HEAD 2>/dev/null || true)
  fi

  if [ -n "${changed_files}" ]; then
    echo "[security-guard] Running optional codespell fast pass"
    echo "${changed_files}" |       grep -E '\.(md|txt|py|ts|tsx|js|go|rs|kt|java|yaml|yml)$' |       xargs -r codespell -q 2 -L "hte,teh" || true
  fi
fi
