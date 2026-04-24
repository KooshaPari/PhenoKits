# ADR-001: Parser Combinator Architecture for Expression Languages

## Status
**ACCEPTED**

## Context

PhenoLang needs to parse multiple domain-specific languages (DSLs) across 33 modules, ranging from simple configuration formats to complex expression languages. The initial implementation used monolithic regex-based parsing, which has become unmaintainable as the language complexity grows.

Current parsing approaches in the codebase:
- Configuration parsing: Mix of YAML/JSON with custom validation
- Expression languages: Regex-based extraction (fragile, hard to extend)
- Query languages: String concatenation (SQL injection risks)

We need a unified, testable, and maintainable parsing strategy.

## Decision

We will adopt a **Parser Combinator Architecture** using the `parsy` library for all expression and mini-language parsing within PhenoLang.

### Decision Details

#### Primary: parsy (Parser Combinators)

Parser combinators enable composable, testable, and type-safe parsers through function composition:

```python
from parsy import string, regex, seq, alt, generate

# Atomic parsers
whitespace = regex(r'\s*')
identifier = regex(r'[a-zA-Z_][a-zA-Z0-9_]*')
number = regex(r'-?\d+(\.\d+)?').map(float)

# Combinators build complex parsers from simple ones
@generate
def port_spec():
    """Parse 'port: 8080' or 'port: ${ENV_PORT}'"""
    yield string('port')
    yield string(':')
    yield whitespace
    value = yield alt(number, variable_ref)
    return PortSpec(value)

@generate
def variable_ref():
    """Parse '${varname}' syntax"""
    yield string('${')
    name = yield identifier
    yield string('}')
    return VariableRef(name)
```

#### Secondary: Lark (Grammar-based DSLs)

For full configuration languages with complex syntax, we will use Lark with EBNF grammars:

```python
# pheno_config/grammar/pheno.lark
grammar = """
    ?start: config+
    
    config: section | setting | comment
    
    section: "[" identifier "]" "{" config* "}"
    
    setting: identifier ":" value
    
    value: STRING | NUMBER | BOOLEAN | list | reference
    
    list: "[" [value ("," value)*] "]"
    
    reference: "${" identifier ("." identifier)* "}"
    
    comment: /#.*$/
    
    STRING: /"[^"]*"/
    NUMBER: /-?\d+(\.\d+)?/
    BOOLEAN: "true" | "false"
    
    %ignore /\s+/
"""

from lark import Lark
parser = Lark(grammar, parser='lalr', transformer=PhenoConfigTransformer())
```

### Rationale

| Criterion | Parser Combinators | Parser Generators | Hand-written | Regex |
|-----------|-------------------|-------------------|--------------|-------|
| **Composability** | ★★★★★ | ★★★☆☆ | ★★☆☆☆ | ★☆☆☆☆ |
| **Testability** | ★★★★★ | ★★★★☆ | ★★★☆☆ | ★★☆☆☆ |
| **Maintainability** | ★★★★★ | ★★★☆☆ | ★★☆☆☆ | ★☆☆☆☆ |
| **Error Messages** | ★★★☆☆ | ★★★★★ | ★★★★★ | ★☆☆☆☆ |
| **Performance** | ★★★☆☆ | ★★★★★ | ★★★★★ | ★★★★★ |
| **Learning Curve** | ★★★★☆ | ★★☆☆☆ | ★★★☆☆ | ★★★★★ |

Parser combinators excel in the areas most critical for PhenoLang's continued evolution: composability and testability. Each production rule can be tested independently, and complex grammars emerge from simple, reusable components.

## Consequences

### Positive
- **Incremental Parsing**: Can parse partial inputs for real-time validation
- **Type Safety**: Parser results are properly typed through `map()` operations
- **Testability**: Each parser is a pure function, easily unit tested
- **Composability**: Complex parsers built from simple, reusable primitives
- **Maintainability**: Grammar changes are localized to specific combinators

