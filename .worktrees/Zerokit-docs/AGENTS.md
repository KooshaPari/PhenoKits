# Zerokit — AGENTS.md

## Project Overview

Cryptography utilities with zero-knowledge proofs and encryption.

## Agent Rules

1. **Read CLAUDE.md first** before making changes
2. **Test first** - write tests before implementation
3. **Clippy clean** - all lints must pass before PR
4. **No unsafe code** - this crate is `#[forbid(unsafe_code)]`

## Quality Gates

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Security Rules

- **No hand-rolled crypto** - use audited crates only
- **Constant-time comparisons** - for all secret data
- **Zeroize on drop** - keys must be zeroized
- **Opaque errors** - no timing leaks via error messages

## Testing

All cryptographic implementations must have:
1. Unit tests
2. Known-answer tests (KATs) against test vectors
3. Fuzz tests for parsers

## See Also

- **CLAUDE.md**: `./CLAUDE.md`
- **PRD.md**: `./PRD.md`
