# ADR: model — TypeScript Data Modeling Library

## ADR-001: Standalone Implementation over Zod Wrapper

**Status**: Accepted

**Context**: Zod is the dominant TypeScript schema library. Should `model` wrap Zod or implement its own schema engine?

**Decision**: Standalone implementation with Zod-compatible API surface where practical.

**Rationale**: Wrapping Zod would lock in Zod version and bundle all of Zod. A standalone implementation allows Phenotype-specific ergonomics (TOML support, Phenotype error format) and a smaller bundle. API compatibility with Zod lowers migration friction.

**Consequences**: Higher initial implementation cost. Must track Zod API changes manually.

---

## ADR-002: Result Type over Exception-Only API

**Status**: Accepted

**Context**: Validation APIs can throw exceptions or return Result types. Exceptions are idiomatic in Zod (`.parse()`); Result types are safer for non-throwing paths.

**Decision**: Provide both `.parse()` (throws) and `.safeParse()` (returns Result). Default documentation shows `.safeParse()`.

**Rationale**: Library consumers in async/server contexts prefer Result types. Throwing API retained for ergonomics in non-critical contexts (e.g., startup config parsing).

**Consequences**: Dual API increases surface area but follows established Zod/Valibot conventions that users expect.

---

## ADR-003: Optional Serialization Adapters

**Status**: Accepted

**Context**: YAML and TOML serialization require additional dependencies (js-yaml, @iarna/toml). Including them in the core bundle increases size.

**Decision**: YAML and TOML serializers are exported from optional subpaths (`@model/core/yaml`, `@model/core/toml`) with explicit peer dependency requirements.

**Rationale**: Core bundle stays < 10KB. Users who need YAML/TOML opt in explicitly.

**Consequences**: Import paths differ from core. Documentation must clearly show optional import paths.
