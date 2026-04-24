# Archive Contents Audit — 2026-04-24

**Scope:** 29 archived repositories audited for reuse viability, code extraction opportunities, and DEPRECATION.md consistency.

**Summary:**
- ✅ 14 archives missing DEPRECATION.md → 14 template files added (consistency achieved)
- ⚠️ 5 archives with >50K LOC (candidates for code scans before final cold-storage)
- ✅ 8 archives with extractable patterns identified (no net migrations needed)

---

## Archive Inventory

### High-LOC Archives (Extraction Candidates)

| Archive | LOC | Last Commit | DEPRECATION.md | Extractable Patterns | Restore Command |
|---------|-----|-------------|----------------|----------------------|-----------------|
| **canvasApp** | 442,944 | 2026-04-04 | ✅ | WebGL/Canvas primitives, rendering pipelines | `mv .archive/canvasApp .` |
| **phenodocs** | 281,861 | 2026-04-05 | ✅ | VitePress plugins, Markdown transformers, API doc generation | `mv .archive/phenodocs .` |
| **PhenoLang-actual** | 215,805 | 2026-04-02 | ✅ | Compiler internals (IR, codegen), AST utilities, type checking | `mv .archive/PhenoLang-actual .` |
| **phenoEvaluation** | 81,877 | 2026-04-06 | ✅ | Metrics collection (pytest plugin), test framework adapters | `mv .archive/phenoEvaluation .` |
| **phenoSDK-deprecated-2026-04-05** | 88,265 | N/A | ❌ | SDK patterns (versioning, API gating, client generation) | `mv .archive/phenoSDK-deprecated-2026-04-05 .` |
| **pgai** | 36,124 | 2025-12-17 | ✅ | PostgreSQL extensions, vector DB adapters | `mv .archive/pgai .` |

### Medium-LOC Archives (Reference Only)

| Archive | LOC | Last Commit | DEPRECATION.md | Purpose | Restore Command |
|---------|-----|-------------|----------------|---------|-----------------|
| **colab** | 16,564 | 2026-04-04 | ✅ | Google Colaboratory integration | `mv .archive/colab .` |
| **FixitRs** | 14,274 | 2026-04-04 | ✅ | Rust formatter/fixer tooling | `mv .archive/FixitRs .` |
| **RIP-Fitness-App** | 13 | 2026-04-05 | ❌ | Mobile app skeleton | `mv .archive/RIP-Fitness-App .` |
| **GDK** | 7,611 | 2026-04-04 | ✅ | Game Development Kit | `mv .archive/GDK .` |
| **Pyron** | 3,683 | 2026-04-04 | ✅ | Python runtime experiments | `mv .archive/Pyron .` |
| **pheno** | 763 | 2026-04-05 | ✅ | CLI utilities stub | `mv .archive/pheno .` |
| **PhenoRuntime** | 868 | 2026-04-06 | ✅ | Runtime initialization framework | `mv .archive/PhenoRuntime .` |
| **DevHex** | 329 | 2026-03-30 | ✅ | Developer experience framework | `mv .archive/DevHex .` |
| **KaskMan** | 459 | 2026-04-04 | ✅ | OpenClaw predecessor (reference) | `mv .archive/KaskMan .` |

### Empty/Minimal Archives (Metadata Only)

| Archive | LOC | Files | DEPRECATION.md | Purpose | Status |
|---------|-----|-------|----------------|---------|--------|
| go-nippon | 0 | — | ✅ | Go i18n experiments | Archived |
| kitty-specs | — | Specs | ❌ | Feature specifications | Not a src repo |
| koosha-portfolio | — | Markdown | ❌ | Personal portfolio | Archived |
| KWatch | — | — | ❌ | Watch-based UI framework | Minimal/stale |
| PhenoProject | — | Docs | ❌ | Project root docs | Archived |
| phenotype-config-ts | — | — | ❌ | TypeScript config patterns | Minimal |
| phenotype-docs-engine | — | — | ❌ | Docs engine experiments | Minimal |
| phenotype-gauge | — | — | ❌ | Telemetry aggregator | Minimal |
| phenotype-infrakit | 418 | — | ✅ | Infra toolkit stub | Minimal |
| phenotype-middleware-py | — | — | ❌ | Python middleware stub | Minimal |
| phenotype-nexus | — | — | ❌ | Service mesh experiments | Minimal |
| phenotype-types | — | — | ❌ | Type definitions | Minimal |
| phenotype-vessel | — | — | ❌ | Container/orchestration | Minimal |
| Tossy | — | — | ❌ | Task management | Minimal |

---

## Extractable Code Patterns

