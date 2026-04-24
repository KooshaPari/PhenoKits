# Architecture Decision Records — hexagon-ts

## ADR-001: TypeScript Interfaces for Ports

**Status:** Accepted

**Context:** TypeScript supports interfaces, abstract classes, and type aliases for contract definition. The template must choose one.

**Decision:** Use TypeScript `interface` for all port definitions. Abstract classes are used only when shared implementation across adapters is needed.

**Rationale:** Interfaces are the lightest-weight TypeScript construct for contracts. They are erased at runtime, produce no bundle overhead, and are the idiomatic TypeScript pattern for inversion of control.

**Alternatives Considered:**
- Abstract classes: heavier; carry runtime overhead; require `extends` rather than `implements`.
- Type aliases: cannot be implemented with `implements` — no way to enforce contract at class definition.

**Consequences:** Test doubles implement the port interface; they can be plain objects as long as TypeScript structurally checks them.

---

## ADR-002: vitest for Testing

**Status:** Accepted

**Context:** The TypeScript ecosystem offers Jest, vitest, and Mocha. The template must choose one.

**Decision:** Use `vitest` as the test runner. It is Jest-compatible, ESM-native, and significantly faster.

**Rationale:** `vitest` is the current standard for TypeScript testing in the Phenotype ecosystem. Its ESM support avoids transpilation issues with modern TypeScript.

**Alternatives Considered:**
- Jest: requires `ts-jest` or `babel-jest`; slower; legacy CommonJS assumptions.
- Mocha: minimal but lacks built-in type checking and coverage integration.

**Consequences:** Test files use `import { describe, it, expect } from "vitest"`.

---

## ADR-003: pnpm as Package Manager

**Status:** Accepted

**Context:** npm, yarn, and pnpm are all viable. The template must choose one consistent with the ecosystem.

**Decision:** Use `pnpm` as the package manager. `package.json` `"packageManager"` field specifies the exact version.

**Rationale:** `pnpm` is the Phenotype ecosystem standard. Faster installs, strict dependency isolation, and workspace support make it the default choice.

**Consequences:** `pnpm-lock.yaml` is committed. Developers must install pnpm (`npm i -g pnpm` or via corepack).
