//! Validation logic for sketches and bounded contexts.

use crate::sketch::{ObjectId, Sketch};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Location in source code for error reporting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceLocation {
    /// File path
    pub file: Option<String>,
    /// Line number (1-indexed)
    pub line: Option<u32>,
    /// Column number (1-indexed)
    pub column: Option<u32>,
}

impl SourceLocation {
    /// Create a new source location.
    pub fn new(file: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            file: Some(file.into()),
            line: Some(line),
            column: Some(column),
        }
    }
}

/// The severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Error: Must be fixed
    Error,
    /// Warning: Should be reviewed
    Warning,
    /// Hint: Suggestion for improvement
    Hint,
}

/// A validation error or warning.
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("{message}")]
pub struct ValidationError {
    /// Error code (e.g., "E0001")
    pub code: String,

    /// Human-readable message
    pub message: String,

    /// Severity level
    pub severity: Severity,

    /// Location in source
    pub location: SourceLocation,

    /// Suggested fix
    pub suggestion: Option<String>,
}

impl ValidationError {
    /// Create a new error.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            severity: Severity::Error,
            location: SourceLocation::default(),
            suggestion: None,
        }
    }

    /// Create a new warning.
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            severity: Severity::Warning,
            location: SourceLocation::default(),
            suggestion: None,
        }
    }

    /// Add a location to this error.
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = location;
        self
    }

    /// Add a suggestion to this error.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

/// Result of validating a sketch.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationResult {
    /// List of errors and warnings
    pub issues: Vec<ValidationError>,
}

impl ValidationResult {
    /// Create a new empty result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an issue.
    pub fn add(&mut self, error: ValidationError) {
        self.issues.push(error);
    }

    /// Check if validation passed (no errors).
    pub fn is_ok(&self) -> bool {
        !self.issues.iter().any(|e| e.severity == Severity::Error)
    }

    /// Check if there are any issues.
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Get only errors.
    pub fn errors(&self) -> impl Iterator<Item = &ValidationError> {
        self.issues
            .iter()
            .filter(|e| e.severity == Severity::Error)
    }

    /// Get only warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationError> {
        self.issues
            .iter()
            .filter(|e| e.severity == Severity::Warning)
    }

    /// Count errors.
    pub fn error_count(&self) -> usize {
        self.errors().count()
    }

    /// Count warnings.
    pub fn warning_count(&self) -> usize {
        self.warnings().count()
    }
}

/// Validate a sketch for basic consistency.
pub fn validate_sketch(sketch: &Sketch) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Check that morphism sources and targets exist
    for morphism in sketch.graph.morphisms() {
        if sketch.graph.get_object(morphism.source).is_none() {
            result.add(ValidationError::error(
                "E0001",
                format!(
                    "Morphism '{}' references non-existent source object",
                    morphism.name
                ),
            ));
        }
        if sketch.graph.get_object(morphism.target).is_none() {
            result.add(ValidationError::error(
                "E0002",
                format!(
                    "Morphism '{}' references non-existent target object",
                    morphism.name
                ),
            ));
        }
    }

    // Check that equations are well-formed
    for equation in &sketch.equations {
        if !equation.is_well_formed() {
            result.add(ValidationError::error(
                "E0010",
                format!(
                    "Equation '{}' is not well-formed: paths have different sources or targets",
                    equation.name
                ),
            ));
        }
    }

    // Check for duplicate object names
    let mut seen_names: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for object in sketch.graph.objects() {
        if !seen_names.insert(&object.name) {
            result.add(ValidationError::error(
                "E0020",
                format!("Duplicate object name: '{}'", object.name),
            ));
        }
    }

    // Warn about potentially large aggregates
    for limit in &sketch.limits {
        if limit.is_aggregate && limit.projections.len() > 5 {
            result.add(
                ValidationError::warning(
                    "W0001",
                    format!(
                        "Aggregate '{}' contains {} objects, which may be too large",
                        limit.name,
                        limit.projections.len()
                    ),
                )
                .with_suggestion("Consider splitting into smaller aggregates"),
            );
        }
    }

    result
}

/// Validate that an object exists in a sketch.
pub fn object_exists(sketch: &Sketch, id: ObjectId) -> bool {
    sketch.graph.get_object(id).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_sketch_is_valid() {
        let sketch = Sketch::new("Test");
        let result = validate_sketch(&sketch);
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_names_detected() {
        let mut sketch = Sketch::new("Test");
        sketch.add_object("Customer");
        sketch.add_object("Customer"); // Duplicate!

        let result = validate_sketch(&sketch);
        assert!(!result.is_ok());
        assert_eq!(result.error_count(), 1);
    }

    #[test]
    fn test_validation_error_builder() {
        let err = ValidationError::error("E0001", "Test error")
            .with_location(SourceLocation::new("test.sketch", 10, 5))
            .with_suggestion("Try this instead");

        assert_eq!(err.code, "E0001");
        assert_eq!(err.location.line, Some(10));
        assert!(err.suggestion.is_some());
    }
}
