# Functional Requirements: model

## FR-MOD-001: Primitive Schema Types
The library SHALL support schema definitions for: string, number, boolean, Date, bigint, symbol, null, undefined, and literal values.

## FR-MOD-002: Composite Schema Types
FR-MOD-002a: The library SHALL support object schemas with named field definitions.
FR-MOD-002b: The library SHALL support array schemas with element type validation.
FR-MOD-002c: The library SHALL support union schemas (A | B | C).
FR-MOD-002d: The library SHALL support intersection schemas (A & B).
FR-MOD-002e: The library SHALL support tuple schemas with positional types.

## FR-MOD-003: Schema Modifiers
FR-MOD-003a: Any schema SHALL support `.optional()` to allow undefined.
FR-MOD-003b: Any schema SHALL support `.nullable()` to allow null.
FR-MOD-003c: Any schema SHALL support `.default(value)` to supply a default on missing input.
FR-MOD-003d: String schemas SHALL support `.min(n)`, `.max(n)`, `.email()`, `.url()`, `.regex(r)`.
FR-MOD-003e: Number schemas SHALL support `.min(n)`, `.max(n)`, `.int()`, `.positive()`.

## FR-MOD-004: Parsing API
FR-MOD-004a: Every schema SHALL expose `.parse(input): T` that throws `ModelValidationError` on failure.
FR-MOD-004b: Every schema SHALL expose `.safeParse(input): { success: true; data: T } | { success: false; error: ModelValidationError }`.
FR-MOD-004c: `ModelValidationError` SHALL include an array of issues, each with: `path` (string[]), `code`, `message`, `expected`, `received`.

## FR-MOD-005: Custom Validation
FR-MOD-005a: Any schema SHALL support `.refine(fn, message)` for custom validation predicates.
FR-MOD-005b: Object schemas SHALL support `.superRefine(fn)` for cross-field validation.

## FR-MOD-006: Transformation
FR-MOD-006a: Any schema SHALL support `.transform(fn)` to map valid values before returning.
FR-MOD-006b: String schemas SHALL support `.trim()`, `.toLowerCase()`, `.toUpperCase()` transforms.

## FR-MOD-007: TypeScript Inference
FR-MOD-007a: The library SHALL export `Infer<T>` type helper that extracts the TypeScript type from any schema.
FR-MOD-007b: All schema methods SHALL preserve full TypeScript type inference through chaining.

## FR-MOD-008: Serialization
FR-MOD-008a: The library SHALL provide `toJSON(schema, value)` and `fromJSON(schema, json)` utilities.
FR-MOD-008b: JSON serialization SHALL correctly handle Date (ISO string), BigInt (string), and Map/Set.
FR-MOD-008c: The library SHALL provide YAML and TOML serialization adapters as optional imports.
