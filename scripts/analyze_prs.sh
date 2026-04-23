#!/bin/bash
# PR Merge Analysis Script
# Usage: ./analyze_prs.sh [repo]

REPO="${1:-heliosApp}"

echo "=========================================="
echo "PR Merge Analysis for: $REPO"
echo "=========================================="
echo ""

cd "/Users/kooshapari/CodeProjects/Phenotype/repos/$REPO" 2>/dev/null || {
  echo "Error: Could not find repo $REPO"
  exit 1
}

echo "Fetching open PRs..."
echo ""

# Get open PRs and their merge status
gh pr list --state open --json number,title,mergeStateStatus,mergeable,headRefName --limit 20 | jq -r '.[] | "\(.number)|\(.title)|\(.mergeStateStatus)|\(.mergeable)|\(.headRefName)"' | while IFS='|' read -r num title status mergeable branch; do
  echo "--- PR #$num: $title ---"
  echo "  Branch: $branch"
  echo "  Status: $status"
  echo "  Mergeable: $mergeable"
  
  if [ "$status" = "BLOCKED" ]; then
    echo "  Action: BLOCKED - Requires status checks or review"
  elif [ "$status" = "CLEAN" ]; then
    echo "  Action: READY - Can be merged immediately"
  elif [ "$status" = "BEHIND" ]; then
    echo "  Action: NEEDS UPDATE - Branch is behind main"
  else
    echo "  Action: CHECK - Unknown status: $status"
  fi
  echo ""
done

echo "=========================================="
echo "Summary Commands:"
echo "=========================================="
echo "Merge a specific PR: gh pr merge <number> --admin --squash"
echo "View PR details: gh pr view <number>"
echo "Check PR checks: gh pr checks <number>"
echo ""
echo "If API merges fail due to branch protection:"
echo "1. Use GitHub web UI: https://github.com/KooshaPari/$REPO/pulls"
echo "2. Or temporarily disable branch protection in Settings > Branches"
