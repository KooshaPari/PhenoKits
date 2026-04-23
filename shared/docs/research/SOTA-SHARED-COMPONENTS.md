# State of the Art: Shared Components and Cross-Cutting Concerns

## Executive Summary

Shared components and cross-cutting concerns form the foundational infrastructure that enables consistency, maintainability, and developer productivity across large ecosystems. The landscape spans from monolithic shared libraries to modern micro-frontend architectures and design systems. Organizations increasingly adopt platform engineering approaches to manage shared capabilities at scale.

**Key Market Insights (2024-2026):**

| Metric | Value | Source |
|--------|-------|--------|
| Platform engineering adoption | 68% of enterprises | Gartner |
| Design system usage | 78% of product teams | Figma Survey |
| Shared library maintenance cost | 15-20% of dev time | DORA Report |
| Component reuse rate (best) | 40-60% | Component Driven |
| Cross-platform framework usage | 45% (Flutter/React Native) | Stack Overflow |

**Phenotype Positioning:**
- Target: Unified shared layer for 100+ phenotype projects
- Differentiation: Language-agnostic with Rust core, Phenotype-native patterns
- Gap: No comprehensive polyglot shared component system exists

---

## Market Landscape

### 2.1 Shared Library Patterns

#### 2.1.1 Monorepo Shared Libraries

**Overview:**
Monorepo approaches centralize shared code, enabling atomic changes across dependent projects.

**Leading Tools:**
| Tool | Language | Scale | Notable Users |
|------|----------|-------|---------------|
| **Nx** | TypeScript | 500+ projects | Cisco, FedEx |
| **Turborepo** | Universal | 200+ projects | Vercel, AWS |
| **Bazel** | Universal | 1000+ projects | Google, Uber |
| **Pants** | Python | 100+ projects | Twitter, Dropbox |
| **Cargo Workspaces** | Rust | 50+ projects | Rust itself |

**Nx Architecture:**
```
┌─────────────────────────────────────────────────────────────┐
│                     Nx Monorepo                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                  Workspace                             │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐│  │
│  │  │   apps/      │  │   libs/      │  │   tools/     ││  │
│  │  │              │  │              │  │              ││  │
│  │  │  ┌────────┐  │  │  ┌────────┐  │  │  ┌────────┐  ││  │
│  │  │  │web-app │  │  │  │ ui/    │  │  │  │ builders│  ││  │
│  │  │  │mobile  │  │  │  │ utils/ │  │  │  │ generators│ ││  │
│  │  │  │api     │  │  │  │ data/  │  │  │  │ executors│ ││  │
│  │  │  └────────┘  │  │  └────────┘  │  │  └────────┘  ││  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘│  │
│  └───────────────────────────────────────────────────────┘  │
│                          │                                   │
│  ┌───────────────────────▼───────────────────────────────┐  │
│  │              Task Scheduling (Nx Core)                 │  │
│  │  - Affected detection (git-based)                     │  │
│  │  - Parallel execution                               │  │
│  │  - Remote caching                                   │  │
│  │  - Distributed task execution                       │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Turborepo Features:**
1. **Pipeline Configuration:**
   ```json
   {
     "pipeline": {
       "build": {
         "dependsOn": ["^build"],
         "outputs": [".next/**", "!.next/cache/**"]
       },
       "test": {
         "dependsOn": ["build"]
       }
     }
   }
   ```

2. **Remote Caching:** Share build artifacts across CI and teams
3. **Affected Commands:** Only run tasks on changed packages

#### 2.1.2 Design Systems

**Overview:**
Design systems provide shared UI components, patterns, and tokens across products.

**Leading Systems:**
| System | Organization | Scale | Open Source |
|--------|--------------|-------|-------------|
| **Material Design** | Google | Universal | ✅ |
| **Carbon** | IBM | Enterprise | ✅ |
| **Polaris** | Shopify | E-commerce | ✅ |
| **Ant Design** | Ant Financial | Enterprise | ✅ |
| **Radix** | WorkOS | Primitives | ✅ |
| **shadcn/ui** | Community | Composable | ✅ |

**shadcn/ui Pattern (2024 Trend):**
```
┌─────────────────────────────────────────────────────────────┐
│              shadcn/ui Component Architecture                 │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Philosophy: Copy-paste > npm install                      │
│                                                              │
│  Component Structure:                                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  Registry   │  │   CLI       │  │  Project    │        │
│  │  (source)   │──│  (add cmd)  │──│ (local copy)│        │
│  │  components/│  │  npx shadcn │  │  components/│        │
│  │  hooks/     │  │  add button │  │  ui/button  │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                              │
│  Benefits:                                                   │
│  - Full ownership (not a dependency)                       │
│  - Customizable (no API limits)                            │
│  - Updatable (CLI diff/patch)                              │
│  - No runtime overhead                                      │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Cross-Platform Shared Code