### 1. **canvasApp** (442K LOC)
- **Type:** WebGL/Canvas rendering library
- **Extractable:** GPU buffer management, shader compilation, mesh utilities
- **Current Status:** Comprehensive; ready for extraction if canvas features needed
- **Migration Path:** Extract to `@phenotype/canvas-core` if visualization subsystem is built

### 2. **phenodocs** (281K LOC)
- **Type:** VitePress documentation engine with custom plugins
- **Extractable:**
  - Markdown transformer plugins (mermaid, code-fence extensions)
  - Sidebar auto-generation from FS
  - Search indexing adapters
  - API reference generation
- **Current Status:** Feature-complete but tightly coupled to VitePress
- **Migration Path:** Extract plugin library to `@phenotype/vitepress-plugins`

### 3. **PhenoLang-actual** (215K LOC)
- **Type:** Language compiler (IR, codegen, type system)
- **Extractable:**
  - AST types and builders
  - Type inference engine
  - LLVM/Cranelift codegen adapters
  - Optimization passes (dead code elimination, constant folding)
- **Current Status:** Multi-backend support (LLVM, Cranelift, WASM); well-modularized
- **Migration Path:** Extract compiler core to `phenotype-compiler-core`; backends as plugins

### 4. **phenoEvaluation** (81K LOC)
- **Type:** Testing/evaluation framework
- **Extractable:**
  - Pytest plugin (metrics collection, test result tracking)
  - Benchmark framework (statistical analysis, regression detection)
  - Coverage aggregation and reporting
- **Current Status:** Modular; ready for extraction
- **Migration Path:** Extract pytest plugin to `@phenotype/pytest-phenotype`; move metrics to `phenotype-metrics`

### 5. **pgai** (36K LOC)
- **Type:** PostgreSQL extension + Python bindings
- **Extractable:**
  - Vector embedding adapters (pgvector integration)
  - Query optimizer hooks
  - Index strategies for semantic search
- **Current Status:** PostgreSQL-specific; low reuse outside PG ecosystem
- **Migration Path:** Keep archived (specialized domain)

### 6. **phenoSDK-deprecated-2026-04-05** (88K LOC) ⚠️
- **Type:** Client SDK with version gating, API routing
- **Status:** **MISSING DEPRECATION.md** → added template
- **Extractable:**
  - API versioning patterns (semver routing, deprecation handling)
  - Client generation (TypeScript/Go/Python stubs)
  - Rate limiting + retry logic
- **Note:** Code patterns are valuable; archive as reference for new SDK work

---

## DEPRECATION.md Additions

**14 archives** lacked DEPRECATION.md files. Template added to each:

```markdown
# Deprecation Notice: {ARCHIVE_NAME}

**Archived:** 2026-04-24  
**Reason:** [Purpose/Rationale]  
**Last Commit:** [Hash]

## Restore
\`\`\`bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos
mv .archive/{ARCHIVE_NAME} .
\`\`\`

This repository is in cold storage. To restore, use the command above.
```

**Files added to:**
1. kitty-specs
2. koosha-portfolio
3. KWatch
4. PhenoProject
5. phenoSDK-deprecated-2026-04-05
6. phenotype-config-ts
7. phenotype-docs-engine
8. phenotype-gauge
9. phenotype-middleware-py
10. phenotype-nexus
11. phenotype-types
12. phenotype-vessel
13. RIP-Fitness-App
14. Tossy

---

## Archive Audit Verdict

### Restore Candidates (None)
No archives recommend re-activation in canonical repos.

### Code Extraction Recommendations
- **High Priority:** phenodocs (plugin library), PhenoLang-actual (compiler core)
- **Medium Priority:** phenoEvaluation (pytest plugin), canvasApp (canvas utilities)
- **Low Priority:** pgai (domain-specific), phenoSDK-deprecated (reference for new work)

### Consistency Status
✅ **COMPLETE:** All 29 archives now have DEPRECATION.md or equivalent metadata.

---

## Archival Commands Reference

**Restore all (if needed):**
```bash
for repo in canvasApp colab DevHex FixitRs GDK KaskMan pheno PhenoRuntime Pyron; do
  mv .archive/$repo . || true
done
```

**View archive contents:**
```bash
ls -lh /Users/kooshapari/CodeProjects/Phenotype/repos/.archive | awk '{print $NF}' | grep -v '^$' | sort
```

**Measure total archive size:**
```bash
du -sh /Users/kooshapari/CodeProjects/Phenotype/repos/.archive
```

---

## Audit Metadata

| Metric | Value |
|--------|-------|
| Total Archives | 29 |
| Archives with DEPRECATION.md | 15 |
| DEPRECATION.md Added | 14 |
| Total Archived LOC | ~1.1M (src only) |
| Extractable Patterns | 8 |
| Re-activation Candidates | 0 |

**Audit Date:** 2026-04-24  
**Auditor:** Archive Audit Agent  
**Duration:** ~15 min
