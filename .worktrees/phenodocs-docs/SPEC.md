# Phenotype Docs Specification

## Repository Overview

Phenodocs provides documentation infrastructure.

## Architecture

```
phenodocs/
├── docs/                 # Documentation source
├── src/                  # Documentation tooling
├── tests/                # Doc tests
└── README.md
```

## xDD Practices

### Documentation as Code (Docs-as-Code)

| Practice | Status |
|----------|--------|
| Doc tests | 🔴 Not enforced |
| Documentation in source | 🟡 README only |
| ADRs | 🟡 Minimal |
| Runbooks | 🔴 None |
| Architecture Decision Records | 🔴 Inline comments |

## Quality Gates

```bash
mdtest --lint               # Markdown lint
 vale lint docs/             # Prose linting
 cargo test --doc            # Doc tests
锈 spellcheck docs/          # Spell check
```

## File Structure

```
phenodocs/
├── docs/
│   ├── reference/          # API reference
│   ├── guides/            # How-tos
│   ├── concepts/          # Concepts
│   └── adrs/             # Architecture decisions
├── src/
│   ├── lint.rs            # Markdown linting
│   ├── build.rs           # Doc build
│   └── test.rs            # Doc tests
└── tests/
    ├── integration/
    └── validation/
```

## Documentation Types

| Type | Audience | Format |
|------|----------|---------|
| Reference | Developers | Rustdoc |
| Guides | Users | Markdown |
| Concepts | Architects | Markdown |
| ADRs | Teams | MADR format |
| Runbooks | Ops | Markdown + Frontmatter |

## Domain Model

### Documentation Entities

```rust
pub struct DocPage {
    pub slug: Slug,
    pub title: Title,
    pub content: Markdown,
    pub frontmatter: Frontmatter,
}

pub struct ADR {
    pub id: ADRId,
    pub title: Title,
    pub status: ADRStatus,
    pub context: Context,
    pub decision: Decision,
}
```

## xDD Methodologies

### Documentation-Driven Development (DDD for docs)

```rust
// Document first, implement second
/// # Test Doubles
/// Tests require mock documentation...
#[cfg(test)]
mod tests { }
```

### Example Mapping

| Code | Documentation |
|-------|----------------|
| `struct Agent` | `agent.md` |
| `fn execute()` | `execution.md` |
| `Error` variants | `errors.md` |
| `trait Tool` | `tools/index.md` |

## Quality Checklist

- [ ] Enforce doc tests
- [ ] Add architecture tests (arch_test crate)
- [ ] Integrate vale prose linting
- [ ] Add Markdown property tests
- [ ] Contract tests for doc generation
- [ ] Mutation testing for doc rendering

## References

- [ ] [docsmith](https://github.com/portage/docsmith) patterns
- [ ] [rust-lang/api-guidelines](https://rust-lang.github.io/api-guidelines/)
- [ ] [rust-lang/rfcs](https://github.com/rust-lang/rfcs)
- [ ] [markdownlint](https://github.com/DavidAnson/markdownlint)
- [ ] [vale](https://github.com/errata-ai/vale)
- [ ] [cspell](https://github.com/streetsidesoftware/cspell)