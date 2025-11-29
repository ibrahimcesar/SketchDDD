//! # SketchDDD
//!
//! A Categorical Framework for Domain-Driven Design.
//!
//! SketchDDD bridges the gap between domain experts and developers by providing
//! precise mathematical definitions for DDD concepts using category theory.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use sketchddd::prelude::*;
//!
//! // Create a bounded context
//! let mut ctx = BoundedContext::new("Commerce");
//!
//! // Add entities
//! let customer = ctx.add_entity("Customer");
//! let order = ctx.add_entity("Order");
//!
//! // Add value objects
//! let money = ctx.add_value_object("Money");
//!
//! // Add enumerations
//! let status = ctx.add_enum("OrderStatus", vec![
//!     "Pending".into(),
//!     "Confirmed".into(),
//!     "Shipped".into(),
//! ]);
//! ```
//!
//! ## Crate Features
//!
//! This crate re-exports functionality from:
//! - [`sketchddd_core`] - Core data structures and categorical semantics
//! - [`sketchddd_parser`] - DSL parser
//! - [`sketchddd_codegen`] - Code generation (Rust, TypeScript, Kotlin)
//! - [`sketchddd_viz`] - Visualization (Graphviz, Mermaid)

pub use sketchddd_core as core;
pub use sketchddd_parser as parser;
pub use sketchddd_codegen as codegen;
pub use sketchddd_viz as viz;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use sketchddd_core::{
        BoundedContext,
        ContextMap,
        Sketch,
        ValidationError,
        ValidationResult,
    };
    pub use sketchddd_codegen::{generate, Target};
    pub use sketchddd_viz::{generate as generate_viz, Format};
}
