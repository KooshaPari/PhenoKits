# ADR-003: Validator Composition Approach

**Status:** Accepted  
**Date:** 2026-04-05  
**Author:** Phenotype Architecture Team  
**Reviewers:** Phenotype Core Team  
**Supersedes:** N/A  
**Superseded by:** N/A

---

## Context

Validation often requires combining multiple validation rules. A user might need to validate:
1. Email format
2. Minimum length
3. Required presence

This ADR defines how phenotype-validation enables composing multiple validators into reusable validation logic.

### Design Requirements

1. **Ergonomic combination:** Easy to combine multiple validators
2. **Reusable rules:** Define validation logic once, apply many times
3. **Short-circuit option:** Stop at first error OR collect all errors
4. **No macros:** Plain Rust functions, no procedural macros
5. **User-extensible:** Users can create custom validators

---

## Decision

Provide a **functional composition approach** using the `Validator` struct with static methods and a `validate_all` function for aggregation.

### Validator Struct

```rust
pub struct Validator;

impl Validator {
    pub fn email(email: &str) -> Result<(), ValidationError> { /* ... */ }
    pub fn url(url: &str) -> Result<(), ValidationError> { /* ... */ }
    pub fn uuid(uuid: &str) -> Result<(), ValidationError> { /* ... */ }
    pub fn not_empty(value: &str, field: &str) -> Result<(), ValidationError> { /* ... */ }
    pub fn min_length(value: &str, min: usize, field: &str) -> Result<(), ValidationError> { /* ... */ }
    pub fn max_length(value: &str, max: usize, field: &str) -> Result<(), ValidationError> { /* ... */ }
    pub fn range<T: PartialOrd + std::fmt::Debug>(value: T, min: T, max: T, field: &str) -> Result<(), ValidationError> { /* ... */ }
    pub fn regex(value: &str, pattern: &str, field: &str) -> Result<(), ValidationError> { /* ... */ }
}
```

### Aggregation Function

```rust
pub fn validate_all(validations: Vec<Result<(), ValidationError>>) -> Result<(), ValidationErrors> {
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

### Composition Pattern

Users combine validators by calling `validate_all` with a vector of results:

```rust
impl Validate for User {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_all(vec![
            Validator::email(&self.email),
            Validator::range(self.age, 18, u32::MAX, "age"),
            Validator::not_empty(&self.name, "name"),
            Validator::min_length(&self.name, 2, "name"),
            Validator::max_length(&self.name, 100, "name"),
        ])
    }
}
```

---

## Consequences

### Positive

1. **No macro magic:** Plain function calls, easy to understand
2. **Full control:** Users decide which validators to combine
3. **Testable:** Each validator is independently testable
4. **Extensible:** Users can create their own validator functions
5. **Debuggable:** Stack traces show exact validation location

### Negative

1. **Boilerplate:** `validate_all(vec![...])` is verbose
2. **Order dependency:** Validators run in vector order
3. **No lazy evaluation:** All validators run even if one fails
4. **No short-circuit:** Cannot stop at first error with `validate_all`

### Mitigations

1. **Short-circuit method:** `is_valid()` stops on first error
2. **Future: chain! macro:** Consider adding macro for cleaner syntax
3. **Custom helpers:** Users can create domain-specific validators

---

## Implementation Details

### Validator Function Signature

All built-in validators follow the same pattern:

```rust
pub fn email(email: &str) -> Result<(), ValidationError>
pub fn url(url: &str) -> Result<(), ValidationError>
pub fn not_empty(value: &str, field: &str) -> Result<(), ValidationError>
```

**Rationale:**
- Input as reference (no ownership taken)
- `&str` for strings, generic `T` for numeric types
- Returns `Result<(), ValidationError>` (not `Result<(), ValidationErrors>`)
- Field name passed explicitly for context

### Custom Validators

Users create validators as regular functions:

```rust
pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let phone_regex = regex::Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    
    if phone_regex.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("phone", "Invalid phone number"))
    }
}
```

### Composable Helper Functions

Users can create reusable validation functions:

```rust
fn validate_email_field(email: &str) -> Result<(), ValidationErrors> {
    validate_all(vec![
        Validator::not_empty(email, "email"),
        Validator::email(email),
    ])
}

