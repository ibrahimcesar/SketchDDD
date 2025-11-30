//! # SketchDDD Core
//!
//! Core categorical semantics and data structures for domain-driven design.
//!
//! This crate provides the mathematical foundation for SketchDDD, modeling
//! DDD concepts using category theory. A bounded context is represented as
//! a **sketch** `S = (G, E, L, C)` where:
//!
//! - `G`: Directed graph (objects and morphisms)
//! - `E`: Path equations (business rules)
//! - `L`: Limit cones (aggregates, value objects)
//! - `C`: Colimit cocones (sum types, enumerations)
//!
//! ## DDD to Category Theory Mapping
//!
//! | DDD Concept | Categorical Structure |
//! |-------------|----------------------|
//! | Bounded Context | Sketch |
//! | Ubiquitous Language | Graph + Equations |
//! | Entity | Object with identity morphism |
//! | Value Object | Limit with structural equality |
//! | Aggregate | Limit cone with root |
//! | Invariant | Equalizer |
//! | Context Map | Sketch morphism |

pub mod context;
pub mod mapping;
pub mod sketch;
pub mod validation;

pub use context::{BoundedContext, Invariant};
pub use mapping::{
    check_functorial_consistency, ContextMap, FunctorCheckResult, FunctorError, MorphismMapping,
    ObjectMapping, RelationshipPattern,
};
pub use sketch::Sketch;
pub use validation::{ValidationError, ValidationResult};
