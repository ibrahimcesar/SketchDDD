//! # SketchDDD Parser
//!
//! Parser for the SketchDDD domain-specific language.
//!
//! This crate provides:
//! - A pest-based parser that produces an AST
//! - AST → Semantic Model transformation
//! - Pretty-printing for debugging
//!
//! ## Example
//!
//! ```text
//! context Commerce {
//!   objects { Customer, Order, LineItem, Product, Money }
//!
//!   morphisms {
//!     placedBy: Order -> Customer
//!     items: Order -> List<LineItem>
//!     product: LineItem -> Product
//!     price: LineItem -> Money
//!   }
//!
//!   aggregate Order {
//!     root: Order
//!     contains: [LineItem]
//!     invariant: totalPrice = sum(items.price)
//!   }
//!
//!   value Money {
//!     amount: Decimal
//!     currency: Currency
//!   }
//!
//!   enum OrderStatus = Pending | Confirmed | Shipped | Cancelled
//! }
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use sketchddd_parser::{parse_file, transform::transform, pretty::PrettyPrint};
//!
//! let source = r#"
//!     context Commerce {
//!         objects { Customer, Order }
//!     }
//! "#;
//!
//! // Parse to AST
//! let file = parse_file(source)?;
//!
//! // Pretty-print the AST
//! println!("{}", file.pretty_print());
//!
//! // Transform to semantic model
//! let result = transform(&file)?;
//! for ctx in result.contexts {
//!     println!("Context: {}", ctx.name());
//! }
//! ```

pub mod ast;
pub mod error;
pub mod grammar;
pub mod pretty;
pub mod transform;

pub use ast::*;
pub use error::ParseError;
pub use grammar::Rule;
pub use pretty::PrettyPrint;
pub use transform::{transform, TransformResult, TransformWarning};

use grammar::SketchDDDParser;
use pest::Parser;

/// Create a Span from a pest Pair.
fn span_from_pest<R: pest::RuleType>(pair: &pest::iterators::Pair<'_, R>) -> Span {
    let span = pair.as_span();
    let (line, column) = pair.line_col();
    Span::new(span.start(), span.end(), line as u32, column as u32)
}

/// Parse a SketchDDD source file into a File AST.
pub fn parse_file(source: &str) -> Result<File, ParseError> {
    let pairs = SketchDDDParser::parse(Rule::file, source).map_err(|e| {
        ParseError::new(format!("Parse error: {}", e))
    })?;

    let mut file = File::default();

    // The top-level is a single "file" rule containing context_decl and map_decl
    for pair in pairs {
        if pair.as_rule() == Rule::file {
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::context_decl => {
                        file.contexts.push(parse_context_decl(inner)?);
                    }
                    Rule::map_decl => {
                        file.context_maps.push(parse_map_decl(inner)?);
                    }
                    Rule::EOI => {}
                    _ => {}
                }
            }
        }
    }

    Ok(file)
}

/// Parse a SketchDDD source file into a list of context declarations.
/// This is a convenience function for backward compatibility.
pub fn parse(source: &str) -> Result<Vec<ContextDecl>, ParseError> {
    let file = parse_file(source)?;
    Ok(file.contexts)
}

/// Parse a single context definition.
pub fn parse_context(source: &str) -> Result<ContextDecl, ParseError> {
    let contexts = parse(source)?;
    contexts
        .into_iter()
        .next()
        .ok_or_else(|| ParseError::new("No context found in source"))
}

// =============================================================
// Context Parsing
// =============================================================

fn parse_context_decl(pair: pest::iterators::Pair<'_, Rule>) -> Result<ContextDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut context = ContextDecl::default();
    context.span = span;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                context.name = inner.as_str().to_string();
            }
            Rule::context_body => {
                parse_context_body(inner, &mut context)?;
            }
            _ => {}
        }
    }

    Ok(context)
}

fn parse_context_body(
    pair: pest::iterators::Pair<'_, Rule>,
    context: &mut ContextDecl,
) -> Result<(), ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::objects_block => {
                parse_objects_block(inner, context)?;
            }
            Rule::entity_block => {
                context.entities.push(parse_entity_block(inner)?);
            }
            Rule::morphisms_block => {
                parse_morphisms_block(inner, context)?;
            }
            Rule::aggregate_block => {
                context.aggregates.push(parse_aggregate_block(inner)?);
            }
            Rule::value_block => {
                context.value_objects.push(parse_value_block(inner)?);
            }
            Rule::enum_block => {
                context.enums.push(parse_enum_block(inner)?);
            }
            Rule::equation_block => {
                context.equations.push(parse_equation_block(inner)?);
            }
            _ => {}
        }
    }
    Ok(())
}

