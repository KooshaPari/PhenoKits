# ADR-001: Trait-Based Validation Pattern

**Status:** Accepted  
**Date:** 2026-04-05  
**Author:** Phenotype Architecture Team  
**Reviewers:** Phenotype Core Team  
**Supersedes:** N/A  
**Superseded by:** N/A

---

## Context

phenotype-validation is a Rust validation framework that needs to provide a simple, extensible, and type-safe approach to validating domain objects. The primary design decision is choosing between macro-based validation (like the `validator` crate) and trait-based validation (like `pectus`).

### Problem Statement

Existing validation approaches in Rust have significant trade-offs:

1. **Macro-based (validator):**
   - Excellent for simple cases
   - Hard to debug generated code
   - Limited composition
   - Complex error messages

2. **Trait-based (pectus, ruvalid):**
   - Simple and explicit
   - More boilerplate required
   - Better composition
   - Clear control flow

3. **Schema-based (jsonschema):**
   - Runtime validation only
   - No compile-time guarantees
   - External schema required

### Goals

1. **Simplicity:** Easy to understand, minimal concepts
2. **Extensibility:** Users can create custom validators
3. **Performance:** Zero-cost abstractions where possible
4. **Ergonomics:** Clean API that feels idiomatic to Rust

---

## Decision

Adopt a **trait-based validation pattern** with manual implementation.

### Core Trait

```rust
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
    
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}
```

### Design Rationale

#### Why Trait-Based?

1. **Explicit control flow:** Validation logic is readable as regular Rust code
2. **No hidden magic:** No generated code, no macros to debug
3. **Easy to extend:** Just implement the trait
4. **Composition friendly:** Validators are plain functions
5. **IDE support:** Full autocomplete and refactoring support

#### Why NOT Derive Macros?

1. **Debugging difficulty:** Generated code is hard to trace
2. **Error message complexity:** Macro errors can be cryptic
3. **Limited composition:** Hard to combine multiple validation strategies
4. **Hidden behavior:** Magic happens behind the scenes

### Implementation Pattern

Users implement `Validate` for their types:

```rust
use phenotype_validation::{Validate, ValidationErrors, Validator};

#[derive(Debug)]
struct User {
    email: String,
    age: u32,
}

impl Validate for User {
    fn validate(&self) -> Result<(), ValidationErrors> {
        use phenotype_validation::validate_all;
        
        validate_all(vec![
            Validator::email(&self.email),
            Validator::range(self.age, 18, u32::MAX, "age"),
        ])
    }
}
```

### Comparison to Alternatives

| Aspect | Trait-Based | Derive Macros | Schema-Based |
|--------|-------------|---------------|--------------|
| Learning curve | Low | Medium | High |
| Debugging | Easy | Hard | Medium |
| Composition | Excellent | Limited | Good |
| Performance | Excellent | Excellent | Good |
| Type safety | Full | Full | Partial |

---

## Consequences

### Positive

1. **Transparency:** All validation logic is explicit, readable Rust code
2. **Debugging:** Standard Rust debugging techniques apply
3. **Testing:** Easy to test validation logic in isolation
4. **Customization:** Users have full control over validation logic
5. **Error messages:** Clear, predictable error formatting

### Negative

1. **Boilerplate:** More code than derive macro alternatives
2. **Repetition:** Similar validation logic requires repetition
3. **No automatic derive:** Can't automatically implement for structs
4. **Migration effort:** Users switching from validator crate need to rewrite

### Mitigations

1. **Helper functions:** Provide reusable validator functions
2. **Composition utilities:** `validate_all` for combining checks
3. **Documentation:** Clear examples showing common patterns
4. **Future derive:** Consider adding derive macro later

---

## Implementation Details

### Directory Structure

```
src/
├── lib.rs                    # Public API, Validate trait, errors
├── validators/
│   ├── mod.rs               # Validator struct
│   ├── string.rs            # String validators
│   ├── numeric.rs           # Numeric validators
│   └── pattern.rs           # Regex validators
└── combinators/
    ├── mod.rs               # Composition utilities
    └── validate_all.rs      # Aggregate validation
```

### Key Types

```rust
// Core error type
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("Validation error on field '{field}': {message}")]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

// Error collection
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("Validation failed with {} errors", errors.len())]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

// Core trait
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}
```

---

## Alternatives Considered

### Alternative 1: Derive Macro (rejected)

**Approach:** Use `#[derive(Validate)]` to generate validation code.

**Rejection Reason:**
- Hidden generated code
- Complex error messages
- Hard to debug
- Limited composition

### Alternative 2: Schema-Based (rejected)

**Approach:** Define validation schemas separate from code.

**Rejection Reason:**
- No compile-time safety
- Separate schema files
- Runtime overhead
- Less idiomatic Rust

### Alternative 3: Hybrid Trait + Derive (deferred)

**Approach:** Provide both trait-based and derive-based validation.

**Decision:** Defer derive support to future phase. Current trait-based approach is sufficient for MVP.

---

## Validation

### Test Strategy

| Validation Type | Approach | Coverage Target |
|----------------|----------|-----------------|
| Unit tests | Test each validator function | 95%+ |
| Integration tests | Test trait implementations | 90%+ |
| Property tests | Generate random valid/invalid inputs | 70%+ |

### Architecture Compliance

```bash
# Verify no derive macros in core
grep -r "derive.*Validate" src/ && exit 1

# Verify trait is public
grep "pub trait Validate" src/lib.rs
```

---

## Related Decisions

- ADR-002: Error Aggregation Strategy
- ADR-003: Validator Composition Approach

---

## References

1. **validator crate** - github.com/Keats/validator
2. **pectus crate** - github.com/sdroege/pectus
3. **Trait-based validation patterns** - Various Rust forum discussions

---

## Changelog

| Date | Version | Change | Author |
|------|---------|--------|--------|
| 2026-04-05 | 1.0 | Initial decision | Phenotype Team |

---

**END OF ADR**
