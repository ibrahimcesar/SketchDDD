# SketchDDD

A Categorical Framework for Domain-Driven Design.

**Build precise domain models visually or with code.**

## Overview

SketchDDD bridges the gap between domain experts and developers by providing precise mathematical definitions for DDD concepts using category theory.

| DDD Concept | Mathematical Definition |
|-------------|------------------------|
| Bounded Context | Sketch (graph + equations + limits) |
| Aggregate | Limit cone with root |
| Value Object | Limit with structural equality |
| Context Map | Sketch morphism |

## Installation

```toml
[dependencies]
sketchddd = "0.1"
```

## Quick Start

```rust
use sketchddd::prelude::*;

// Create a bounded context
let mut ctx = BoundedContext::new("Commerce");

// Add entities
let customer = ctx.add_entity("Customer");
let order = ctx.add_entity("Order");

// Add value objects
let money = ctx.add_value_object("Money");

// Add enumerations
let status = ctx.add_enum("OrderStatus", vec![
    "Pending".into(),
    "Confirmed".into(),
    "Shipped".into(),
]);
```

## Crates

- `sketchddd` - This crate (facade)
- `sketchddd-core` - Core data structures
- `sketchddd-parser` - DSL parser
- `sketchddd-codegen` - Code generation
- `sketchddd-viz` - Visualization
- `sketchddd-cli` - Command-line tool

## License

Licensed under either of [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE) at your option.
