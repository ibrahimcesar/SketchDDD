//! Document management and analysis

use ropey::Rope;
use tower_lsp::lsp_types::*;

use sketchddd_parser::parse;

/// A document being edited
pub struct Document {
    /// Document URI
    pub uri: Url,
    /// Document content as a rope for efficient editing
    pub content: Rope,
    /// Document version
    pub version: i32,
    /// Parsed result (if successful) - reserved for future semantic analysis
    #[allow(dead_code)]
    pub parse_result: Option<Vec<sketchddd_parser::ast::ContextDecl>>,
    /// Type definitions found in the document
    pub definitions: Vec<Definition>,
}

/// A type definition in the document
#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub kind: DefinitionKind,
    pub range: Range,
    pub selection_range: Range,
    pub children: Vec<Definition>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionKind {
    Context,
    Entity,
    Value,
    Enum,
    EnumVariant,
    Aggregate,
    Morphism,
    Field,
    ContextMap,
}

impl Document {
    /// Create a new document
    pub fn new(uri: Url, text: String, version: i32) -> Self {
        let content = Rope::from_str(&text);
        let parse_result = parse(&text).ok();
        let definitions = Self::extract_definitions(&text);

        Self {
            uri,
            content,
            version,
            parse_result,
            definitions,
        }
    }

    /// Get the document text
    pub fn text(&self) -> String {
        self.content.to_string()
    }

