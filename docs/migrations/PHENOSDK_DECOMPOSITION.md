# phenoSDK Decomposition Migration Guide

## Overview

**phenoSDK** was a monolithic SDK providing authentication, security, and credential management. It has been intentionally decomposed into three specialized, independent modules within **AuthKit** to improve maintainability, reduce surface area, and enable granular dependency management.

This guide explains the decomposition rationale, provides module mapping, and offers migration paths for downstream consumers.

---

## Decomposition Rationale

### Problem Statement
- **phenoSDK** combined three distinct concerns (auth, security, credentials) in a single monolith
- High coupling made it difficult to adopt individual features without taking full SDK dependency
- Versioning concerns: auth logic evolves faster than credential storage
- Security updates to one module required re-releasing and re-integrating the entire SDK

### Solution
Extract three independent, composable modules:
1. **AuthKit/pheno-auth** — Authentication flows, token management, session handling
2. **AuthKit/pheno-security** — Security primitives, encryption, validation
3. **AuthKit/pheno-credentials** — Credential storage, lifecycle, rotation

Each module is:
- **Independently versioned** (can patch security without touching auth)
- **Composable** (use auth + security together, or security standalone)
- **Testable** (each has isolated test suite)
- **Documentable** (clear contracts and examples per module)

---

## Module Mapping

### Authentication (formerly `phenoSDK.auth`)

**New Location:** `AuthKit/python/pheno-auth/`

| Old Path | New Path | Status |
|----------|----------|--------|
| `phenoSDK.oauth` | `pheno_auth.oauth` | Moved, enhanced |
| `phenoSDK.session` | `pheno_auth.session` | Moved, enhanced |
| `phenoSDK.token` | `pheno_auth.token` | Moved, enhanced |
| `phenoSDK.oidc` | `pheno_auth.oidc` | Moved, enhanced |
| `phenoSDK.mfa` | `pheno_auth.mfa` | New (formerly in security) |

**Key Classes:**
- `pheno_auth.oauth.OAuthClient` — OAuth 2.0 flows (authorization code, PKCE, refresh)
- `pheno_auth.session.SessionManager` — Stateful session handling with TTL
- `pheno_auth.token.TokenFactory` — JWT and opaque token generation
- `pheno_auth.oidc.OIDCClient` — OpenID Connect federation
- `pheno_auth.mfa.MFAValidator` — Multi-factor authentication

**Install:** `pip install pheno-auth`

**Usage:**
```python
from pheno_auth import OAuthClient, SessionManager

client = OAuthClient(
    client_id="...",
    client_secret="...",
    redirect_uri="http://localhost:8000/callback"
)

session_mgr = SessionManager(backend="redis")
session = session_mgr.create(user_id="user_123", ttl=3600)
```

---

### Security (formerly `phenoSDK.security`)

**New Location:** `AuthKit/python/pheno-security/`

| Old Path | New Path | Status |
|----------|----------|--------|
| `phenoSDK.crypto` | `pheno_security.crypto` | Moved, enhanced |
| `phenoSDK.validation` | `pheno_security.validation` | Moved, enhanced |
| `phenoSDK.encryption` | `pheno_security.encryption` | Moved, enhanced |
| `phenoSDK.hashing` | `pheno_security.hashing` | Moved, enhanced |
| `phenoSDK.audit` | `pheno_security.audit` | Extracted |

**Key Classes:**
- `pheno_security.crypto.ECDSA` — Elliptic Curve Digital Signature Algorithm
- `pheno_security.crypto.RSA` — RSA encryption and signing
- `pheno_security.encryption.AES` — AES-256-GCM symmetric encryption
- `pheno_security.hashing.Argon2` — Memory-hard password hashing
- `pheno_security.validation.Validator` — Input validation (email, URL, etc.)
- `pheno_security.audit.AuditLogger` — Compliance logging

**Install:** `pip install pheno-security`

**Usage:**
```python
from pheno_security import AES, Argon2

cipher = AES.create_cipher()
encrypted = cipher.encrypt(b"secret data")

hasher = Argon2(time_cost=2, memory_cost=65536)
hashed = hasher.hash(password="mypassword")
verified = hasher.verify(password="mypassword", hash=hashed)
```

---

### Credentials (formerly `phenoSDK.credentials`)

**New Location:** `AuthKit/python/pheno-credentials/`

| Old Path | New Path | Status |
|----------|----------|--------|
| `phenoSDK.storage` | `pheno_credentials.storage` | Moved, enhanced |
| `phenoSDK.rotation` | `pheno_credentials.rotation` | Moved, enhanced |
| `phenoSDK.lifecycle` | `pheno_credentials.lifecycle` | Moved, enhanced |
| `phenoSDK.provider` | `pheno_credentials.provider` | Moved, enhanced |

