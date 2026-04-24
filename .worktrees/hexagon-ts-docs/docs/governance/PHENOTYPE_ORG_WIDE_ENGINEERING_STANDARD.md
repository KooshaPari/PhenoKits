# Phenotype Org–Wide Engineering Standard (Local + Cloud)

**Version:** 1.0.0  
**Status:** Active  
**Scope:** Phenotype-owned repositories, worktrees, and cloud runtimes (including Dinoforge-style deployments).

## Hand-roll rules (explicit)

| Rule | Requirement |
|------|-------------|
| **No silent hand-rolls** | One-offs become scripts + docs or ADR-tracked steps. |
| **Worktree-first** | Feature work under `repos/worktrees/<project>/<topic>/…`. |
| **CI completeness** | Fix all failing PR checks per org policy. |
| **Secrets** | Never commit tokens; use env / secret managers. |
| **Reproducibility** | Documented entrypoints; avoid laptop-only path hacks. |

## Cloud (Dinoforge)

Treat cloud as **adapters**: ports in domain; config schema-validated like local.

## Root layout (normative)

`README`, `AGENTS`/`CLAUDE`, `docs/`, `src` or `cmd`/`internal`, `ci`, `scripts/` — keep repo root thin.

## Architecture

Hexagonal ports, DDD where complex, polyrepo with **versioned** extracted libs, microservices with async integration where possible.

## Related

- [POLYREPO_PACKAGE_NAMING_AND_PRODUCTIZATION.md](./POLYREPO_PACKAGE_NAMING_AND_PRODUCTIZATION.md)
- [PHENOTYPE_PACKAGES_INVENTORY_AND_RENAME_BACKLOG.md](./PHENOTYPE_PACKAGES_INVENTORY_AND_RENAME_BACKLOG.md)
