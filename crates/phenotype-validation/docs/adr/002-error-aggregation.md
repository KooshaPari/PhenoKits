# ADR-002: Error Aggregation Strategy

**Status:** Accepted  
**Date:** 2026-04-05  
**Author:** Phenotype Architecture Team  
**Reviewers:** Phenotype Core Team  
**Supersedes:** N/A  
**Superseded by:** N/A

---

## Context

Validation frameworks must communicate failure details to callers. The error representation strategy impacts API ergonomics, debugging experience, and performance. This ADR defines how phenotype-validation represents and aggregates validation errors.

### Design Requirements

1. **Multiple errors:** Support validating multiple fields, collecting all failures
2. **Field context:** Each error must identify which field failed
3. **Human-readable:** Error messages should be clear and actionable
4. **Machine-parseable:** Errors should have structured data for tooling
5. **Zero allocation:** Single-error case should not heap-allocate

---

## Decision

Use a **two-tier error system** with `ValidationError` for single errors and `ValidationErrors` for collections.

### Error Types

#### ValidationError

```rust
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("Validation error on field '{field}': {message}")]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}
```

**Design Rationale:**
- `thiserror` provides `#[error(...)]` formatting with zero overhead
- Two fields: `field` for context, `message` for human-readable text
- `Clone` enables error re-use in collections
- `PartialEq` enables error comparison in tests

#### ValidationErrors

```rust
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("Validation failed with {} errors", errors.len())]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}
```

**Design Rationale:**
- `Vec<ValidationError>` for multiple failures
- `thiserror` formatting shows error count
- Implements `IntoIterator` for ergonomic iteration
- `Default` enables empty error collection without allocation

### Aggregation API

```rust
pub fn validate_all(
    validations: Vec<Result<(), ValidationError>>
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();
    
    for validation in validations {
        if let Err(e) = validation {
            errors.add(e);
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

---

## Consequences

### Positive

1. **Collection semantics:** `ValidationErrors` collects all errors, not just first
2. **Field identification:** Every error knows its field context
3. **Ergonomic iteration:** `IntoIterator` enables `for error in errors`
4. **Field filtering:** `for_field()` method finds errors by field
5. **Short-circuit possible:** `is_valid()` for quick boolean check

### Negative

1. **Vec allocation:** Multiple errors require heap allocation
2. **Error count:** Cannot distinguish error types from count alone
3. **No error hierarchy:** All errors are flat, no nested structure

### Mitigations

1. **Single-error case:** Return `ValidationError` directly, no Vec allocation
2. **Capacity hint:** `ValidationErrors::with_capacity(n)` for known count
3. **Structured fields:** Field names can use dot notation for hierarchy

---

## Implementation Details

### ValidationError Methods

```rust
impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}
```

### ValidationErrors Methods

```rust
impl ValidationErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
    
    pub fn add(&mut self, error: ValidationError) {
        self.errors.push(error);
    }
    
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    
    pub fn for_field(&self, field: &str) -> Vec<&ValidationError> {
        self.errors.iter().filter(|e| e.field == field).collect()
    }
}
```

### Iterator Implementation

```rust
impl IntoIterator for ValidationErrors {
    type Item = ValidationError;
    type IntoIter = std::vec::IntoIter<ValidationError>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a> IntoIterator for &'a ValidationErrors {
    type Item = &'a ValidationError;
    type IntoIter = std::slice::Iter<'a, ValidationError>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}
```

---

## Usage Examples

### Collect All Errors

```rust
impl Validate for User {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let validations = vec![
            Validator::email(&self.email),
            Validator::range(self.age, 18, 150, "age"),
            Validator::min_length(&self.name, 1, "name"),
        ];
        
        validate_all(validations)
    }
}
```

### Iterate Over Errors

```rust
match user.validate() {
    Ok(()) => println!("User is valid!"),
    Err(errors) => {
        for error in &errors {
            eprintln!("{}: {}", error.field, error.message);
        }
    }
}
```

### Filter by Field

```rust
let errors = user.validate().unwrap_err();
let email_errors: Vec<_> = errors.for_field("email");
```

### Quick Boolean Check

```rust
if user.is_valid() {
    // Proceed with user
}
```

---

## Comparison to Alternatives

| Approach | Error Type | Collection | Performance |
|----------|------------|------------|-------------|
| **Our decision** | `ValidationError` + `ValidationErrors` | Vec | Good |
| `validator` crate | `ValidationErrors` struct | Internal | Good |
| `pectus` | `Validation` struct | Vec | Good |
| `thiserror` only | Single error type | None | Excellent |

### Why Not Single Error Type?

A single error type returning `Result<(), MyError>` would require:
- Custom error enum with field variants
- No collection of multiple errors
- Lost context when only first error matters

---

## Alternatives Considered

### Alternative 1: Single Error Enum (rejected)

```rust
enum ValidationError {
    Email(String),
    Age(String),
    Name(String),
}
```

**Rejection Reason:**
- Enum must be extended for each new field
- No dynamic field names
- Pattern matching required
- Not extensible by users

### Alternative 2: Box<dyn Error> (rejected)

```rust
type ValidationResult = Result<(), Box<dyn Error>>;
```

**Rejection Reason:**
- Heap allocation always required
- No structured access to field/message
- Error downcasting required
- Not object-safe

### Alternative 3: Custom error with Vec inside (superseded)

```rust
struct ValidationError {
    field: String,
    message: String,
    children: Vec<ValidationError>,
}
```

**Decision:** Nested structure adds complexity without benefit. Flat structure is sufficient.

---

## Validation

### Test Cases

```rust
#[test]
fn test_validation_error_format() {
    let error = ValidationError::new("email", "Invalid email");
    assert_eq!(
        format!("{}", error),
        "Validation error on field 'email': Invalid email"
    );
}

#[test]
fn test_validation_errors_empty() {
    let errors = ValidationErrors::new();
    assert!(errors.is_empty());
}

#[test]
fn test_validation_errors_iteration() {
    let mut errors = ValidationErrors::new();
    errors.add(ValidationError::new("email", "Invalid"));
    errors.add(ValidationError::new("age", "Too young"));
    
    let fields: Vec<_> = errors.iter().map(|e| e.field.clone()).collect();
    assert_eq!(fields, vec!["email", "age"]);
}
```

---

## Related Decisions

- ADR-001: Trait-Based Validation Pattern
- ADR-003: Validator Composition Approach

---

## References

1. **thiserror crate** - github.com/dtolnay/thiserror
2. **Error handling in Rust** - Rust API guidelines

---

## Changelog

| Date | Version | Change | Author |
|------|---------|--------|--------|
| 2026-04-05 | 1.0 | Initial decision | Phenotype Team |

---

**END OF ADR**
