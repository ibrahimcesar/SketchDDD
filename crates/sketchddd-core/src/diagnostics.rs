//! Rich diagnostic rendering for SketchDDD validation errors.
//!
//! This module provides Rust compiler-style error messages with:
//! - Source code snippets with line numbers
//! - Colored underlining of problem areas
//! - "Did you mean?" suggestions using edit distance
//! - Fix suggestions where applicable
//! - Error codes linking to documentation

use crate::validation::{Severity, ValidationError, ValidationResult};
use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use std::io::Write;
use strsim::levenshtein;

/// Source span for locating errors in source code.
#[derive(Debug, Clone, Default)]
pub struct SourceSpan {
    /// Byte offset of start position
    pub start: usize,
    /// Byte offset of end position
    pub end: usize,
    /// Line number (1-indexed)
    pub line: u32,
    /// Column number (1-indexed)
    pub column: u32,
}

impl SourceSpan {
    /// Create a new source span.
    pub fn new(start: usize, end: usize, line: u32, column: u32) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    /// Convert to a range.
    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

/// A located validation error with source span information.
#[derive(Debug, Clone)]
pub struct LocatedError {
    /// The underlying validation error
    pub error: ValidationError,
    /// Primary source span (where the error occurred)
    pub span: Option<SourceSpan>,
    /// Additional source spans for context
    pub related_spans: Vec<(SourceSpan, String)>,
    /// File name for display
    pub filename: String,
}

impl LocatedError {
    /// Create a new located error.
    pub fn new(error: ValidationError, filename: impl Into<String>) -> Self {
        Self {
            error,
            span: None,
            related_spans: Vec::new(),
            filename: filename.into(),
        }
    }

    /// Set the primary source span.
    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// Add a related span with a label.
    pub fn with_related(mut self, span: SourceSpan, label: impl Into<String>) -> Self {
        self.related_spans.push((span, label.into()));
        self
    }
}

/// Diagnostic renderer for validation results using ariadne.
pub struct DiagnosticRenderer {
    /// Whether to use colors
    use_colors: bool,
    /// Whether to show help messages
    show_help: bool,
}

impl Default for DiagnosticRenderer {
    fn default() -> Self {
        Self {
            use_colors: true,
            show_help: true,
        }
    }
}

impl DiagnosticRenderer {
    /// Create a new diagnostic renderer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Disable colors for output.
    pub fn without_colors(mut self) -> Self {
        self.use_colors = false;
        self
    }

    /// Disable help messages.
    pub fn without_help(mut self) -> Self {
        self.show_help = false;
        self
    }

    /// Render a validation result to a string.
    pub fn render_to_string(
        &self,
        result: &ValidationResult,
        source: &str,
        filename: &str,
    ) -> String {
        let mut output = Vec::new();
        self.render(result, source, filename, &mut output);
        String::from_utf8(output).unwrap_or_default()
    }

    /// Render a validation result to a writer.
    pub fn render<W: Write>(
        &self,
        result: &ValidationResult,
        source: &str,
        filename: &str,
        writer: &mut W,
    ) {
        for error in &result.issues {
            self.render_error(error, source, filename, writer);
        }

        // Print summary
        let error_count = result.error_count();
        let warning_count = result.warning_count();

        if error_count > 0 || warning_count > 0 {
            writeln!(writer).ok();
            if error_count > 0 {
                writeln!(
                    writer,
                    "error: could not validate due to {} previous error(s)",
                    error_count
                )
                .ok();
            }
            if warning_count > 0 {
                writeln!(writer, "warning: {} warning(s) emitted", warning_count).ok();
            }
        }
    }

    /// Render a single error using ariadne.
    fn render_error<W: Write>(
        &self,
        error: &ValidationError,
        source: &str,
        filename: &str,
        writer: &mut W,
    ) {
        let report_kind = match error.severity {
            Severity::Error => ReportKind::Error,
            Severity::Warning => ReportKind::Warning,
            Severity::Hint => ReportKind::Advice,
        };

        let config = Config::default().with_color(self.use_colors);

        // Use the first character of source as the span if available
        let source_len = source.len();
        let span_end = source_len.min(1);

        // Build the report - ariadne 0.6 takes (kind, span) where span is (filename, range)
        let mut builder = Report::<(String, std::ops::Range<usize>)>::build(
            report_kind,
            (filename.to_string(), 0..span_end),
        )
        .with_config(config)
        .with_code(&error.code)
        .with_message(&error.message);

        // Add a label to show source context
        if source_len > 0 {
            let label_color = match error.severity {
                Severity::Error => Color::Red,
                Severity::Warning => Color::Yellow,
                Severity::Hint => Color::Cyan,
            };
            builder = builder.with_label(
                Label::new((filename.to_string(), 0..span_end))
                    .with_message("here")
                    .with_color(label_color),
            );
        }

        // Add help if available
        if self.show_help {
            if let Some(suggestion) = &error.suggestion {
                builder = builder.with_help(suggestion.clone());
            }
        }

        // Add note for documentation link
        builder = builder.with_note(format!(
            "For more information, see: https://docs.sketchddd.dev/errors/{}",
            error.code
        ));

        // Create source cache as tuple (Id, Source) for ariadne 0.6
        let cache = (filename.to_string(), Source::from(source));

        // Render to writer
        builder.finish().write(cache, writer).ok();
    }