// =============================================================
// Objects Parsing
// =============================================================

fn parse_objects_block(
    pair: pest::iterators::Pair<'_, Rule>,
    context: &mut ContextDecl,
) -> Result<(), ParseError> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::identifier_list {
            for ident in inner.into_inner() {
                if ident.as_rule() == Rule::identifier {
                    context.objects.push(ObjectDecl {
                        name: ident.as_str().to_string(),
                        span: span_from_pest(&ident),
                    });
                }
            }
        }
    }
    Ok(())
}

// =============================================================
// Entity Parsing
// =============================================================

fn parse_entity_block(pair: pest::iterators::Pair<'_, Rule>) -> Result<EntityDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut entity = EntityDecl {
        name: String::new(),
        fields: Vec::new(),
        span,
    };

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                entity.name = inner.as_str().to_string();
            }
            Rule::entity_body => {
                for field_pair in inner.into_inner() {
                    if field_pair.as_rule() == Rule::field_decl {
                        entity.fields.push(parse_field_decl(field_pair)?);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(entity)
}

// =============================================================
// Morphisms Parsing
// =============================================================

fn parse_morphisms_block(
    pair: pest::iterators::Pair<'_, Rule>,
    context: &mut ContextDecl,
) -> Result<(), ParseError> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::morphism_decl {
            context.morphisms.push(parse_morphism_decl(inner)?);
        }
    }
    Ok(())
}

fn parse_morphism_decl(pair: pest::iterators::Pair<'_, Rule>) -> Result<MorphismDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut name = String::new();
    let mut source = TypeExpr::Simple(String::new());
    let mut target = TypeExpr::Simple(String::new());
    let mut annotations = Vec::new();
    let mut type_count = 0;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                name = inner.as_str().to_string();
            }
            Rule::type_expr | Rule::simple_type | Rule::generic_type => {
                let type_expr = parse_type_expr(inner)?;
                if type_count == 0 {
                    source = type_expr;
                } else {
                    target = type_expr;
                }
                type_count += 1;
            }
            Rule::morphism_annotations => {
                annotations = parse_annotations(inner)?;
            }
            _ => {}
        }
    }

    Ok(MorphismDecl {
        name,
        source,
        target,
        annotations,
        span,
    })
}

fn parse_annotations(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Result<Vec<Annotation>, ParseError> {
    let mut annotations = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::annotation {
            let mut name = String::new();
            let mut value = None;

            for ann_inner in inner.into_inner() {
                match ann_inner.as_rule() {
                    Rule::identifier => {
                        if name.is_empty() {
                            name = ann_inner.as_str().to_string();
                        } else {
                            value = Some(ann_inner.as_str().to_string());
                        }
                    }
                    Rule::string_literal => {
                        // Remove quotes from string
                        let s = ann_inner.as_str();
                        value = Some(s[1..s.len() - 1].to_string());
                    }
                    Rule::number => {
                        value = Some(ann_inner.as_str().to_string());
                    }
                    _ => {}
                }
            }

            annotations.push(Annotation { name, value });
        }
    }

    Ok(annotations)
}

// =============================================================
// Type Expression Parsing
// =============================================================

fn parse_type_expr(pair: pest::iterators::Pair<'_, Rule>) -> Result<TypeExpr, ParseError> {
    let inner = pair.into_inner();

    // Check for optional marker at the end
    let type_parts: Vec<_> = inner.collect();

    if type_parts.is_empty() {
        return Err(ParseError::new("Expected type expression"));
    }

    let base_type = match type_parts[0].as_rule() {
        Rule::simple_type | Rule::identifier => TypeExpr::Simple(type_parts[0].as_str().to_string()),
        Rule::generic_type => parse_generic_type(type_parts[0].clone())?,
        _ => TypeExpr::Simple(type_parts[0].as_str().to_string()),
    };

    // The grammar now includes "?" inline, so check the original string
    let pair_str = type_parts.last().map(|p| p.as_str()).unwrap_or("");
    if pair_str == "?" {
        Ok(TypeExpr::Optional(Box::new(base_type)))
    } else {
        Ok(base_type)
    }
}

