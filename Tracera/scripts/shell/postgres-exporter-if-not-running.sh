#!/usr/bin/env bash
# Start postgres_exporter only if port 9187 is not in use (avoids "address already in use"
# when exporter was started by a previous process-compose run or another process).

set -e
POSTGRES_EXPORTER_PORT="${POSTGRES_EXPORTER_PORT:-9187}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

if ! command -v postgres_exporter >/dev/null 2>&1; then
  echo "postgres_exporter not found; skipping optional Postgres exporter."
  exec tail -f /dev/null
fi

bash "$ROOT/scripts/shell/guard-port.sh" "postgres_exporter" "$POSTGRES_EXPORTER_PORT" "postgres_exporter"

exec postgres_exporter --no-collector.wal