fn validate_password(password: &str) -> Result<(), ValidationErrors> {
    validate_all(vec![
        Validator::min_length(password, 8, "password"),
        Validator::regex(password, r"[A-Z]", "password"),
        Validator::regex(password, r"[0-9]", "password"),
    ])
}
```

---

## Comparison to Alternatives

### Alternative 1: Builder Pattern (rejected)

```rust
let validator = Validator::new()
    .email()
    .range(18, u32::MAX)
    .required();
```

**Rejection Reason:**
- Complex state management
- Hard to test individual rules
- Builder state must be validated
- Less explicit

### Alternative 2: Chain/Monad Pattern (deferred)

```rust
email.validate().and_then(password.validate())
```

**Decision:** Deferred to future. Current approach is simpler and sufficient.

### Alternative 3: Macro-based (deferred)

```rust
validate! {
    email: email,
    age: range(18, 150),
}
```

**Decision:** Deferred. Consider if boilerplate becomes a real problem.

---

## Design Patterns

### Pattern 1: Basic Field Validation

```rust
impl Validate for User {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_all(vec![
            Validator::not_empty(&self.email, "email"),
            Validator::email(&self.email),
        ])
    }
}
```

### Pattern 2: Multiple Constraints

```rust
impl Validate for Password {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_all(vec![
            Validator::min_length(&self.0, 8, "password"),
            Validator::max_length(&self.0, 128, "password"),
            Validator::regex(&self.0, r"[A-Z]", "password"),
            Validator::regex(&self.0, r"[a-z]", "password"),
            Validator::regex(&self.0, r"[0-9]", "password"),
        ])
    }
}
```

### Pattern 3: Cross-Field Validation

```rust
impl Validate for Event {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        if self.end_time <= self.start_time {
            errors.add(ValidationError::new(
                "end_time",
                "End time must be after start time"
            ));
        }
        
        validate_all(vec![
            Validator::range(self.start_time, 0, i64::MAX, "start_time"),
        ]).unwrap_or_else(|e| errors.extend(e));
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Pattern 4: Nested Validation

```rust
impl Validate for Order {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        // Validate nested
        if let Err(e) = self.customer.validate() {
            errors.extend(e);
        }
        
        // Validate collection
        for (i, item) in self.items.iter().enumerate() {
            if let Err(e) = item.validate() {
                for mut err in e.errors.into_iter() {
                    err.field = format!("items[{}].{}", i, err.field);
                    errors.add(err);
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

---

## Future Enhancements

### Potential Improvements

1. **Chain macro:**
   ```rust
   chain![Validator::email, Validator::not_empty].validate(&email)
   ```

2. **Short-circuit variant:**
   ```rust
   validate_first(vec![
       Validator::email(&self.email),
       // ...
   ])
   ```

3. **Validation builders:**
   ```rust
   let email_validator = Validators::email().required();
   email_validator.validate(&self.email)?;
   ```

---

## Validation

### Test Strategy

```rust
#[test]
fn test_validate_all_success() {
    let validations = vec![
        Validator::email("test@example.com"),
        Validator::not_empty("hello", "field"),
    ];
    assert!(validate_all(validations).is_ok());
}

#[test]
fn test_validate_all_collects_errors() {
    let validations = vec![
        Validator::email("invalid"),
        Validator::email("also@invalid"),
        Validator::not_empty("hello", "field"),
    ];
    let result = validate_all(validations);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().errors.len(), 2);
}
```

---

## Related Decisions

- ADR-001: Trait-Based Validation Pattern
- ADR-002: Error Aggregation Strategy

---

## References

1. **Functional composition in Rust** - Various Rust educational materials
2. **Validation patterns** - Compare with validator crate, pectus crate

---

## Changelog

| Date | Version | Change | Author |
|------|---------|--------|--------|
| 2026-04-05 | 1.0 | Initial decision | Phenotype Team |

---

**END OF ADR**
