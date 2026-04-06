#!/bin/sh
set -eu

if ! command -v ggshield >/dev/null 2>&1; then
  echo "error: ggshield is required but was not found in PATH" >&2
  echo "install: pip install ggshield" >&2
  exit 127
fi

scan_target="${1:-.}"

if [ ! -d "$scan_target" ]; then
  echo "error: scan target '$scan_target' is not a directory" >&2
  exit 2
fi

echo "Running ggshield secret scan on $scan_target"
exec ggshield secret scan path "$scan_target" --recursive