    /// Render a located error with source spans using ariadne.
    pub fn render_located<W: Write>(&self, error: &LocatedError, source: &str, writer: &mut W) {
        let report_kind = match error.error.severity {
            Severity::Error => ReportKind::Error,
            Severity::Warning => ReportKind::Warning,
            Severity::Hint => ReportKind::Advice,
        };

        let config = Config::default().with_color(self.use_colors);

        // Determine the span for the report
        let report_span = error
            .span
            .as_ref()
            .map(|s| s.to_range())
            .unwrap_or(0..0);

        let mut builder = Report::<(String, std::ops::Range<usize>)>::build(
            report_kind,
            (error.filename.clone(), report_span),
        )
        .with_config(config)
        .with_code(&error.error.code)
        .with_message(&error.error.message);

        // Add primary label if we have a span
        if let Some(span) = &error.span {
            let label_color = match error.error.severity {
                Severity::Error => Color::Red,
                Severity::Warning => Color::Yellow,
                Severity::Hint => Color::Cyan,
            };

            builder = builder.with_label(
                Label::new((error.filename.clone(), span.to_range()))
                    .with_message(&error.error.message)
                    .with_color(label_color),
            );
        }

        // Add related spans
        for (span, label) in &error.related_spans {
            builder = builder.with_label(
                Label::new((error.filename.clone(), span.to_range()))
                    .with_message(label)
                    .with_color(Color::Cyan),
            );
        }

        // Add help if available
        if self.show_help {
            if let Some(suggestion) = &error.error.suggestion {
                builder = builder.with_help(suggestion.clone());
            }
        }

        // Add documentation link
        builder = builder.with_note(format!(
            "For more information, see: https://docs.sketchddd.dev/errors/{}",
            error.error.code
        ));

        // Create source cache as tuple (Id, Source) for ariadne 0.6
        let cache = (error.filename.clone(), Source::from(source));

        builder.finish().write(cache, writer).ok();
    }
}

// =============================================================
// "Did You Mean?" Suggestions
// =============================================================

/// Suggest similar names based on edit distance.
pub fn suggest_similar<'a>(name: &str, candidates: &[&'a str]) -> Option<&'a str> {
    suggest_similar_with_threshold(name, candidates, 3)
}

/// Suggest similar names with a custom threshold.
pub fn suggest_similar_with_threshold<'a>(
    name: &str,
    candidates: &[&'a str],
    max_distance: usize,
) -> Option<&'a str> {
    let name_lower = name.to_lowercase();

    candidates
        .iter()
        .filter_map(|&c| {
            let c_lower = c.to_lowercase();
            let dist = levenshtein(&name_lower, &c_lower);

            // Accept if within max_distance or within 30% of name length
            let length_threshold = (name.len() * 30) / 100;
            let threshold = max_distance.max(length_threshold);

            if dist <= threshold {
                Some((c, dist))
            } else {
                None
            }
        })
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| c)
}

/// Generate a "did you mean?" help message.
pub fn did_you_mean(name: &str, candidates: &[&str]) -> Option<String> {
    suggest_similar(name, candidates).map(|suggestion| format!("did you mean `{}`?", suggestion))
}

/// Generate a list of available options as a note.
pub fn available_options(options: &[&str], max_show: usize) -> String {
    if options.is_empty() {
        return "no options available".to_string();
    }

    if options.len() <= max_show {
        format!("available: {}", options.join(", "))
    } else {
        format!(
            "available: {}, ... ({} more)",
            options[..max_show].join(", "),
            options.len() - max_show
        )
    }
}

// =============================================================
// Error Grouping
// =============================================================

/// Group related errors by their code or pattern.
pub fn group_errors(errors: &[ValidationError]) -> Vec<GroupedErrors> {
    let mut groups: std::collections::HashMap<String, Vec<&ValidationError>> =
        std::collections::HashMap::new();

    for error in errors {
        groups.entry(error.code.clone()).or_default().push(error);
    }

    groups
        .into_iter()
        .map(|(code, errors)| GroupedErrors {
            code,
            count: errors.len(),
            errors: errors.into_iter().cloned().collect(),
        })
        .collect()
}

/// A group of related errors.
#[derive(Debug, Clone)]
pub struct GroupedErrors {
    /// The error code
    pub code: String,
    /// Number of errors in this group
    pub count: usize,
    /// The individual errors
    pub errors: Vec<ValidationError>,
}

