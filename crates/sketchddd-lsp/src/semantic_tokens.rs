//! Semantic tokens for enhanced highlighting
//!
//! This module provides semantic token generation for more accurate
//! syntax highlighting than TextMate grammars can provide.

// Semantic token types and encoding for LSP
// These are prepared for future semantic highlighting support

/// Token types for semantic highlighting
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Namespace = 0,   // context
    Class = 1,       // entity, value
    Enum = 2,        // enum
    EnumMember = 3,  // enum variant
    Property = 4,    // field
    Function = 5,    // morphism
    Type = 6,        // type reference
    Keyword = 7,     // keywords
    Comment = 8,     // comments
    String = 9,      // strings
    Number = 10,     // numbers
    Operator = 11,   // operators
    Decorator = 12,  // annotations
}

/// Token modifiers
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenModifier {
    Declaration = 0,
    Definition = 1,
    Readonly = 2,
}

/// A semantic token
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SemanticToken {
    pub line: u32,
    pub start_char: u32,
    pub length: u32,
    pub token_type: TokenType,
    pub modifiers: u32,
}

/// Generate semantic tokens for a document
#[allow(dead_code)]
pub fn generate_semantic_tokens(text: &str) -> Vec<SemanticToken> {
    let mut tokens = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let line_num = line_idx as u32;
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Comments
        if trimmed.starts_with("//") {
            if let Some(pos) = line.find("//") {
                tokens.push(SemanticToken {
                    line: line_num,
                    start_char: pos as u32,
                    length: (line.len() - pos) as u32,
                    token_type: TokenType::Comment,
                    modifiers: 0,
                });
            }
            continue;
        }

        // Keywords
        for keyword in &[
            "context",
            "entity",
            "value",
            "enum",
            "aggregate",
            "morphisms",
            "map",
            "pattern",
            "mappings",
            "root",
            "contains",
            "invariant",
        ] {
            if let Some(pos) = find_word(line, keyword) {
                let token_type = match *keyword {
                    "context" => TokenType::Keyword,
                    "entity" | "value" => TokenType::Keyword,
                    "enum" => TokenType::Keyword,
                    "aggregate" => TokenType::Keyword,
                    _ => TokenType::Keyword,
                };

                tokens.push(SemanticToken {
                    line: line_num,
                    start_char: pos as u32,
                    length: keyword.len() as u32,
                    token_type,
                    modifiers: 0,
                });
            }
        }

        // Type declarations (PascalCase after keyword)
        if trimmed.starts_with("context")
            || trimmed.starts_with("entity")
            || trimmed.starts_with("value")
            || trimmed.starts_with("enum")
            || trimmed.starts_with("aggregate")
        {
            // Find the type name
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let type_name = parts[1].trim_end_matches('{').trim_end_matches('=');
                if let Some(pos) = line.find(type_name) {
                    let token_type = match parts[0] {
                        "context" => TokenType::Namespace,
                        "entity" | "value" | "aggregate" => TokenType::Class,
                        "enum" => TokenType::Enum,
                        _ => TokenType::Type,
                    };

                    tokens.push(SemanticToken {
                        line: line_num,
                        start_char: pos as u32,
                        length: type_name.len() as u32,
                        token_type,
                        modifiers: (1 << TokenModifier::Declaration as u32)
                            | (1 << TokenModifier::Definition as u32),
                    });
                }
            }
        }

        // Operators
        for op in &["->", "=>", "=", "|"] {
            let mut search_pos = 0;
            while let Some(rel_pos) = line[search_pos..].find(op) {
                let pos = search_pos + rel_pos;
                tokens.push(SemanticToken {
                    line: line_num,
                    start_char: pos as u32,
                    length: op.len() as u32,
                    token_type: TokenType::Operator,
                    modifiers: 0,
                });
                search_pos = pos + op.len();
            }
        }

        // Field names (before colon)
        if trimmed.contains(':') && !trimmed.starts_with("//") {
            if let Some(colon_pos) = line.find(':') {
                let before_colon = &line[..colon_pos];
                let field_name = before_colon.trim();

                if !field_name.is_empty()
                    && field_name.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
                {
                    let start = line.len() - line.trim_start().len();
                    tokens.push(SemanticToken {
                        line: line_num,
                        start_char: start as u32,
                        length: field_name.len() as u32,
                        token_type: TokenType::Property,
                        modifiers: 0,
                    });
                }
            }
        }

        // Annotations (@something)
        let mut search_pos = 0;
        while let Some(rel_pos) = line[search_pos..].find('@') {
            let pos = search_pos + rel_pos;
            let rest = &line[pos + 1..];
            let annotation: String = rest.chars().take_while(|c| c.is_alphanumeric()).collect();

            if !annotation.is_empty() {
                tokens.push(SemanticToken {
                    line: line_num,
                    start_char: pos as u32,
                    length: (annotation.len() + 1) as u32,
                    token_type: TokenType::Decorator,
                    modifiers: 0,
                });
            }

            search_pos = pos + 1;
        }
    }

    // Sort tokens by position
    tokens.sort_by(|a, b| {
        a.line
            .cmp(&b.line)
            .then_with(|| a.start_char.cmp(&b.start_char))
    });

    tokens
}

/// Find a word in a line (whole word match)
#[allow(dead_code)]
fn find_word(line: &str, word: &str) -> Option<usize> {
    let mut pos = 0;
    while let Some(rel_pos) = line[pos..].find(word) {
        let abs_pos = pos + rel_pos;
        let before_ok =
            abs_pos == 0 || !line.chars().nth(abs_pos - 1).map(|c| c.is_alphanumeric()).unwrap_or(false);
        let after_ok = abs_pos + word.len() >= line.len()
            || !line
                .chars()
                .nth(abs_pos + word.len())
                .map(|c| c.is_alphanumeric())
                .unwrap_or(false);

        if before_ok && after_ok {
            return Some(abs_pos);
        }

        pos = abs_pos + 1;
    }
    None
}

/// Encode tokens for LSP response
#[allow(dead_code)]
pub fn encode_tokens(tokens: &[SemanticToken]) -> Vec<u32> {
    let mut result = Vec::with_capacity(tokens.len() * 5);
    let mut prev_line = 0u32;
    let mut prev_char = 0u32;

    for token in tokens {
        let delta_line = token.line - prev_line;
        let delta_char = if delta_line == 0 {
            token.start_char - prev_char
        } else {
            token.start_char
        };

        result.push(delta_line);
        result.push(delta_char);
        result.push(token.length);
        result.push(token.token_type as u32);
        result.push(token.modifiers);

        prev_line = token.line;
        prev_char = token.start_char;
    }

    result
}
