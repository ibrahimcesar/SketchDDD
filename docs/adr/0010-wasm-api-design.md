# ADR-0010: WASM API Design

## Status
Accepted

## Context
The SketchDDD visual builder runs in the browser and needs access to the core Rust engine for:

1. **Parsing** - Convert DSL text to model
2. **Validation** - Check model consistency
3. **Serialization** - Export/import models as JSON
4. **Code Generation** - Generate code from models (optional in browser)

We need to compile the Rust core to WebAssembly and expose a JavaScript-friendly API.

Key considerations:
- Minimize WASM bundle size
- Efficient data transfer between JS and WASM
- Error handling across the boundary
- TypeScript type definitions
- Async operations for large models

## Decision

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Browser (JavaScript)                    │
├─────────────────────────────────────────────────────────────┤
│  React App ←→ TypeScript Bindings ←→ WASM Module            │
│                      │                    │                 │
│                      ↓                    ↓                 │
│              sketchddd.d.ts        sketchddd_bg.wasm        │
└─────────────────────────────────────────────────────────────┘
                              ↑
                              │ wasm-bindgen
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                  sketchddd-wasm (Rust)                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────────┐ │
│  │  Parser  │  │Validator │  │   Serde  │  │  Codegen    │ │
│  └──────────┘  └──────────┘  └──────────┘  └─────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Core API

```typescript
// sketchddd.d.ts - Generated TypeScript definitions

/**
 * Initialize the WASM module. Must be called before other functions.
 */
export function init(): Promise<void>;

/**
 * Parse DSL text into a model.
 */
export function parse(source: string): ParseResult;

/**
 * Validate a model for consistency.
 */
export function validate(model: Model): ValidationResult;

/**
 * Serialize model to JSON string.
 */
export function toJson(model: Model): string;

/**
 * Deserialize model from JSON string.
 */
export function fromJson(json: string): Model;

/**
 * Generate code for a target language.
 */
export function generateCode(model: Model, target: CodegenTarget): string;
```

### Data Types

```typescript
// Core model types
export interface Model {
  contexts: BoundedContext[];
  contextMaps: ContextMap[];
}

export interface BoundedContext {
  name: string;
  objects: DomainObject[];
  morphisms: Morphism[];
  entities: Entity[];
  valueObjects: ValueObject[];
  aggregates: Aggregate[];
  enums: Enumeration[];
}

export interface ParseResult {
  success: boolean;
  model?: Model;
  errors: ParseError[];
  warnings: ParseWarning[];
}

export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
}

export interface ValidationError {
  code: string;
  message: string;
  severity: 'error' | 'warning' | 'hint';
  location?: SourceLocation;
  suggestion?: string;
}

export interface SourceLocation {
  file?: string;
  line: number;
  column: number;
  length?: number;
}

export type CodegenTarget =
  | 'rust'
  | 'typescript'
  | 'kotlin'
  | 'python'
  | 'java'
  | 'clojure';
```

### Rust Implementation

```rust
// sketchddd-wasm/src/lib.rs

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use sketchddd_core::*;
use sketchddd_parser::*;

#[wasm_bindgen]
pub fn parse(source: &str) -> JsValue {
    let result = match sketchddd_parser::parse_file(source) {
        Ok(ast) => match transform(&ast) {
            Ok(model) => ParseResultJs {
                success: true,
                model: Some(model.into()),
                errors: vec![],
                warnings: vec![],
            },
            Err(e) => ParseResultJs {
                success: false,
                model: None,
                errors: vec![e.into()],
                warnings: vec![],
            }
        },
        Err(e) => ParseResultJs {
            success: false,
            model: None,
            errors: vec![e.into()],
            warnings: vec![],
        }
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn validate(model: JsValue) -> JsValue {
    let model: ModelJs = serde_wasm_bindgen::from_value(model).unwrap();
    let result = sketchddd_core::validate_model(&model.into());
    serde_wasm_bindgen::to_value(&ValidationResultJs::from(result)).unwrap()
}

#[wasm_bindgen]
pub fn to_json(model: JsValue) -> String {
    let model: ModelJs = serde_wasm_bindgen::from_value(model).unwrap();
    serde_json::to_string_pretty(&model).unwrap()
}

#[wasm_bindgen]
pub fn from_json(json: &str) -> JsValue {
    let model: ModelJs = serde_json::from_str(json).unwrap();
    serde_wasm_bindgen::to_value(&model).unwrap()
}
```

### Error Handling

Errors are returned as structured data, not thrown exceptions:

```typescript
const result = parse(source);
if (!result.success) {
  for (const error of result.errors) {
    console.error(`${error.code}: ${error.message}`);
    if (error.location) {
      console.error(`  at line ${error.location.line}`);
    }
  }
}
```

### Bundle Size Optimization

1. **Feature Flags**: Exclude codegen from browser builds if not needed
2. **wasm-opt**: Run optimization pass
3. **Compression**: Serve with gzip/brotli
4. **Lazy Loading**: Load WASM only when needed

```toml
# Cargo.toml
[features]
default = ["parser", "validator"]
full = ["parser", "validator", "codegen"]
codegen = ["sketchddd-codegen"]

[profile.release]
opt-level = "z"  # Optimize for size
lto = true
```

Expected sizes:
- Core (parse + validate): ~200KB gzipped
- Full (with codegen): ~400KB gzipped

### Async Support

For large models, expose async versions:

```typescript
export async function parseAsync(source: string): Promise<ParseResult>;
export async function validateAsync(model: Model): Promise<ValidationResult>;
```

### Usage Example

```typescript
import init, { parse, validate, toJson } from 'sketchddd-wasm';

async function loadModel(source: string) {
  // Initialize WASM module
  await init();

  // Parse DSL
  const parseResult = parse(source);
  if (!parseResult.success) {
    throw new Error(parseResult.errors[0].message);
  }

  // Validate
  const validation = validate(parseResult.model);
  if (!validation.valid) {
    console.warn('Validation warnings:', validation.warnings);
  }

  // Serialize
  const json = toJson(parseResult.model);
  localStorage.setItem('model', json);

  return parseResult.model;
}
```

### Build Process

```bash
# Build WASM module
cd crates/sketchddd-wasm
wasm-pack build --target web --release

# Output structure
pkg/
├── sketchddd_wasm.js      # JS glue code
├── sketchddd_wasm_bg.wasm # WASM binary
├── sketchddd_wasm.d.ts    # TypeScript types
└── package.json
```

## Consequences

### Positive
- Full Rust engine available in browser
- Type-safe API with TypeScript definitions
- Structured error handling
- Same validation logic in CLI and browser
- Small bundle size with feature flags

### Negative
- WASM adds initial load time (~100ms)
- Debugging across JS/WASM boundary is harder
- Memory management requires care
- Some Rust features unavailable (file I/O, threads)

### Neutral
- Requires wasm-pack toolchain
- Bundle served alongside JS application
- Works in all modern browsers

## References
- [Issue #13: Compile core engine to WASM](https://github.com/ibrahimcesar/SketchDDD/issues/13)
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)
- [serde-wasm-bindgen](https://github.com/cloudflare/serde-wasm-bindgen)
