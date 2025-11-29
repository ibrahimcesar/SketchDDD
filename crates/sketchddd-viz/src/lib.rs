//! # SketchDDD Visualization
//!
//! Generate diagrams and visualizations from SketchDDD domain models.
//!
//! ## Supported Formats
//!
//! - **Graphviz DOT**: For rendering with Graphviz
//! - **Mermaid**: For rendering in Markdown/GitHub

pub mod graphviz;
pub mod mermaid;

use sketchddd_core::BoundedContext;
use thiserror::Error;

/// Error during visualization generation.
#[derive(Debug, Error)]
pub enum VizError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Invalid model: {0}")]
    InvalidModel(String),
}

/// Output format for visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Graphviz,
    Mermaid,
}

impl std::str::FromStr for Format {
    type Err = VizError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "graphviz" | "dot" => Ok(Format::Graphviz),
            "mermaid" | "md" => Ok(Format::Mermaid),
            _ => Err(VizError::UnsupportedFormat(s.to_string())),
        }
    }
}

/// Generate visualization from a bounded context.
pub fn generate(context: &BoundedContext, format: Format) -> Result<String, VizError> {
    match format {
        Format::Graphviz => graphviz::generate(context),
        Format::Mermaid => mermaid::generate(context),
    }
}
