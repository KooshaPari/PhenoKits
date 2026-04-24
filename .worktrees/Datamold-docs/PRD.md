# PRD: model — TypeScript Data Modeling Library

## Overview
`model` is a TypeScript-first data modeling and validation library. It provides schema definitions, runtime type validation, data transformation, and serialization (JSON, YAML, TOML) with full TypeScript type inference.

## Problem Statement
TypeScript projects need runtime type checking that mirrors compile-time types. Existing solutions (Zod, Yup, io-ts) have tradeoffs between ergonomics, bundle size, and inference quality. `model` provides a Phenotype-standard, opinionated approach.

## Goals
1. Define data schemas once, get runtime validation and TypeScript types
2. Composable validators with clear error messages
3. Transform and sanitize data at schema boundaries
4. Serialize/deserialize across JSON, YAML, TOML formats

## Epics

### E1: Schema Definition
- E1.1: Primitive types (string, number, boolean, date, bigint)
- E1.2: Composite types (object, array, union, intersection, tuple)
- E1.3: Schema modifiers (optional, nullable, default, brand)

### E2: Validation Engine
- E2.1: Parse (throw on invalid) and safeParse (return Result)
- E2.2: Detailed validation error reporting with field paths
- E2.3: Custom validators and refinements

### E3: Transformation
- E3.1: Input sanitization (trim, lowercase, coerce)
- E3.2: Output transformation (pick, omit, rename)
- E3.3: Schema composition (merge, extend, partial, required)

### E4: Serialization
- E4.1: JSON serialization with Date/BigInt handling
- E4.2: YAML parsing and serialization
- E4.3: TOML parsing and serialization

## Acceptance Criteria
- Full TypeScript inference: `Model.infer<typeof schema>` works for all schema types
- Bundle size < 10KB gzipped for core
- 100% of parse errors include field path and expected/received types
