# OpenAI API Key Revocation Runbook

## Critical Issue
An OpenAI API key has been exposed in canvasApp git history since session W-19. The key must be revoked immediately to prevent unauthorized API usage.

**Key Details:**
- **Key ID:** `user-YWoUAJuhYBAGWm9McVkEEYnk` (length: 48 chars)
- **Associated Account:** rohan.agra49@gmail.com
- **Exposure Location:** canvasApp/.git (multiple commits)
- **Status:** PENDING REVOCATION (HIGH PRIORITY)

---

## Revocation Steps (2 minutes)

### Step 1: Access OpenAI Dashboard
Navigate to: https://platform.openai.com/api-keys

### Step 2: Locate the Exposed Key
1. Log into your OpenAI account (ensure you're on rohan.agra49@gmail.com)
2. In the API Keys section, search for or identify the key starting with `user-YWoUA...`
3. Look for the full key ID: `user-YWoUAJuhYBAGWm9McVkEEYnk`

### Step 3: Revoke the Key
1. Click the three-dot menu (⋯) next to the key
2. Select **"Delete"** or **"Revoke"**
3. Confirm the revocation action

### Step 4: Verify Revocation
After revocation, test that the key is no longer active:

```bash
curl https://api.openai.com/v1/models \
  -H "Authorization: Bearer user-YWoUAJuhYBAGWm9McVkEEYnk"
```

**Expected response:** `HTTP 401 Unauthorized` (or similar auth error)

---

## Post-Revocation Actions

1. **Verify no API usage:** Check OpenAI billing/usage dashboard for activity on this key before revocation time
2. **Update local credentials:** Remove the key from any local config files or environment variables
3. **Regenerate if needed:** If this key was actively used, generate a new key and update deployment secrets (GitHub Secrets, .env files, etc.)
4. **Audit git history:** The canvasApp repo should be scrubbed via BFG or git-filter-branch if this was the only sensitive exposure
   - Reference: `/repos/canvasApp/.git` (if cleanup is needed)

---

## Related Documentation
- **GitHub Actions Secrets Setup:** `/repos/docs/security/github-secrets-infrastructure.md`
- **Token Rotation Policy:** `/repos/docs/governance/token-rotation-playbook.md`
- **Billing Constraint:** `/repos/docs/governance/billing-constraint.md`

---

## Timeline
- **Exposed since:** Session W-19 (2026-01-XX)
- **Risk level:** HIGH (open API key in public/shared git history)
- **Action required:** NOW
