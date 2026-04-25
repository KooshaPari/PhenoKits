#!/usr/bin/env bash
# 5-line shell glue: invoke gh CLI for org repo metadata, persist as JSON.
# Justified shell: gh is the canonical interface; a Rust port would re-implement gh's auth+pagination.
# Migrate to Rust (octocrab) when this file grows beyond ~10 lines or needs caching/diffing.
set -euo pipefail
OUT="${1:-data/repos.json}"
mkdir -p "$(dirname "$OUT")"
gh repo list KooshaPari --limit 200 \
  --json name,description,url,homepageUrl,isArchived,repositoryTopics,pushedAt,primaryLanguage,stargazerCount \
  > "$OUT"
echo "wrote $(jq 'length' "$OUT") repos to $OUT"