### Negative
- **Error Messages**: Default error messages require enhancement for user-facing features
- **Performance**: Slightly slower than hand-written parsers (acceptable for our use case)
- **Learning Curve**: Team needs to understand monadic composition patterns
- **Debugging**: Stack traces can be deep due to nested function calls

### Mitigations

1. **Error Enhancement**: Implement custom `Parser.label()` and `Parser.desc()` for user-friendly errors
2. **Performance**: Profile critical paths; hand-write parsers only if profiling identifies bottlenecks
3. **Training**: Document patterns in `docs/patterns/parsing.md`
4. **Debugging**: Use `pdb` with `parsy` debug mode; add tracing decorators for development

## Implementation

### Directory Structure

```
pheno-parser/
├── src/
│   └── pheno_parser/
│       ├── __init__.py
│       ├── primitives.py        # Atomic parsers (whitespace, numbers, etc.)
│       ├── expressions.py       # Expression language parsers
│       ├── config.py            # Configuration format parsers
│       ├── query.py             # Query language parsers
│       ├── errors.py            # Parsing error types
│       └── transformers.py      # AST -> Domain object transformers
├── tests/
│   ├── test_primitives.py       # Property-based tests for primitives
│   ├── test_expressions.py      # Expression language tests
│   └── test_integration.py      # End-to-end parsing tests
└── grammar/
    └── pheno.lark               # Lark grammar for complex DSLs
```

### Example: Port Configuration Parser

```python
# pheno_parser/config.py
from dataclasses import dataclass
from typing import Union, Optional
from parsy import string, regex, seq, alt, success, fail

@dataclass(frozen=True)
class PortNumber:
    value: int
    
    def __post_init__(self):
        if not 1 <= self.value <= 65535:
            raise ValueError(f"Port must be 1-65535, got {self.value}")

@dataclass(frozen=True)
class PortReference:
    variable: str
    default: Optional[int] = None

PortSpec = Union[PortNumber, PortReference]

# Primitives
ws = regex(r'\s*')
identifier = regex(r'[a-zA-Z_][a-zA-Z0-9_]*')

# Variable reference parsing
variable_ref = (string('${') >> identifier << string('}')).map(PortReference)

# Number parsing with validation
def parse_port_number(s: str) -> PortNumber:
    try:
        return PortNumber(int(s))
    except ValueError as e:
        return fail(str(e))

port_number = regex(r'\d+').map(parse_port_number)

# Full port specification
port_spec = ws >> alt(
    variable_ref,
    port_number
) << ws

# Configuration line parser
port_config = seq(
    ws >> string('port') << ws << string(':') << ws,
    port_spec,
).map(lambda r: ("port", r[1]))
```

## Alternatives Considered

### Alternative 1: ANTLR4
- **Pros**: Mature, generates efficient parsers, excellent error messages
- **Cons**: Heavy dependency (Java runtime for generation), generated code, harder to test incrementally
- **Verdict**: Rejected for internal DSLs; may use for external-facing language servers

### Alternative 2: Hand-written Recursive Descent
- **Pros**: Maximum performance, full control over error messages
- **Cons**: Verbose, error-prone, hard to maintain across 33 modules
- **Verdict**: Rejected; maintenance burden too high for team size

### Alternative 3: PLY (Python Lex-Yacc)
- **Pros**: Mature, familiar to those with C background
- **Cons**: LALR parsing limitations, complex conflict resolution
- **Verdict**: Rejected; prefer modern parser combinator approach

## Related Decisions
- ADR-002: Domain-Driven Design Entity Boundaries
- ADR-005: Code Generation from Specifications

## References
- Ford, B. (2004). Parsing Expression Grammars. *POPL*.
- Hutton, G., & Meijer, E. (1998). Monadic Parser Combinators. *JFP*.
- parsy documentation: https://parsy.readthedocs.io/
- Lark documentation: https://lark-parser.readthedocs.io/

## Notes
- Migration from regex-based parsing will be gradual per-module
- Each module owner responsible for updating their parsing logic
- Target: All new parsing uses combinators by 2026-Q2