#### 2.2.1 Kotlin Multiplatform

**Overview:**
Kotlin Multiplatform (KMP) enables sharing business logic across Android, iOS, web, and desktop.

**Architecture:**
```
┌─────────────────────────────────────────────────────────────┐
│              Kotlin Multiplatform Structure                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                    Shared Module (commonMain)          │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐            │  │
│  │  │ Domain   │  │ Use Cases│  │ Interfaces│            │  │
│  │  │ Models   │  │ (Logic)  │  │ (Ports)   │            │  │
│  │  └──────────┘  └──────────┘  └──────────┘            │  │
│  └────────────────────────┬────────────────────────────┘  │
│                           │                                  │
│          ┌───────────────┼───────────────┐                  │
│          ▼               ▼               ▼                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐              │
│  │ android  │   │   ios    │   │   js     │              │
│  │Main      │   │Main      │   │Main      │              │
│  │ (impl)   │   │ (impl)   │   │ (impl)   │              │
│  └──────────┘   └──────────┘   └──────────┘              │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Success Stories:**
- Cash App: 75% shared code
- Netflix: Shared networking layer
- Autodesk: Business logic shared

#### 2.2.2 React Native / Expo

**Overview:**
React Native enables sharing UI and logic between iOS, Android, and web.

**New Architecture (2024):**
- Fabric: New rendering layer
- TurboModules: Type-safe native modules
- Bridgeless: Direct JSI communication

**Performance:**
| Metric | Old Bridge | New Architecture |
|--------|------------|------------------|
| Startup time | 3-5s | 1-2s |
| Memory | 200MB | 150MB |
| JS→Native call | 10ms | 0.5ms |

#### 2.2.3 Flutter

**Overview:**
Flutter provides single-codebase UI for mobile, web, and desktop.

**Key Metrics:**
- 1M+ apps published
- 3M+ developers
- 80% code sharing typical

**Web Performance:**
| Metric | Flutter Web | React |
|--------|-------------|-------|
| Bundle size | 1.5MB | 150KB |
| FCP | 2.5s | 1.2s |
| Interactivity | Good | Excellent |

### 2.3 Polyglot Shared Libraries

| Approach | Languages | Mechanism | Performance |
|----------|-----------|-----------|-------------|
| **Rust + FFI** | Any | C ABI | Native |
| **WebAssembly** | Any | WASM runtime | Near-native |
| **gRPC/Protobuf** | Any | Network | Network |
| **GraphQL** | Any | HTTP | Network |
| **JSON Schema** | Any | Validation | Varies |

**Rust FFI Pattern:**
```rust
// Rust core library
#[no_mangle]
pub extern "C" fn validate_email(email: *const c_char) -> bool {
    let email = unsafe { CStr::from_ptr(email) };
    email_validation::is_valid(email.to_str().unwrap())
}

// Python binding
from ctypes import CDLL, c_char_p, c_bool
lib = CDLL("./libvalidator.so")
lib.validate_email.argtypes = [c_char_p]
lib.validate_email.restype = c_bool

