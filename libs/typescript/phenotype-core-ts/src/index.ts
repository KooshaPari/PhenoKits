/**
 * @phenotype/core - TypeScript SDK for Phenotype Core
 * 
 * This package provides TypeScript/JavaScript bindings for the
 * Phenotype Core library, with WASM acceleration where available.
 */

// Core types
export {
  EntityId,
  Entity,
  ValidationResult,
  validateEntity,
  generateId
} from './entity';

// Config types
export {
  Config,
  ConfigSource,
  Configuration,
  ConfigStore
} from './config';

// Re-export WASM bindings when available
export {
  WasmEntityId,
  WasmConfig,
  validateEntity as validateEntityWasm
} from '@phenotype/core-wasm';