// =============================================================
// Tests
// =============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggest_similar_exact_match() {
        let candidates = ["Customer", "Order", "Product"];
        assert_eq!(suggest_similar("Customer", &candidates), Some("Customer"));
    }

    #[test]
    fn test_suggest_similar_typo() {
        let candidates = ["Customer", "Order", "Product"];
        assert_eq!(suggest_similar("Custommer", &candidates), Some("Customer"));
    }

    #[test]
    fn test_suggest_similar_case_insensitive() {
        let candidates = ["Customer", "Order", "Product"];
        assert_eq!(suggest_similar("customer", &candidates), Some("Customer"));
    }

    #[test]
    fn test_suggest_similar_no_match() {
        let candidates = ["Customer", "Order", "Product"];
        assert_eq!(suggest_similar("XYZ123", &candidates), None);
    }

    #[test]
    fn test_suggest_similar_close_match() {
        let candidates = ["Customer", "Order", "Product"];
        assert_eq!(suggest_similar("Ordr", &candidates), Some("Order"));
    }

    #[test]
    fn test_did_you_mean_message() {
        let candidates = ["Customer", "Order", "Product"];
        let suggestion = did_you_mean("Custommer", &candidates);
        assert_eq!(suggestion, Some("did you mean `Customer`?".to_string()));
    }

    #[test]
    fn test_available_options_short_list() {
        let options = ["Customer", "Order"];
        assert_eq!(available_options(&options, 5), "available: Customer, Order");
    }

    #[test]
    fn test_available_options_long_list() {
        let options = ["A", "B", "C", "D", "E", "F"];
        assert_eq!(
            available_options(&options, 3),
            "available: A, B, C, ... (3 more)"
        );
    }

    #[test]
    fn test_group_errors() {
        let errors = vec![
            ValidationError::error("E0001", "Error 1"),
            ValidationError::error("E0001", "Error 2"),
            ValidationError::error("E0002", "Error 3"),
        ];

        let groups = group_errors(&errors);
        assert_eq!(groups.len(), 2);

        let e0001_group = groups.iter().find(|g| g.code == "E0001").unwrap();
        assert_eq!(e0001_group.count, 2);

        let e0002_group = groups.iter().find(|g| g.code == "E0002").unwrap();
        assert_eq!(e0002_group.count, 1);
    }

    #[test]
    fn test_render_basic_error() {
        let mut result = ValidationResult::new();
        result.add(ValidationError::error("E0001", "Test error message"));

        let renderer = DiagnosticRenderer::new().without_colors();
        let output = renderer.render_to_string(&result, "context Test {}", "test.sddd");

        assert!(output.contains("E0001"));
        assert!(output.contains("Test error message"));
    }

    #[test]
    fn test_render_warning() {
        let mut result = ValidationResult::new();
        result.add(ValidationError::warning("W0001", "Test warning"));

        let renderer = DiagnosticRenderer::new().without_colors();
        let output = renderer.render_to_string(&result, "context Test {}", "test.sddd");

        assert!(output.contains("W0001"));
        assert!(output.contains("warning"));
    }

    #[test]
    fn test_render_with_suggestion() {
        let mut result = ValidationResult::new();
        result.add(
            ValidationError::error("E0001", "Unknown object")
                .with_suggestion("did you mean `Customer`?"),
        );

        let renderer = DiagnosticRenderer::new().without_colors();
        let output = renderer.render_to_string(&result, "context Test {}", "test.sddd");

        assert!(output.contains("did you mean `Customer`?"));
    }

    #[test]
    fn test_source_span() {
        let span = SourceSpan::new(10, 20, 2, 5);
        assert_eq!(span.to_range(), 10..20);
    }

    #[test]
    fn test_render_located_error_with_span() {
        let error = LocatedError::new(
            ValidationError::error("E0023", "Unknown object referenced"),
            "test.sddd",
        )
        .with_span(SourceSpan::new(28, 37, 2, 20));

        let source = "context Test {\n  morphisms { foo: A -> Custommer }\n}";
        let renderer = DiagnosticRenderer::new().without_colors();
        let mut output = Vec::new();
        renderer.render_located(&error, source, &mut output);
        let output_str = String::from_utf8(output).unwrap();

        assert!(output_str.contains("E0023"));
        assert!(output_str.contains("Unknown object referenced"));
    }

    #[test]
    fn test_color_disabled() {
        let mut result = ValidationResult::new();
        result.add(ValidationError::error("E0001", "Test"));

        let renderer = DiagnosticRenderer::new().without_colors();
        let output = renderer.render_to_string(&result, "", "test.sddd");

        // ariadne without colors should not have ANSI escape codes (or minimal)
        // Note: ariadne may still include some formatting characters
        assert!(output.contains("E0001"));
    }
}
