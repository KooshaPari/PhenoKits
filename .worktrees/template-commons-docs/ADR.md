# ADR — template-commons

## ADR-001: Shared Template Utility Strategy
**Status:** Accepted
**Context:** Multiple language and project templates share common scaffolding logic (file copying, variable substitution, post-scaffold hooks). Duplicating this across each template creates maintenance drift.
**Decision:** Extract shared scaffolding utilities (template rendering, variable substitution, hook runner) into `template-commons` as a standalone library consumed by all other template packages.
**Rationale:** DRY principle at the template layer; bug fixes in scaffolding logic apply to all templates at once.

## ADR-002: Template Variable Format
**Status:** Accepted
**Context:** Need a consistent placeholder format that works across all file types (YAML, TOML, Rust, Go, TS, etc.).
**Decision:** Use `{{VARIABLE_NAME}}` (double-brace Mustache-style) for all template variables. No conditional logic in template files; logic lives in the scaffolding scripts.
**Rationale:** Universally parseable; not confused with language-specific syntax; tooling support in editors.

## ADR-003: Post-Scaffold Hook Protocol
**Status:** Accepted
**Context:** Templates often need post-generation steps (git init, install deps, format).
**Decision:** Define a `hooks/` directory in each template with named hook scripts: `post-scaffold.sh`, `post-install.sh`. `template-commons` provides the hook runner that executes hooks in order.
**Rationale:** Standardized hook names allow the scaffold runner to be generic; per-template customization lives in hook scripts.

## ADR-004: Distribution Format
**Status:** Accepted
**Context:** Templates must be distributable and version-pinnable.
**Decision:** Each template is a standalone package (npm for TS, Python package for Python templates, etc.) with `template-commons` as a dependency. Templates can also be used as git submodules.
**Rationale:** Package-based distribution allows pinning exact template versions; submodule support for low-dependency scenarios.