fn parse_generic_type(pair: pest::iterators::Pair<'_, Rule>) -> Result<TypeExpr, ParseError> {
    let mut name = String::new();
    let mut args = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                name = inner.as_str().to_string();
            }
            Rule::type_expr_list => {
                for type_pair in inner.into_inner() {
                    if matches!(
                        type_pair.as_rule(),
                        Rule::type_expr | Rule::simple_type | Rule::generic_type
                    ) {
                        args.push(parse_type_expr(type_pair)?);
                    }
                }
            }
            Rule::type_expr | Rule::simple_type | Rule::generic_type => {
                args.push(parse_type_expr(inner)?);
            }
            _ => {}
        }
    }

    Ok(TypeExpr::Generic { name, args })
}

// =============================================================
// Aggregate Parsing
// =============================================================

fn parse_aggregate_block(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Result<AggregateDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut aggregate = AggregateDecl {
        name: String::new(),
        root: None,
        contains: Vec::new(),
        invariants: Vec::new(),
        span,
    };

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                aggregate.name = inner.as_str().to_string();
            }
            Rule::aggregate_body => {
                parse_aggregate_body(inner, &mut aggregate)?;
            }
            _ => {}
        }
    }

    Ok(aggregate)
}

fn parse_aggregate_body(
    pair: pest::iterators::Pair<'_, Rule>,
    aggregate: &mut AggregateDecl,
) -> Result<(), ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::root_clause => {
                for ident in inner.into_inner() {
                    if ident.as_rule() == Rule::identifier {
                        aggregate.root = Some(ident.as_str().to_string());
                    }
                }
            }
            Rule::contains_clause => {
                for id_list in inner.into_inner() {
                    if id_list.as_rule() == Rule::identifier_list {
                        for ident in id_list.into_inner() {
                            if ident.as_rule() == Rule::identifier {
                                aggregate.contains.push(ident.as_str().to_string());
                            }
                        }
                    }
                }
            }
            Rule::invariant_clause => {
                aggregate.invariants.push(parse_invariant_clause(inner)?);
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_invariant_clause(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Result<InvariantDecl, ParseError> {
    let span = span_from_pest(&pair);

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::expression {
            return Ok(InvariantDecl {
                expression: parse_expression(inner)?,
                span,
            });
        }
    }

    Err(ParseError::new("Expected expression in invariant"))
}

// =============================================================
// Value Object Parsing
// =============================================================

fn parse_value_block(pair: pest::iterators::Pair<'_, Rule>) -> Result<ValueObjectDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut value_object = ValueObjectDecl {
        name: String::new(),
        fields: Vec::new(),
        span,
    };

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                value_object.name = inner.as_str().to_string();
            }
            Rule::field_decl => {
                value_object.fields.push(parse_field_decl(inner)?);
            }
            _ => {}
        }
    }

    Ok(value_object)
}

fn parse_field_decl(pair: pest::iterators::Pair<'_, Rule>) -> Result<FieldDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut name = String::new();
    let mut type_expr = TypeExpr::Simple(String::new());

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                name = inner.as_str().to_string();
            }
            Rule::type_expr | Rule::simple_type | Rule::generic_type => {
                type_expr = parse_type_expr(inner)?;
            }
            _ => {}
        }
    }

    Ok(FieldDecl {
        name,
        type_expr,
        span,
    })
}

// =============================================================
// Enum Parsing
// =============================================================

