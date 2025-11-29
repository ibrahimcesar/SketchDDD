# ADR-0002: Use Rust for Core Engine

## Status
Accepted

## Context
The core engine of SketchDDD must:
1. Parse and validate domain models
2. Generate code for multiple target languages
3. Run in the browser via WebAssembly (WASM)
4. Run as a native CLI tool
5. Be performant for large models
6. Be memory-safe and reliable

We evaluated several languages:
- **TypeScript**: Good WASM story but runtime overhead, type system limitations
- **Go**: Good performance but poor WASM support, no algebraic data types
- **Haskell**: Excellent for DSLs but WASM compilation is immature
- **Rust**: Excellent WASM support, strong type system, no GC, algebraic data types

## Decision
We will use **Rust** for the core engine, including:

- `sketchddd-core`: Core data structures and semantics
- `sketchddd-parser`: DSL parser using pest
- `sketchddd-codegen`: Code generation
- `sketchddd-viz`: Visualization generation
- `sketchddd-wasm`: WebAssembly bindings
- `sketchddd-cli`: Command-line interface

## Consequences

### Positive
- **WASM support**: First-class compilation to WebAssembly via `wasm-bindgen`
- **Performance**: Zero-cost abstractions, no garbage collection pauses
- **Type system**: Algebraic data types (enums) model categorical structures naturally
- **Memory safety**: No null pointers, no data races
- **Ecosystem**: Excellent parser libraries (pest), CLI libraries (clap)
- **Single codebase**: Same code runs natively and in browser

### Negative
- **Learning curve**: Rust has a steep learning curve for new contributors
- **Compilation time**: Rust compile times are longer than Go/TypeScript
- **Binary size**: WASM binaries may be larger than hand-written JS

### Neutral
- Rust's ownership model requires careful API design for the WASM boundary
- The choice enables but doesn't require advanced type-level programming

## References
- [Rust WASM Working Group](https://rustwasm.github.io/)
- [wasm-bindgen documentation](https://rustwasm.github.io/wasm-bindgen/)
- [pest parser documentation](https://pest.rs/)
