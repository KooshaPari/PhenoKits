# FUNCTIONAL_REQUIREMENTS — phenotype-cli-extensions

## FR-CORE-001: Plugin Discovery
**SHALL** scan `~/.phenotype/extensions/` and `PHENOTYPE_EXT_PATH` for installed extension manifests.
Traces to: E1.1

## FR-CORE-002: Manifest Validation
**SHALL** validate extension manifests against JSON Schema before loading.
Traces to: E2.2, E3.2

## FR-CORE-003: Plugin Installation
**SHALL** support `phenotype ext install <name|path>` to fetch and install extensions.
Traces to: E1.2

## FR-CORE-004: Plugin Activation
**SHALL** load extension entry point in isolated module scope; collisions in global namespace are a hard error.
Traces to: E1.3

## FR-CORE-005: Plugin Removal
**SHALL** support `phenotype ext remove <name>` and clean all extension artifacts.
Traces to: E1.4

## FR-API-001: Extension Interface
**SHALL** expose a typed `PhenotypeExtension` interface with `commands`, `hooks`, and `middleware` fields.
Traces to: E2.1

## FR-API-002: Capability Declaration
**SHALL** require extensions to declare capabilities in manifest; CLI enforces declared vs. used capability match.
Traces to: E2.3

## FR-SEC-001: Integrity Verification
**SHALL** verify SHA-256 checksum of extension archives before extraction; reject mismatches with a hard error.
Traces to: E3.3

## FR-DX-001: Scaffolding
**SHALL** provide `phenotype ext init` that scaffolds a valid extension skeleton with tests.
Traces to: E4.1

## FR-DX-002: Test Harness
**SHALL** provide a `PhenotypeExtensionTestContext` helper for unit-testing extensions in isolation.
Traces to: E4.3