fn parse_enum_block(pair: pest::iterators::Pair<'_, Rule>) -> Result<EnumDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut enum_decl = EnumDecl {
        name: String::new(),
        variants: Vec::new(),
        span,
    };

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                enum_decl.name = inner.as_str().to_string();
            }
            Rule::variant_list => {
                for variant in inner.into_inner() {
                    if variant.as_rule() == Rule::variant {
                        enum_decl.variants.push(parse_variant(variant)?);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(enum_decl)
}

fn parse_variant(pair: pest::iterators::Pair<'_, Rule>) -> Result<VariantDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut name = String::new();
    let mut payload = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                name = inner.as_str().to_string();
            }
            Rule::variant_payload => {
                for type_list in inner.into_inner() {
                    if type_list.as_rule() == Rule::type_expr_list {
                        for type_pair in type_list.into_inner() {
                            if matches!(
                                type_pair.as_rule(),
                                Rule::type_expr | Rule::simple_type | Rule::generic_type
                            ) {
                                payload.push(parse_type_expr(type_pair)?);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(VariantDecl {
        name,
        payload,
        span,
    })
}

// =============================================================
// Equation Parsing
// =============================================================

fn parse_equation_block(pair: pest::iterators::Pair<'_, Rule>) -> Result<EquationDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut name = None;
    let mut lhs = Path::new(Vec::new());
    let mut rhs = Path::new(Vec::new());
    let mut path_count = 0;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                name = Some(inner.as_str().to_string());
            }
            Rule::path => {
                let path = parse_path(inner)?;
                if path_count == 0 {
                    lhs = path;
                } else {
                    rhs = path;
                }
                path_count += 1;
            }
            _ => {}
        }
    }

    Ok(EquationDecl {
        name,
        lhs,
        rhs,
        span,
    })
}

fn parse_path(pair: pest::iterators::Pair<'_, Rule>) -> Result<Path, ParseError> {
    let mut components = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::identifier {
            components.push(inner.as_str().to_string());
        }
    }

    Ok(Path::new(components))
}

// =============================================================
// Context Map Parsing
// =============================================================

fn parse_map_decl(pair: pest::iterators::Pair<'_, Rule>) -> Result<ContextMapDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut name = String::new();
    let mut source_context = String::new();
    let mut target_context = String::new();
    let mut pattern = None;
    let mut object_mappings = Vec::new();
    let mut morphism_mappings = Vec::new();
    let mut ident_count = 0;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                match ident_count {
                    0 => name = inner.as_str().to_string(),
                    1 => source_context = inner.as_str().to_string(),
                    2 => target_context = inner.as_str().to_string(),
                    _ => {}
                }
                ident_count += 1;
            }
            Rule::map_body => {
                parse_map_body(
                    inner,
                    &mut pattern,
                    &mut object_mappings,
                    &mut morphism_mappings,
                )?;
            }
            _ => {}
        }
    }

    Ok(ContextMapDecl {
        name,
        source_context,
        target_context,
        pattern,
        object_mappings,
        morphism_mappings,
        span,
    })
}

fn parse_map_body(
    pair: pest::iterators::Pair<'_, Rule>,
    pattern: &mut Option<String>,
    object_mappings: &mut Vec<ObjectMappingDecl>,
    morphism_mappings: &mut Vec<MorphismMappingDecl>,
) -> Result<(), ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::pattern_clause => {
                for pat in inner.into_inner() {
                    if pat.as_rule() == Rule::map_pattern {
                        *pattern = Some(normalize_pattern(pat.as_str()));
                    }
                }
            }
            Rule::mappings_block => {
                for mapping in inner.into_inner() {
                    if mapping.as_rule() == Rule::object_mapping_decl {
                        object_mappings.push(parse_object_mapping(mapping)?);
                    }
                }
            }
            Rule::morphism_mappings_block => {
                for mapping in inner.into_inner() {
                    if mapping.as_rule() == Rule::morphism_mapping_decl {
                        morphism_mappings.push(parse_morphism_mapping(mapping)?);
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn normalize_pattern(pattern: &str) -> String {
    match pattern {
        "ACL" => "AntiCorruptionLayer".to_string(),
        "OHS" => "OpenHostService".to_string(),
        other => other.to_string(),
    }
}

fn parse_object_mapping(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Result<ObjectMappingDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut source = String::new();
    let mut target = String::new();
    let mut description = None;
    let mut ident_count = 0;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                if ident_count == 0 {
                    source = inner.as_str().to_string();
                } else {
                    target = inner.as_str().to_string();
                }
                ident_count += 1;
            }
            Rule::mapping_description => {
                for desc in inner.into_inner() {
                    if desc.as_rule() == Rule::string_literal {
                        let s = desc.as_str();
                        description = Some(s[1..s.len() - 1].to_string());
                    }
                }
            }
            _ => {}
        }
    }

    Ok(ObjectMappingDecl {
        source,
        target,
        description,
        span,
    })
}

