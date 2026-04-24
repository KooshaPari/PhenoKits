# Secrets Scan Workflow Deployment Playbook

**Objective:** Deploy `secrets-scan.yml` to all 30 active repos  
**Effort:** ~15 min (automated via `gh` CLI)  
**Status:** Ready to deploy

---

## Quick Deploy (All Repos)

### Option A: Automated Bash Loop (Recommended)

```bash
#!/bin/bash
# Deploy secrets-scan.yml to all repos in /Users/kooshapari/CodeProjects/Phenotype/repos/

REPOS_DIR="/Users/kooshapari/CodeProjects/Phenotype/repos"
WORKFLOW_FILE="docs/security/secrets-scan-workflow.yml"
DEST=".github/workflows/secrets-scan.yml"

cd "$REPOS_DIR"

for dir in */; do
  repo="${dir%/}"
  [ -d "$repo/.git" ] || continue
  
  echo "Deploying to $repo..."
  cp "$WORKFLOW_FILE" "$repo/$DEST"
  
  cd "$repo"
  git add "$DEST"
  git commit -m "chore(security): add trufflehog secrets-scan workflow" --allow-empty 2>/dev/null || true
  git push origin main 2>/dev/null &
  cd ..
done

wait
echo "Deployment complete. Workflows will activate on next PR + nightly schedule."
```

### Option B: Per-Repo Manual Deploy

For a single repo:

```bash
repo="AgilePlus"
cd /Users/kooshapari/CodeProjects/Phenotype/repos/$repo
mkdir -p .github/workflows
cp ../docs/security/secrets-scan-workflow.yml .github/workflows/secrets-scan.yml
git add .github/workflows/secrets-scan.yml
git commit -m "chore(security): add trufflehog secrets-scan workflow"
git push origin main
```

---

## Verification

### 1. Check Deployment

```bash
# Verify workflow file exists
find /Users/kooshapari/CodeProjects/Phenotype/repos \
  -path "*/.github/workflows/secrets-scan.yml" | wc -l
# Expected: 30 (all repos deployed)
```

### 2. Trigger Initial Run

Push a dummy commit to trigger the PR workflow:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/<repo>
git checkout -b test/secrets-scan-verify
echo "# Secrets scan verification" >> README.md
git add README.md
git commit -m "test: trigger secrets-scan workflow"
git push origin test/secrets-scan-verify
# Create PR, observe workflow run
```

### 3. Monitor First Nightly Run

- First nightly run: next 2 AM UTC
- Check GitHub Actions tab in each repo
- Confirm: workflow runs, exits cleanly (continue-on-error = true)

---

## Customization Per Repo

### Skip Scan (Optional)

If a repo intentionally stores test fixtures with credential-like patterns:

1. Create `.trufflehogignore` in repo root:
```
# .trufflehogignore
tests/fixtures/mock-config.json
docs/examples/sample-secrets.yaml
```

2. Update workflow to reference it:
```yaml
extra_args: --only-verified --fail
```

### Enforce Scan (Strict Mode)

For critical repos (AgilePlus, phenotype-infrakit):

```yaml
continue-on-error: false  # Fail PRs if secrets detected
```

---

## Troubleshooting

### Issue: Workflow Not Running

**Solution:** Check GitHub Actions enabled:
```bash
gh repo view <owner>/<repo> --json isPrivate
gh api repos/<owner>/<repo>/actions/permissions
```

### Issue: False Positives in Logs

**Solution:** Whitelist patterns in `.trufflehogignore`:
```
# Whitelist base64-encoded placeholder
aGVsbG8gd29ybGQ=
```

### Issue: Slow Scan (>60s)

**Solution:** Reduce history depth in workflow:
```yaml
base: ${{ github.event.repository.default_branch }}
head: HEAD  # Compare only against HEAD, not full history
```

---

## Monitoring & Audit

### Enable Logging

Add to your GitHub Actions runner environment:

```bash
export DEBUG=trufflehog*
```

### Periodic Review

Schedule a recurring audit (monthly):

```bash
# Full scan against last 100 commits
cd /Users/kooshapari/CodeProjects/Phenotype/repos/<repo>
trufflehog git file://. --since-commit HEAD~100 --only-verified --fail
```

---

## Rollback

If you need to remove the workflow:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/<repo>
git rm .github/workflows/secrets-scan.yml
git commit -m "chore(security): remove secrets-scan workflow"
git push origin main
```

---

## Post-Deployment Checklist

- [ ] Workflow files deployed to all 30 repos
- [ ] Initial test run triggered and passing
- [ ] First nightly run confirmed (2 AM UTC)
- [ ] No false-positive alerts in logs
- [ ] Team notified of new workflow
- [ ] Documentation updated in `docs/security/`

---

**Estimated Timeline:**  
- **Immediate:** Deploy workflow files (~5 min)
- **Day 1:** Monitor first nightly run (~1 min)
- **Ongoing:** Review alerts as they arrive

---

*Last Updated: 2026-04-24*
