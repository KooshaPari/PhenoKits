# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take the security of **Authvault** seriously. If you discover a security vulnerability, please do NOT open a public issue. Instead, report it privately.

Authvault handles authentication and authorization - security is paramount.

### What to include

- A detailed description of the vulnerability
- Steps to reproduce (proof of concept)
- Potential impact (especially for auth bypass scenarios)
- Any suggested fixes or mitigations

We will acknowledge your report within 24 hours and provide a timeline for resolution.

## Security Features

Authvault implements multiple security measures:

- **Password Hashing**: Argon2 and Bcrypt with configurable work factors
- **Token Security**: RS256/HS256 JWT with short expiry and refresh rotation
- **Rate Limiting**: Built-in protection against brute force attacks
- **Audit Logging**: Complete event trail for compliance
- **Tenant Isolation**: Multi-tenancy with domain-based routing

## Dependency Scanning

Authvault regularly scans dependencies for known vulnerabilities using:

- `cargo audit` in CI/CD
- Security advisories from RustSec
- Automated dependency updates

---

Thank you for helping keep the community secure!
