# ADR-002: Schema Validation Strategy for Hexagonal Ports

**Date**: 2026-04-04  
**Status**: Proposed  
**Deciders**: Phenotype Architecture Team  
**Related**: [SOTA Research](./SOTA-RESEARCH.md), [ADR-001](./ADR-001-api-contract-format.md)

## Context

The Phenotype Contracts system requires robust schema validation at multiple layers:

1. **Input Validation**: Validating incoming requests at port boundaries
2. **Output Validation**: Ensuring responses conform to contracts
3. **Event Validation**: Validating domain events before persistence
4. **Configuration Validation**: Validating plugin and adapter configurations

Validation must be:
- Type-safe (compile-time where possible)
- Fast at runtime
- Consistent across languages (Go, Rust, TypeScript, Python)
- Testable and mockable
- Composable (validators can be combined)

## Decision Drivers

| Driver | Weight | Description |
|--------|--------|-------------|
| **Type Safety** | Critical | Compile-time type checking preferred |
| **Performance** | High | Validation must not bottleneck requests |
| **Composability** | High | Validators should compose like middleware |
| **Error Quality** | High | Clear, actionable error messages |
| **Cross-Language** | Medium | Similar patterns across languages |
| **Testing** | Medium | Easy to test validators in isolation |

## Options Considered

### Option A: JSON Schema Runtime Validation

**Description**: Validate using JSON Schema at runtime with libraries like `jsonschema` (Go) or `ajv` (TypeScript).

**Pros**:
- Single source of truth (OpenAPI schema)
- Consistent across all languages
- Can validate partial updates
- Rich validation vocabulary

**Cons**:
- Runtime overhead
- Error messages can be cryptic
- No compile-time safety
- Requires schema distribution

**Performance**:

| Validator | Cold (ms) | Warm (μs/op) |
|-----------|-----------|--------------|
| jsonschema (Go) | 8 | 2 |
| jsonschema-rs (Rust) | 5 | 0.5 |
| ajv (TypeScript) | 15 | 1 |

### Option B: Native Type System Validation

**Description**: Use language-native type systems with runtime validation libraries.

**Go**: `go-playground/validator` with struct tags  
**Rust**: `validator` derive macros  
**TypeScript**: Zod or io-ts  
**Python**: Pydantic

**Pros**:
- Idiomatic for each language
- Better error messages
- No schema distribution needed
- Can be compile-time checked (some languages)

**Cons**:
- Duplicated validation logic across languages
- Different validation patterns per language
- Schema drift risk

**Example - Go**:

```go
type CreateOrderRequest struct {
    CustomerID string      `validate:"required,uuid"`
    Items      []OrderItem `validate:"required,min=1,dive"`
    Total      Money       `validate:"required"`
}

func (r *CreateOrderRequest) Validate() error {
    return validate.Struct(r)
}
```

**Example - TypeScript**:

```typescript
const CreateOrderRequest = z.object({
    customerId: z.string().uuid(),
    items: z.array(OrderItem).min(1),
    total: Money
});

type CreateOrderRequest = z.infer<typeof CreateOrderRequest>;
```

**Example - Rust**:

```rust
#[derive(Validate)]
struct CreateOrderRequest {
    #[validate(uuid)]
    customer_id: String,
    #[validate(length(min = 1))]
    items: Vec<OrderItem>,
    #[validate]
    total: Money,
}
```

### Option C: Code-Generated Validators

**Description**: Generate validation code from OpenAPI schemas.

**Pros**:
- Single source of truth (OpenAPI)
- Native performance
- Type-safe
- Consistent across languages

**Cons**:
- Build-time complexity
- Generated code can be verbose
- Limited customization

**Tools**:
- openapi-generator (Java-based)
- oapi-codegen + custom templates
- protobuf with protoc-gen-validate

### Option D: Hybrid Approach (Recommended)