fn parse_morphism_mapping(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Result<MorphismMappingDecl, ParseError> {
    let span = span_from_pest(&pair);
    let mut source = String::new();
    let mut target = String::new();
    let mut description = None;
    let mut ident_count = 0;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                if ident_count == 0 {
                    source = inner.as_str().to_string();
                } else {
                    target = inner.as_str().to_string();
                }
                ident_count += 1;
            }
            Rule::mapping_description => {
                for desc in inner.into_inner() {
                    if desc.as_rule() == Rule::string_literal {
                        let s = desc.as_str();
                        description = Some(s[1..s.len() - 1].to_string());
                    }
                }
            }
            _ => {}
        }
    }

    Ok(MorphismMappingDecl {
        source,
        target,
        description,
        span,
    })
}

// =============================================================
// Expression Parsing
// =============================================================

fn parse_expression(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    parse_comparison_expr(pair)
}

fn parse_comparison_expr(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner().peekable();

    // Handle expression or comparison_expr
    let first = inner.next().ok_or_else(|| ParseError::new("Expected expression"))?;

    let mut left = match first.as_rule() {
        Rule::additive_expr => parse_additive_expr(first)?,
        Rule::comparison_expr => parse_comparison_expr(first)?,
        Rule::expression => parse_expression(first)?,
        _ => parse_additive_expr(first)?,
    };

    while let Some(op_pair) = inner.next() {
        if op_pair.as_rule() == Rule::comparison_op {
            let op = parse_comparison_op(op_pair.as_str())?;
            let right_pair = inner.next().ok_or_else(|| ParseError::new("Expected right operand"))?;
            let right = parse_additive_expr(right_pair)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
    }

    Ok(left)
}

fn parse_additive_expr(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner().peekable();

    let first = inner.next().ok_or_else(|| ParseError::new("Expected expression"))?;
    let mut left = match first.as_rule() {
        Rule::multiplicative_expr => parse_multiplicative_expr(first)?,
        _ => parse_multiplicative_expr(first)?,
    };

    while let Some(op_pair) = inner.next() {
        if op_pair.as_rule() == Rule::additive_op {
            let op = match op_pair.as_str() {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Sub,
                _ => return Err(ParseError::new("Unknown additive operator")),
            };
            let right_pair = inner.next().ok_or_else(|| ParseError::new("Expected right operand"))?;
            let right = parse_multiplicative_expr(right_pair)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
    }

    Ok(left)
}

fn parse_multiplicative_expr(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner().peekable();

    let first = inner.next().ok_or_else(|| ParseError::new("Expected expression"))?;
    let mut left = match first.as_rule() {
        Rule::unary_expr => parse_unary_expr(first)?,
        _ => parse_unary_expr(first)?,
    };

    while let Some(op_pair) = inner.next() {
        if op_pair.as_rule() == Rule::multiplicative_op {
            let op = match op_pair.as_str() {
                "*" => BinaryOperator::Mul,
                "/" => BinaryOperator::Div,
                "%" => BinaryOperator::Mod,
                _ => return Err(ParseError::new("Unknown multiplicative operator")),
            };
            let right_pair = inner.next().ok_or_else(|| ParseError::new("Expected right operand"))?;
            let right = parse_unary_expr(right_pair)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
    }

    Ok(left)
}

fn parse_unary_expr(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();

    let first = inner.next().ok_or_else(|| ParseError::new("Expected expression"))?;

    match first.as_rule() {
        Rule::unary_op => {
            let op = match first.as_str() {
                "!" => UnaryOperator::Not,
                "-" => UnaryOperator::Neg,
                _ => return Err(ParseError::new("Unknown unary operator")),
            };
            let operand = inner.next().ok_or_else(|| ParseError::new("Expected operand"))?;
            Ok(Expr::UnaryOp {
                op,
                operand: Box::new(parse_postfix_expr(operand)?),
            })
        }
        Rule::postfix_expr => parse_postfix_expr(first),
        _ => parse_postfix_expr(first),
    }
}

fn parse_postfix_expr(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();

    let primary = inner.next().ok_or_else(|| ParseError::new("Expected primary expression"))?;
    let mut expr = parse_primary_expr(primary)?;

    for postfix in inner {
        if postfix.as_rule() == Rule::postfix_op {
            for postfix_inner in postfix.into_inner() {
                match postfix_inner.as_rule() {
                    Rule::identifier => {
                        // Field access: expr.field
                        if let Expr::Path(mut path) = expr {
                            path.components.push(postfix_inner.as_str().to_string());
                            expr = Expr::Path(path);
                        } else {
                            // For non-path expressions, create a new path
                            expr = Expr::Path(Path::single(postfix_inner.as_str()));
                        }
                    }
                    Rule::expression => {
                        // Index access: expr[index]
                        let index = parse_expression(postfix_inner)?;
                        expr = Expr::Index {
                            expr: Box::new(expr),
                            index: Box::new(index),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(expr)
}

fn parse_primary_expr(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    let inner = pair.into_inner().next();

    match inner {
        Some(p) => match p.as_rule() {
            Rule::function_call => parse_function_call(p),
            Rule::path_expr => parse_path_expr(p),
            Rule::number => {
                let num: f64 = p.as_str().parse().map_err(|_| ParseError::new("Invalid number"))?;
                Ok(Expr::Number(num))
            }
            Rule::string_literal => {
                let s = p.as_str();
                Ok(Expr::String(s[1..s.len() - 1].to_string()))
            }
            Rule::expression => parse_expression(p),
            Rule::identifier => Ok(Expr::Path(Path::single(p.as_str()))),
            _ => Err(ParseError::new(format!("Unexpected primary expression: {:?}", p.as_rule()))),
        },
        None => Err(ParseError::new("Expected primary expression")),
    }
}

fn parse_function_call(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    let mut name = String::new();
    let mut args = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                name = inner.as_str().to_string();
            }
            Rule::arg_list => {
                for arg in inner.into_inner() {
                    if arg.as_rule() == Rule::expression {
                        args.push(parse_expression(arg)?);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Expr::FunctionCall { name, args })
}

fn parse_path_expr(pair: pest::iterators::Pair<'_, Rule>) -> Result<Expr, ParseError> {
    let mut components = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::identifier {
            components.push(inner.as_str().to_string());
        }
    }

    Ok(Expr::Path(Path::new(components)))
}

fn parse_comparison_op(op: &str) -> Result<BinaryOperator, ParseError> {
    match op {
        "==" | "=" => Ok(BinaryOperator::Eq),
        "!=" => Ok(BinaryOperator::Ne),
        "<" => Ok(BinaryOperator::Lt),
        "<=" => Ok(BinaryOperator::Le),
        ">" => Ok(BinaryOperator::Gt),
        ">=" => Ok(BinaryOperator::Ge),
        _ => Err(ParseError::new(format!("Unknown comparison operator: {}", op))),
    }
}

// =============================================================
// Tests
// =============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_empty() {
        let result = parse("");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_parse_empty_context() {
        let source = r#"
            context Commerce {
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].name, "Commerce");
    }

    #[test]
    fn test_parse_objects() {
        let source = r#"
            context Commerce {
                objects { Customer, Order, LineItem }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts[0].objects.len(), 3);
        assert_eq!(contexts[0].objects[0].name, "Customer");
        assert_eq!(contexts[0].objects[1].name, "Order");
        assert_eq!(contexts[0].objects[2].name, "LineItem");
    }

    #[test]
    fn test_parse_morphisms() {
        let source = r#"
            context Commerce {
                morphisms {
                    placedBy: Order -> Customer
                    items: Order -> List<LineItem>
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts[0].morphisms.len(), 2);
        assert_eq!(contexts[0].morphisms[0].name, "placedBy");
        assert_eq!(contexts[0].morphisms[0].source, TypeExpr::simple("Order"));
        assert_eq!(contexts[0].morphisms[0].target, TypeExpr::simple("Customer"));
        assert_eq!(contexts[0].morphisms[1].name, "items");
        assert_eq!(
            contexts[0].morphisms[1].target,
            TypeExpr::generic("List", TypeExpr::simple("LineItem"))
        );
    }

    #[test]
    fn test_parse_aggregate() {
        let source = r#"
            context Commerce {
                aggregate Order {
                    root: Order
                    contains: [LineItem, Payment]
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts[0].aggregates.len(), 1);
        assert_eq!(contexts[0].aggregates[0].name, "Order");
        assert_eq!(contexts[0].aggregates[0].root, Some("Order".to_string()));
        assert_eq!(contexts[0].aggregates[0].contains, vec!["LineItem", "Payment"]);
    }

    #[test]
    fn test_parse_value_object() {
        let source = r#"
            context Commerce {
                value Money {
                    amount: Decimal
                    currency: Currency
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts[0].value_objects.len(), 1);
        assert_eq!(contexts[0].value_objects[0].name, "Money");
        assert_eq!(contexts[0].value_objects[0].fields.len(), 2);
        assert_eq!(contexts[0].value_objects[0].fields[0].name, "amount");
        assert_eq!(contexts[0].value_objects[0].fields[1].name, "currency");
    }

    #[test]
    fn test_parse_enum() {
        let source = r#"
            context Commerce {
                enum OrderStatus = Pending | Confirmed | Shipped | Cancelled
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts[0].enums.len(), 1);
        assert_eq!(contexts[0].enums[0].name, "OrderStatus");
        assert_eq!(contexts[0].enums[0].variants.len(), 4);
        assert_eq!(contexts[0].enums[0].variants[0].name, "Pending");
        assert_eq!(contexts[0].enums[0].variants[1].name, "Confirmed");
        assert_eq!(contexts[0].enums[0].variants[2].name, "Shipped");
        assert_eq!(contexts[0].enums[0].variants[3].name, "Cancelled");
    }

    #[test]
    fn test_parse_context_map() {
        let source = r#"
            map CommerceToShipping: Commerce -> Shipping {
                pattern: CustomerSupplier
                mappings {
                    Order -> Shipment
                    Customer -> Recipient
                }
            }
        "#;
        let result = parse_file(source);
        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.context_maps.len(), 1);
        let map = &file.context_maps[0];
        assert_eq!(map.name, "CommerceToShipping");
        assert_eq!(map.source_context, "Commerce");
        assert_eq!(map.target_context, "Shipping");
        assert_eq!(map.pattern, Some("CustomerSupplier".to_string()));
        assert_eq!(map.object_mappings.len(), 2);
    }

    #[test]
    fn test_parse_comments() {
        let source = r#"
            // This is a single-line comment
            context Commerce {
                /* This is a
                   multi-line comment */
                objects { Customer, Order }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts[0].objects.len(), 2);
    }

    #[test]
    fn test_parse_full_example() {
        let source = r#"
            context Commerce {
                objects { Customer, Order, LineItem, Product, Money }

                morphisms {
                    placedBy: Order -> Customer
                    items: Order -> List<LineItem>
                    product: LineItem -> Product
                    price: LineItem -> Money
                }

                aggregate Order {
                    root: Order
                    contains: [LineItem]
                }

                value Money {
                    amount: Decimal
                    currency: Currency
                }

                enum OrderStatus = Pending | Confirmed | Shipped | Cancelled
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].name, "Commerce");
        assert_eq!(contexts[0].objects.len(), 5);
        assert_eq!(contexts[0].morphisms.len(), 4);
        assert_eq!(contexts[0].aggregates.len(), 1);
        assert_eq!(contexts[0].value_objects.len(), 1);
        assert_eq!(contexts[0].enums.len(), 1);
    }

    #[test]
    fn test_parse_acl_alias() {
        let source = r#"
            map LegacyIntegration: Legacy -> New {
                pattern: ACL
            }
        "#;
        let result = parse_file(source);
        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.context_maps[0].pattern, Some("AntiCorruptionLayer".to_string()));
    }

    #[test]
    fn test_parse_entity_block() {
        let source = r#"
            context Commerce {
                entity Customer {
                    id: UUID
                    name: String
                    email: Email
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts[0].entities.len(), 1);
        assert_eq!(contexts[0].entities[0].name, "Customer");
        assert_eq!(contexts[0].entities[0].fields.len(), 3);
    }

    #[test]
    fn test_parse_multiple_contexts() {
        let source = r#"
            context Commerce {
                objects { Customer, Order }
            }

            context Shipping {
                objects { Shipment, Carrier }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok());
        let contexts = result.unwrap();
        assert_eq!(contexts.len(), 2);
        assert_eq!(contexts[0].name, "Commerce");
        assert_eq!(contexts[1].name, "Shipping");
    }
}