    /// Get a line from the document
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.content.len_lines() {
            Some(self.content.line(line_idx).to_string())
        } else {
            None
        }
    }

    /// Get the word at a position
    pub fn word_at(&self, position: Position) -> Option<String> {
        let line = self.line(position.line as usize)?;
        let col = position.character as usize;

        if col >= line.len() {
            return None;
        }

        // Find word boundaries
        let chars: Vec<char> = line.chars().collect();
        let mut start = col;
        let mut end = col;

        while start > 0 && is_word_char(chars[start - 1]) {
            start -= 1;
        }

        while end < chars.len() && is_word_char(chars[end]) {
            end += 1;
        }

        if start == end {
            return None;
        }

        Some(chars[start..end].iter().collect())
    }

    /// Find the definition of a symbol at a position
    pub fn find_definition(&self, position: Position) -> Option<Location> {
        let word = self.word_at(position)?;

        // Search for a definition with this name
        for def in &self.definitions {
            if def.name == word {
                return Some(Location {
                    uri: self.uri.clone(),
                    range: def.selection_range,
                });
            }

            // Check children
            for child in &def.children {
                if child.name == word {
                    return Some(Location {
                        uri: self.uri.clone(),
                        range: child.selection_range,
                    });
                }
            }
        }

        None
    }

    /// Find all references to a symbol at a position
    pub fn find_references(&self, position: Position) -> Vec<Location> {
        let mut refs = Vec::new();

        if let Some(word) = self.word_at(position) {
            let text = self.text();

            // Simple text search for the word
            for (line_idx, line) in text.lines().enumerate() {
                let mut col = 0;
                while let Some(pos) = line[col..].find(&word) {
                    let start_col = col + pos;
                    let end_col = start_col + word.len();

                    // Check it's a whole word
                    let before_ok =
                        start_col == 0 || !is_word_char(line.chars().nth(start_col - 1).unwrap());
                    let after_ok = end_col >= line.len()
                        || !is_word_char(line.chars().nth(end_col).unwrap());

                    if before_ok && after_ok {
                        refs.push(Location {
                            uri: self.uri.clone(),
                            range: Range {
                                start: Position::new(line_idx as u32, start_col as u32),
                                end: Position::new(line_idx as u32, end_col as u32),
                            },
                        });
                    }

                    col = end_col;
                }
            }
        }

        refs
    }

    /// Format the document
    pub fn format(&self) -> Option<String> {
        // For now, just return the parsed and re-serialized version
        // A proper formatter would preserve comments and whitespace choices
        let text = self.text();

        // Basic formatting: ensure consistent indentation
        let mut result = String::new();
        let mut indent: i32 = 0;

        for line in text.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                result.push('\n');
                continue;
            }

            // Decrease indent before closing brace
            if trimmed.starts_with('}') {
                indent = indent.saturating_sub(1);
            }

            // Add indentation
            for _ in 0..indent {
                result.push_str("  ");
            }
            result.push_str(trimmed);
            result.push('\n');

            // Increase indent after opening brace
            if trimmed.ends_with('{') {
                indent += 1;
            }
        }

        Some(result)
    }

    /// Extract definitions from source text
    fn extract_definitions(text: &str) -> Vec<Definition> {
        let mut definitions = Vec::new();
        let mut current_context: Option<Definition> = None;

        for (line_idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            let line_num = line_idx as u32;

            // Context definition
            if let Some(name) = extract_name(trimmed, "context") {
                // Save previous context
                if let Some(ctx) = current_context.take() {
                    definitions.push(ctx);
                }

                let col = line.find("context").unwrap_or(0) as u32;
                let name_col = line.find(&name).unwrap_or(0) as u32;

                current_context = Some(Definition {
                    name: name.clone(),
                    kind: DefinitionKind::Context,
                    range: Range {
                        start: Position::new(line_num, col),
                        end: Position::new(line_num, (line.len()) as u32),
                    },
                    selection_range: Range {
                        start: Position::new(line_num, name_col),
                        end: Position::new(line_num, name_col + name.len() as u32),
                    },
                    children: Vec::new(),
                });
            }
            // Entity definition
            else if let Some(name) = extract_name(trimmed, "entity") {
                let name_col = line.find(&name).unwrap_or(0) as u32;
                let def = Definition {
                    name,
                    kind: DefinitionKind::Entity,
                    range: Range {
                        start: Position::new(line_num, 0),
                        end: Position::new(line_num, line.len() as u32),
                    },
                    selection_range: Range {
                        start: Position::new(line_num, name_col),
                        end: Position::new(line_num, name_col + name_col),
                    },
                    children: Vec::new(),
                };

                if let Some(ctx) = current_context.as_mut() {
                    ctx.children.push(def);
                } else {
                    definitions.push(def);
                }
            }
            // Value definition
            else if let Some(name) = extract_name(trimmed, "value") {
                let name_col = line.find(&name).unwrap_or(0) as u32;
                let def = Definition {
                    name,
                    kind: DefinitionKind::Value,
                    range: Range {
                        start: Position::new(line_num, 0),
                        end: Position::new(line_num, line.len() as u32),
                    },
                    selection_range: Range {
                        start: Position::new(line_num, name_col),
                        end: Position::new(line_num, name_col + name_col),
                    },
                    children: Vec::new(),
                };

                if let Some(ctx) = current_context.as_mut() {
                    ctx.children.push(def);
                } else {
                    definitions.push(def);
                }
            }
            // Enum definition
            else if let Some(name) = extract_name(trimmed, "enum") {
                let name_col = line.find(&name).unwrap_or(0) as u32;
                let def = Definition {
                    name,
                    kind: DefinitionKind::Enum,
                    range: Range {
                        start: Position::new(line_num, 0),
                        end: Position::new(line_num, line.len() as u32),
                    },
                    selection_range: Range {
                        start: Position::new(line_num, name_col),
                        end: Position::new(line_num, name_col + name_col),
                    },
                    children: Vec::new(),
                };

                if let Some(ctx) = current_context.as_mut() {
                    ctx.children.push(def);
                } else {
                    definitions.push(def);
                }
            }
            // Aggregate definition
            else if let Some(name) = extract_name(trimmed, "aggregate") {
                let name_col = line.find(&name).unwrap_or(0) as u32;
                let def = Definition {
                    name,
                    kind: DefinitionKind::Aggregate,
                    range: Range {
                        start: Position::new(line_num, 0),
                        end: Position::new(line_num, line.len() as u32),
                    },
                    selection_range: Range {
                        start: Position::new(line_num, name_col),
                        end: Position::new(line_num, name_col + name_col),
                    },
                    children: Vec::new(),
                };

                if let Some(ctx) = current_context.as_mut() {
                    ctx.children.push(def);
                } else {
                    definitions.push(def);
                }
            }
            // Context map definition
            else if let Some(name) = extract_map_name(trimmed) {
                let name_col = line.find(&name).unwrap_or(0) as u32;
                definitions.push(Definition {
                    name,
                    kind: DefinitionKind::ContextMap,
                    range: Range {
                        start: Position::new(line_num, 0),
                        end: Position::new(line_num, line.len() as u32),
                    },
                    selection_range: Range {
                        start: Position::new(line_num, name_col),
                        end: Position::new(line_num, name_col + name_col),
                    },
                    children: Vec::new(),
                });
            }
        }

        // Don't forget the last context
        if let Some(ctx) = current_context {
            definitions.push(ctx);
        }

        definitions
    }
}

/// Check if a character is a word character
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Extract a name after a keyword
fn extract_name(line: &str, keyword: &str) -> Option<String> {
    if !line.starts_with(keyword) {
        return None;
    }

    let rest = line[keyword.len()..].trim_start();

    // Extract identifier
    let name: String = rest.chars().take_while(|c| is_word_char(*c)).collect();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

/// Extract a context map name
fn extract_map_name(line: &str) -> Option<String> {
    if !line.starts_with("map") {
        return None;
    }

    let rest = line[3..].trim_start();
    let name: String = rest.chars().take_while(|c| is_word_char(*c)).collect();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}