**Description**: Combine compile-time types with runtime validation, using OpenAPI as source of truth with generated validator types.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Hybrid Validation Architecture                             │
│                                                                             │
│  OpenAPI Schema                                                             │
│       │                                                                     │
│       ▼                                                                     │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐         │
│  │   Code Gen      │───►│  Native Types   │───►│  Runtime Valid  │         │
│  │   (oapi-codegen)│    │  (Go/Rust/TS)   │    │  (go-validator) │         │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘         │
│       │                                                                     │
│       ▼                                                                     │
│  ┌─────────────────┐    ┌─────────────────┐                                 │
│  │ JSON Schema     │───►│  Edge Validate  │                                 │
│  │ (for REST)      │    │  (API Gateway)  │                                 │
│  └─────────────────┘    └─────────────────┘                                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Decision

**Adopt a Hybrid Validation Strategy**:

1. **OpenAPI schemas** as source of truth for API contracts
2. **Code-generated types** with native validation for application layer
3. **JSON Schema validation** at API gateway/load balancer for early rejection
4. **Language-native validators** for business logic validation

### Validation Layers

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Request Flow with Validation                          │
│                                                                             │
│   Request                                                                   │
│     │                                                                       │
│     ▼                                                                       │
│  ┌──────────────────────┐  Fast rejection, minimal overhead                │
│  │ JSON Schema          │  ◄── 1. Gateway Validation (OpenAPI)            │
│  │ (API Gateway)        │                                                 │
│  └──────────┬───────────┘                                                 │
│             │                                                               │
│             ▼                                                               │
│  ┌──────────────────────┐  Type safety + business rules                    │
│  │ Native Types         │  ◄── 2. Port Validation (Generated)             │
│  │ + go-validator       │                                                 │
│  └──────────┬───────────┘                                                 │
│             │                                                               │
│             ▼                                                               │
│  ┌──────────────────────┐  Domain invariants                                │
│  │ Domain Validation    │  ◄── 3. Domain Layer (Custom)                 │
│  │ (Custom Logic)       │                                                 │
│  └──────────┬───────────┘                                                 │
│             │                                                               │
│             ▼                                                               │
│       Use Case Execution                                                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Language-Specific Implementation

#### Go

```go
// Generated from OpenAPI + custom validator tags
package contracts

//go:generate oapi-codegen --config=cfg.yaml ../../openapi/orders.yaml

// Additional validation methods
func (r CreateOrderRequest) Validate() error {
    validate := validator.New()
    
    // Generated struct validation
    if err := validate.Struct(r); err != nil {
        return NewValidationError(err)
    }
    
    // Custom business validation
    if r.Total.Amount <= 0 {
        return ErrInvalidTotal
    }
    
    // Cross-field validation
    calculatedTotal := calculateTotal(r.Items)
    if !calculatedTotal.Equal(r.Total) {
        return ErrTotalMismatch
    }
    
    return nil
}
```

#### Rust

```rust
use validator::{Validate, ValidationError};

#[derive(Validate, Debug)]
#[validate(schema(function = "validate_create_order"))]
pub struct CreateOrderRequest {
    #[validate(uuid)]
    pub customer_id: String,
    
    #[validate(length(min = 1, message = "At least one item required"))]
    pub items: Vec<OrderItem>,
    
    #[validate]
    pub total: Money,
}

fn validate_create_order(req: &CreateOrderRequest) -> Result<(), ValidationError> {
    let calculated = calculate_total(&req.items);
    if calculated != req.total {
        return Err(ValidationError::new("total_mismatch"));
    }
    Ok(())
}
```

#### TypeScript

```typescript
import { z } from 'zod';

// Generated base schema from OpenAPI
const CreateOrderRequestSchema = z.object({
    customerId: z.string().uuid(),
    items: z.array(OrderItemSchema).min(1),
    total: MoneySchema,
}).refine(
    (data) => {
        const calculated = calculateTotal(data.items);
        return calculated.equals(data.total);
    },
    { message: 'Total does not match sum of items', path: ['total'] }
);

export type CreateOrderRequest = z.infer<typeof CreateOrderRequestSchema>;

export function validateCreateOrder(data: unknown): CreateOrderRequest {
    return CreateOrderRequestSchema.parse(data);
}
```

## Implementation Plan

### Phase 1: Foundation (Weeks 1-2)
- [ ] Define validation error standard format
- [ ] Set up oapi-codegen with custom templates for Go
- [ ] Implement JSON Schema validation at API gateway
- [ ] Create shared validation utilities