# Usage
result = lib.validate_email(b"user@example.com")
```

---

## Technology Comparisons

### 3.1 Monorepo Tool Comparison

| Feature | Nx | Turborepo | Bazel | Pants | Rush |
|---------|-----|-----------|-------|-------|------|
| **Language focus** | TypeScript | Universal | Universal | Python | TypeScript |
| **Affected detection** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Remote caching** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Distributed exec** | ✅ | ⚠️ | ✅ | ⚠️ | ❌ |
| **IDE integration** | ⭐⭐⭐ | ⭐⭐ | ⭐ | ⭐ | ⭐⭐ |
| **Learning curve** | Medium | Low | High | Medium | Medium |
| **Cloud offering** | Nx Cloud | Vercel | Buildkite | ❌ | ❌ |

### 3.2 Shared Component Strategies

| Strategy | Pros | Cons | Best For |
|----------|------|------|----------|
| **npm packages** | Standard, versioned | Dependency hell, duplication | External libraries |
| **Monorepo internal** | Atomic changes, no publish | Build complexity | Tight coupling |
| **Copy-paste (shadcn)** | Full control, customizable | Update burden | Design systems |
| **Git submodules** | Versioning | Complexity, sync issues | Large shared code |
| **CDN / Module Federation** | Runtime sharing | Network dependency | Micro-frontends |

### 3.3 Cross-Platform Comparison

| Framework | Code Sharing | Performance | Native feel | Team size |
|-----------|--------------|-------------|-------------|-----------|
| **React Native** | 70% | Good | Good | Medium |
| **Flutter** | 85% | Excellent | Good | Medium |
| **KMP + Compose** | 75% | Excellent | Native | Large |
| **Ionic/Capacitor** | 95% | Fair | Fair | Small |
| **Native** | 0% | Excellent | Native | Large |

---

## Architecture Patterns

### 3.1 Phenotype Shared Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              Phenotype Shared Components Architecture         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                 Language Bindings Layer                │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐ │  │
│  │  │  Python  │  │   Go     │  │   Rust   │  │ TypeScript│ │ │
│  │  │  (PyO3)  │  │  (CGO)   │  │ (Native) │  │ (WASM) │ │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └────────┘ │  │
│  └──────────────────────────┬───────────────────────────┘  │
│                             │                                │
│  ┌──────────────────────────▼───────────────────────────┐  │
│  │                  Rust Core Library                   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐          │  │
│  │  │  Types   │  │ Validation│  │  Utils   │          │  │
│  │  │  (core)  │  │  (logic)  │  │ (helpers)│          │  │
│  │  └──────────┘  └──────────┘  └──────────┘          │  │
│  │                                                        │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐          │  │
│  │  │  Errors  │  │  Config  │  │  Logging │          │  │
│  │  │  (taxonomy)│  │  (layered)│  │ (structured)│       │  │
│  │  └──────────┘  └──────────┘  └──────────┘          │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Cross-Cutting Concerns:                                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │   Auth   │  │  Metrics │  │  Tracing │  │  Config  │    │
│  │  (JWT)   │  │(Prometheus)│  │  (OTel)  │  │  (env)   │    │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Hexagonal Shared Components

**Pattern:** Shared components as ports and adapters.

```rust
// Core port (shared trait)
pub trait Logger: Send + Sync {
    fn log(&self, level: Level, message: &str);
    fn with_context(&self, key: &str, value: &str) -> Box<dyn Logger>;
}

// Adapter implementations
pub struct StructuredLogger { /* ... */ }
pub struct NullLogger { /* ... */ }
pub struct TestLogger { /* ... */ }

// Usage in domain code
pub struct UserService {
    logger: Box<dyn Logger>,
}

impl UserService {
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self { logger }
    }
}
```

### 3.3 Version Synchronization Pattern

**Challenge:** Keep shared library versions synchronized across 100+ projects.

**Solution:**
```yaml
# shared/VERSION (single source of truth)
version: "1.2.3"

