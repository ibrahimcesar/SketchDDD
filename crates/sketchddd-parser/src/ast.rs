//! Abstract Syntax Tree for SketchDDD DSL.

use serde::{Deserialize, Serialize};

/// Source location for error reporting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub column: u32,
}

/// A context declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDecl {
    pub name: String,
    pub objects: Vec<ObjectDecl>,
    pub morphisms: Vec<MorphismDecl>,
    pub aggregates: Vec<AggregateDecl>,
    pub value_objects: Vec<ValueObjectDecl>,
    pub enums: Vec<EnumDecl>,
    pub span: Span,
}

/// An object declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDecl {
    pub name: String,
    pub span: Span,
}

/// A morphism declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphismDecl {
    pub name: String,
    pub source: TypeExpr,
    pub target: TypeExpr,
    pub span: Span,
}

/// A type expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeExpr {
    Simple(String),
    Generic { name: String, arg: Box<TypeExpr> },
}

impl TypeExpr {
    pub fn simple(name: impl Into<String>) -> Self {
        Self::Simple(name.into())
    }

    pub fn generic(name: impl Into<String>, arg: TypeExpr) -> Self {
        Self::Generic {
            name: name.into(),
            arg: Box::new(arg),
        }
    }
}

/// An aggregate declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateDecl {
    pub name: String,
    pub root: String,
    pub contains: Vec<String>,
    pub invariants: Vec<InvariantDecl>,
    pub span: Span,
}

/// An invariant declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantDecl {
    pub expression: String,
    pub span: Span,
}

/// A value object declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueObjectDecl {
    pub name: String,
    pub fields: Vec<FieldDecl>,
    pub span: Span,
}

/// A field declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDecl {
    pub name: String,
    pub type_expr: TypeExpr,
    pub span: Span,
}

/// An enum declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<String>,
    pub span: Span,
}

/// A context map declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMapDecl {
    pub name: String,
    pub source_context: String,
    pub target_context: String,
    pub pattern: Option<String>,
    pub mappings: Vec<MappingDecl>,
    pub span: Span,
}

/// A mapping declaration in a context map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingDecl {
    pub source: String,
    pub target: String,
    pub span: Span,
}
