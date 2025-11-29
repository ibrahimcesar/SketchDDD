//! Parser error types.

use thiserror::Error;

/// Error that occurs during parsing.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct ParseError {
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    pub fn with_location(mut self, line: u32, column: u32) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}