### Phase 2: Language Implementations (Weeks 3-6)
- [ ] Go: go-playground/validator integration
- [ ] Rust: validator crate with custom derives
- [ ] TypeScript: Zod schemas with OpenAPI generation
- [ ] Python: Pydantic models with shared patterns

### Phase 3: Testing & Tooling (Weeks 7-8)
- [ ] Property-based testing with Schemathesis
- [ ] Fuzzing for validation edge cases
- [ ] Validation performance benchmarks
- [ ] Contract compliance testing

### Phase 4: Advanced Features (Ongoing)
- [ ] Custom validation DSL
- [ ] AI-assisted validation rule generation
- [ ] Cross-language validation consistency checker
- [ ] Real-time validation metrics

## Validation Error Standard

All validation errors follow a standard format:

```json
{
    "error": "validation_failed",
    "message": "Request validation failed",
    "code": "VALIDATION_ERROR",
    "details": {
        "fields": [
            {
                "field": "items",
                "code": "min_items",
                "message": "At least one item is required",
                "value": []
            },
            {
                "field": "total",
                "code": "total_mismatch",
                "message": "Total does not match sum of items",
                "expected": 99.99,
                "actual": 89.99
            }
        ]
    },
    "request_id": "req_1234567890"
}
```

## Consequences

### Positive
- Type safety at compile time where possible
- Runtime validation ensures data integrity
- Clear error messages for API consumers
- Performance-appropriate validation at each layer
- Single source of truth (OpenAPI) with native implementations

### Negative
- Multiple validation implementations to maintain
- Potential for drift between OpenAPI and native validators
- Build complexity from code generation
- Learning curve for developers

## Testing Strategy

### Unit Tests

```go
func TestCreateOrderRequest_Validate(t *testing.T) {
    tests := []struct {
        name    string
        request CreateOrderRequest
        wantErr bool
        errCode string
    }{
        {
            name: "valid request",
            request: CreateOrderRequest{
                CustomerID: uuid.New().String(),
                Items: []OrderItem{{ProductID: "p1", Quantity: 1, Price: 10.00}},
                Total: Money{Amount: 10.00, Currency: "USD"},
            },
            wantErr: false,
        },
        {
            name: "invalid uuid",
            request: CreateOrderRequest{
                CustomerID: "not-a-uuid",
                Items: []OrderItem{{ProductID: "p1", Quantity: 1, Price: 10.00}},
                Total: Money{Amount: 10.00, Currency: "USD"},
            },
            wantErr: true,
            errCode: "uuid",
        },
        {
            name: "total mismatch",
            request: CreateOrderRequest{
                CustomerID: uuid.New().String(),
                Items: []OrderItem{
                    {ProductID: "p1", Quantity: 2, Price: 10.00}, // 20.00 total
                },
                Total: Money{Amount: 15.00, Currency: "USD"}, // Wrong!
            },
            wantErr: true,
            errCode: "total_mismatch",
        },
    }
    
    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := tt.request.Validate()
            if tt.wantErr {
                assert.Error(t, err)
                var valErr *ValidationError
                if errors.As(err, &valErr) {
                    assert.Equal(t, tt.errCode, valErr.Code)
                }
            } else {
                assert.NoError(t, err)
            }
        })
    }
}
```

### Property-Based Tests

```python
# Using Hypothesis/Schemathesis
from hypothesis import given, strategies as st
import schemathesis

schema = schemathesis.from_path("openapi.yaml")

@schema.parametrize()
def test_api_contract(case):
    """All API responses must match OpenAPI schema."""
    case.call_and_validate()

@given(st.data())
def test_validation_properties(data):
    """Generated inputs should be correctly validated."""
    # Generate random valid/invalid inputs
    # Ensure validation catches all invalid inputs
    pass
```

## References

- [JSON Schema 2020-12](https://json-schema.org/draft/2020-12/schema)
- [go-playground/validator](https://github.com/go-playground/validator)
- [validator.rs](https://github.com/Keats/validator)
- [Zod](https://zod.dev/)
- [Pydantic](https://docs.pydantic.dev/)
- [SOTA Research: Schema Validation](./SOTA-RESEARCH.md#schema-validation-technologies)

---

*This ADR will be updated as validation implementation progresses.*
