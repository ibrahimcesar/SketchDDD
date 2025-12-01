//! # SketchDDD Code Generation
//!
//! Generate code in various languages from SketchDDD domain models.
//!
//! ## Supported Targets
//!
//! - **Rust**: Structs, enums, and validation
//! - **TypeScript**: Interfaces, types, and Zod schemas
//! - **Kotlin**: Data classes and sealed classes
//!
//! ## Example
//!
//! ```rust
//! use sketchddd_core::BoundedContext;
//! use sketchddd_codegen::{generate, Target};
//!
//! let mut context = BoundedContext::new("Commerce");
//! context.add_entity("Customer");
//! context.add_entity("Order");
//!
//! let rust_code = generate(&context, Target::Rust).unwrap();
//! println!("{}", rust_code);
//! ```

pub mod kotlin;
pub mod rust;
pub mod typescript;

pub use rust::RustConfig;

use sketchddd_core::BoundedContext;
use thiserror::Error;

/// Error during code generation.
#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Unsupported target: {0}")]
    UnsupportedTarget(String),

    #[error("Invalid model: {0}")]
    InvalidModel(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Target language for code generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Rust,
    TypeScript,
    Kotlin,
}

impl std::str::FromStr for Target {
    type Err = CodegenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Target::Rust),
            "typescript" | "ts" => Ok(Target::TypeScript),
            "kotlin" | "kt" => Ok(Target::Kotlin),
            _ => Err(CodegenError::UnsupportedTarget(s.to_string())),
        }
    }
}

/// Generate code from a bounded context.
pub fn generate(context: &BoundedContext, target: Target) -> Result<String, CodegenError> {
    match target {
        Target::Rust => rust::generate(context),
        Target::TypeScript => typescript::generate(context),
        Target::Kotlin => kotlin::generate(context),
    }
}
