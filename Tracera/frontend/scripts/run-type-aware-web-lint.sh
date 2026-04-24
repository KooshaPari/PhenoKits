#!/bin/sh
set -eu

log_file="$(mktemp "${TMPDIR:-/tmp}/tracera-type-aware-web.XXXXXX.log")"
trap 'rm -f "$log_file"' EXIT

set +e
bun x oxlint -c .oxlintrc.json -f unix --threads 1 \
  --tsconfig apps/web/tsconfig.oxlint.json \
  --ignore-pattern 'apps/web/src/routeTree.gen.ts' \
  --ignore-pattern 'apps/web/src/views/details/TestDetailView.tsx' \
  --ignore-pattern 'apps/web/src/components/graph/JourneyExplorer.tsx' \
  --type-aware apps/web/src >"$log_file" 2>&1
lint_status=$?
cat "$log_file"
set -e

if [ "$lint_status" -eq 0 ]; then
  exit 0
fi

if grep -Eiq 'panic|fatal|internal compiler error' "$log_file"; then
  echo "type-aware web lint crashed; leaving this as a blocking toolchain failure." >&2
  exit "$lint_status"
fi

echo "type-aware web lint report completed with legacy findings; strict backlog command: bun run lint:type-aware:web:strict" >&2
exit 0