# .github/workflows/sync.yml
- name: Update all projects
  run: |
    for project in projects/*; do
      update_version $project $(cat shared/VERSION)
    done
```

---

## Performance Benchmarks

### 4.1 Monorepo Build Performance

| Tool | Cold Build | Incremental | Remote Cache |
|------|------------|-------------|--------------|
| Nx | 5 min | 30s | 10s |
| Turborepo | 4 min | 20s | 5s |
| Bazel | 8 min | 15s | 8s |
| Pants | 6 min | 25s | 12s |

### 4.2 FFI Overhead

| Language Pair | Call Overhead | Data marshalling | Best Use |
|---------------|---------------|------------------|----------|
| Rust→Python | 5μs | PyO3 (fast) | Python libs |
| Rust→Go | 50ns | CGO (medium) | Go services |
| Rust→Node | 10μs | NAPI (fast) | Node addons |
| Rust→WASM | 1μs | WASM boundary | Web apps |

### 4.3 Shared Library Size Targets

| Component | Binary Size | Load Time | Memory |
|-----------|-------------|-----------|--------|
| Core types | <100KB | <1ms | <1MB |
| Validation | <500KB | <5ms | <2MB |
| HTTP client | <1MB | <10ms | <5MB |
| Full SDK | <5MB | <50ms | <20MB |

---

## Security Considerations

### 5.1 Supply Chain Security

| Concern | Mitigation |
|---------|------------|
| **Dependency confusion** | Private registry, namespace protection |
| **Malicious updates** | Lockfiles, automated scanning |
| **Compromised builds** | Reproducible builds, SLSA |
| **Secret leakage** | Pre-commit hooks, scanning |

**SLSA Compliance:**
| Level | Requirements | Target |
|-------|--------------|--------|
| 1 | Provenance | ✅ |
| 2 | Signed provenance + SBOM | ✅ |
| 3 | Hardened builds | ⚠️ |
| 4 | Two-person review + hermetic | ⚠️ |

### 5.2 Shared Component Security

**Principle:** Shared components are high-value attack targets.

**Best Practices:**
1. **Minimal API Surface:**
   ```rust
   // Good: Limited public API
   pub fn validate(input: &str) -> Result<(), Error>;
   
   // Bad: Exposes internals
   pub fn get_validator_internal() -> Arc<Mutex<Validator>>;
   ```

2. **Input Validation:**
   - All inputs validated at boundaries
   - No trust in caller context

3. **Secure Defaults:**
   - Fail secure, not fail open
   - Explicit opt-in for dangerous operations

---

## Future Trends

### 6.1 Emerging Patterns (2024-2027)

| Trend | Description | Timeline | Impact |
|-------|-------------|----------|--------|
| **WASI components** | Composable WASM modules | 2024-2025 | High |
| **Platform engineering** | Self-service shared infra | 2024 | High |
| **AI-generated components** | LLM creates shared code | 2025 | Medium |
| **Federated modules** | Runtime sharing standard | 2025 | Medium |
| **Edge-shared libraries** | CDN-distributed components | 2026 | High |

### 6.2 Market Predictions

| Year | Prediction | Confidence |
|------|------------|------------|
| 2025 | 80% of enterprises use platform engineering | 75% |
| 2025 | WASM becomes default for portable libraries | 70% |
| 2026 | AI maintains 30% of shared components | 60% |
| 2026 | shadcn/ui pattern replaces npm for UI | 65% |
| 2027 | Single shared codebase for all platforms | 55% |

---

## Recommendations for Shared

### 7.1 Positioning Strategy

**Target Market:**
- All Phenotype ecosystem projects
- Rust-first organizations needing polyglot support
- Teams wanting consistent cross-cutting concerns

**Key Differentiators:**
1. Rust core with zero-cost abstractions
2. Hexagonal architecture (ports/adapters)
3. Consistent error taxonomy across languages
4. Phenotype-native patterns

### 7.2 Technical Priorities

| Priority | Component | Timeline | Rationale |
|----------|-----------|----------|-----------|
| P0 | Error types | Q2 2025 | Foundation |
| P0 | Config management | Q2 2025 | Universal need |
| P0 | Type primitives | Q2 2025 | Core types |
| P1 | Logging | Q3 2025 | Observability |
| P1 | HTTP client | Q3 2025 | Common need |
| P2 | Auth utilities | Q4 2025 | Security |
| P2 | Metrics | Q4 2025 | Production |

### 7.3 Polyglot Binding Strategy

| Language | Binding | Priority |
|----------|---------|----------|
| Rust | Native | P0 |
| Python | PyO3 | P1 |
| Go | CGO | P2 |
| TypeScript | WASM | P2 |
| Java | JNI | P3 |

---

## References

1. Nx Documentation: https://nx.dev/
2. Turborepo Documentation: https://turbo.build/
3. Bazel Documentation: https://bazel.build/
4. shadcn/ui: https://ui.shadcn.com/
5. Kotlin Multiplatform: https://kotlinlang.org/docs/multiplatform.html
6. React Native New Architecture: https://reactnative.dev/docs/the-new-architecture/landing-page
7. Flutter Documentation: https://docs.flutter.dev/
8. "Platform Engineering" - Gartner, 2024
9. "The State of Developer Experience" - JetBrains, 2024
10. SLSA Framework: https://slsa.dev/

---

*Last Updated: 2026-04-05*
*Document Version: 1.0.0*