**Key Classes:**
- `pheno_credentials.storage.CredentialStore` — In-memory, Redis, or Postgres backing
- `pheno_credentials.storage.VaultAdapter` — HashiCorp Vault integration
- `pheno_credentials.rotation.RotationPolicy` — Automated rotation schedules
- `pheno_credentials.lifecycle.CredentialLifecycle` — Creation, activation, revocation
- `pheno_credentials.provider.CredentialProvider` — Factory for different credential types

**Install:** `pip install pheno-credentials`

**Usage:**
```python
from pheno_credentials import CredentialStore, RotationPolicy

store = CredentialStore(backend="redis")
policy = RotationPolicy(rotation_interval_days=90)

cred = store.create(
    user_id="user_123",
    type="api_key",
    rotation_policy=policy
)
print(cred.value)  # New API key
```

---

## Migration Checklist

### For Consumers Currently Using `phenoSDK`

- [ ] Identify which phenoSDK submodules you import
- [ ] Determine which new modules you need (auth, security, credentials)
- [ ] Install target modules: `pip install pheno-auth pheno-security pheno-credentials`
- [ ] Update imports using the mapping table above
- [ ] Run tests to verify behavior is unchanged
- [ ] Update requirements.txt or pyproject.toml
- [ ] Remove phenoSDK from dependencies
- [ ] Commit: "chore(deps): migrate phenoSDK → {pheno-auth,pheno-security,pheno-credentials}"

### Before/After Examples

#### Example 1: Basic OAuth Flow
**Before (phenoSDK):**
```python
from phenosdk import OAuthClient

client = OAuthClient(config="production")
auth_url = client.get_authorization_url(redirect_uri="http://localhost:8000/cb")
```

**After (pheno-auth):**
```python
from pheno_auth import OAuthClient

client = OAuthClient(
    client_id="...",
    client_secret="...",
    redirect_uri="http://localhost:8000/cb"
)
auth_url = client.get_authorization_url()
```

#### Example 2: Password Hashing
**Before (phenoSDK):**
```python
from phenosdk.security import hash_password

hashed = hash_password("mypassword")
```

**After (pheno-security):**
```python
from pheno_security.hashing import Argon2

hasher = Argon2()
hashed = hasher.hash("mypassword")
```

#### Example 3: API Key Management
**Before (phenoSDK):**
```python
from phenosdk import CredentialManager

mgr = CredentialManager(config="production")
key = mgr.generate_api_key(user_id="user_123")
```

**After (pheno-credentials):**
```python
from pheno_credentials import CredentialStore

store = CredentialStore(backend="postgres")
cred = store.create(user_id="user_123", type="api_key")
```

---

## Deprecation Timeline

### Phase 1: Announcement (Current)
- phenoSDK marked as deprecated in PyPI
- DEPRECATION.md added to .archive/phenoSDK
- Migration guide published (this document)

### Phase 2: Parallel Support (Next 2 quarters)
- phenoSDK remains installable and functional
- Bug fixes applied, but no new features
- All Tier 0 enforcement references updated to new modules

### Phase 3: Sunset (End of Year)
- phenoSDK removed from PyPI
- Users must have migrated to pheno-{auth,security,credentials}
- Archive preserved at `.archive/phenoSDK/` for reference

---

## Tier 0 Migration Status

Phenotype enforces **Tier 0** adoption for all internal projects. The following projects have been updated:

| Project | Auth | Security | Credentials | Status |
|---------|------|----------|-------------|--------|
| AuthKit | ✅ | ✅ | ✅ | Migrated |
| PhenoKit | ✅ | ✅ | ✅ | Migrated |
| AgilePlus | ✅ | ✅ | ❌ | In Progress |
| HeliosApp | ✅ | ✅ | ✅ | Migrated |
| Bifrost | ✅ | ❌ | ✅ | In Progress |

See `tooling/legacy-enforcement/TIER0_MIGRATION_REPORT.md` for detailed status.

---

## Support & Questions

**For questions about:**
- **pheno-auth**: See `AuthKit/python/pheno-auth/README.md` and examples
- **pheno-security**: See `AuthKit/python/pheno-security/README.md` and examples
- **pheno-credentials**: See `AuthKit/python/pheno-credentials/README.md` and examples

**Filing Issues:**
- Use the `AuthKit` GitHub issues with labels: `pheno-auth`, `pheno-security`, or `pheno-credentials`

**Migration Help:**
- Open an issue with `migration` label for specific questions
- Include: old code snippet + current error

---

## References

- **Original Decision:** `kitty-specs/001-phenosdk-decomposition/PLAN.md`
- **AuthKit Module:** `AuthKit/python/`
- **Deprecated Code:** `.archive/phenoSDK/`
- **Enforcement Tracking:** `tooling/legacy-enforcement/TIER0_MIGRATION_REPORT.md`
