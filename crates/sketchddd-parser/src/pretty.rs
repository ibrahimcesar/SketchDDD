//! Pretty-printing for AST nodes.
//!
//! This module provides human-readable representations of AST nodes
//! for debugging and error reporting purposes.

use std::fmt::{self, Display, Write};

use crate::ast::*;

/// Pretty-print configuration.
#[derive(Debug, Clone)]
pub struct PrettyConfig {
    /// Indentation string (default: 2 spaces)
    pub indent: String,
    /// Whether to include spans in output
    pub show_spans: bool,
}

impl Default for PrettyConfig {
    fn default() -> Self {
        Self {
            indent: "  ".to_string(),
            show_spans: false,
        }
    }
}

/// A wrapper for pretty-printing AST nodes.
#[allow(dead_code)]
pub struct Pretty<'a, T> {
    node: &'a T,
    config: PrettyConfig,
}

impl<'a, T> Pretty<'a, T> {
    /// Create a new pretty wrapper with default config.
    #[allow(dead_code)]
    pub fn new(node: &'a T) -> Self {
        Self {
            node,
            config: PrettyConfig::default(),
        }
    }

    /// Create a new pretty wrapper with custom config.
    #[allow(dead_code)]
    pub fn with_config(node: &'a T, config: PrettyConfig) -> Self {
        Self { node, config }
    }
}

/// Trait for AST nodes that can be pretty-printed.
pub trait PrettyPrint {
    /// Pretty-print the node to a string.
    fn pretty_print(&self) -> String {
        self.pretty_print_with_config(&PrettyConfig::default())
    }

    /// Pretty-print the node with custom configuration.
    fn pretty_print_with_config(&self, config: &PrettyConfig) -> String;
}

impl PrettyPrint for File {
    fn pretty_print_with_config(&self, config: &PrettyConfig) -> String {
        let mut output = String::new();

        for ctx in &self.contexts {
            output.push_str(&ctx.pretty_print_with_config(config));
            output.push('\n');
        }

        for map in &self.context_maps {
            output.push_str(&map.pretty_print_with_config(config));
            output.push('\n');
        }

        output
    }
}

impl PrettyPrint for ContextDecl {
    fn pretty_print_with_config(&self, config: &PrettyConfig) -> String {
        let mut output = String::new();
        let indent = &config.indent;

        writeln!(output, "context {} {{", self.name).unwrap();

        // Objects
        if !self.objects.is_empty() {
            write!(output, "{}objects {{ ", indent).unwrap();
            let names: Vec<_> = self.objects.iter().map(|o| o.name.as_str()).collect();
            write!(output, "{}", names.join(", ")).unwrap();
            writeln!(output, " }}").unwrap();
        }

        // Entities
        for entity in &self.entities {
            output.push_str(&entity.pretty_print_indented(indent, config));
        }

        // Morphisms
        if !self.morphisms.is_empty() {
            writeln!(output, "{}morphisms {{", indent).unwrap();
            for morph in &self.morphisms {
                output.push_str(&morph.pretty_print_indented(&format!("{}{}", indent, indent), config));
            }
            writeln!(output, "{}}}", indent).unwrap();
        }

        // Aggregates
        for agg in &self.aggregates {
            output.push_str(&agg.pretty_print_indented(indent, config));
        }

        // Value objects
        for vo in &self.value_objects {
            output.push_str(&vo.pretty_print_indented(indent, config));
        }

        // Enums
        for enum_decl in &self.enums {
            output.push_str(&enum_decl.pretty_print_indented(indent, config));
        }

        // Equations
        for eq in &self.equations {
            output.push_str(&eq.pretty_print_indented(indent, config));
        }

        writeln!(output, "}}").unwrap();
        output
    }
}

impl PrettyPrint for ContextMapDecl {
    fn pretty_print_with_config(&self, config: &PrettyConfig) -> String {
        let mut output = String::new();
        let indent = &config.indent;

        writeln!(
            output,
            "map {}: {} -> {} {{",
            self.name, self.source_context, self.target_context
        )
        .unwrap();

        if let Some(pattern) = &self.pattern {
            writeln!(output, "{}pattern: {}", indent, pattern).unwrap();
        }

        if !self.object_mappings.is_empty() {
            writeln!(output, "{}mappings {{", indent).unwrap();
            for mapping in &self.object_mappings {
                write!(
                    output,
                    "{}{}{} -> {}",
                    indent, indent, mapping.source, mapping.target
                )
                .unwrap();
                if let Some(desc) = &mapping.description {
                    write!(output, ": \"{}\"", desc).unwrap();
                }
                writeln!(output).unwrap();
            }
            writeln!(output, "{}}}", indent).unwrap();
        }

        if !self.morphism_mappings.is_empty() {
            writeln!(output, "{}morphism_mappings {{", indent).unwrap();
            for mapping in &self.morphism_mappings {
                write!(
                    output,
                    "{}{}{} -> {}",
                    indent, indent, mapping.source, mapping.target
                )
                .unwrap();
                if let Some(desc) = &mapping.description {
                    write!(output, ": \"{}\"", desc).unwrap();
                }
                writeln!(output).unwrap();
            }
            writeln!(output, "{}}}", indent).unwrap();
        }

        writeln!(output, "}}").unwrap();
        output
    }
}

// Helper trait for indented pretty-printing
trait PrettyPrintIndented {
    fn pretty_print_indented(&self, indent: &str, config: &PrettyConfig) -> String;
}

impl PrettyPrintIndented for EntityDecl {
    fn pretty_print_indented(&self, indent: &str, config: &PrettyConfig) -> String {
        let mut output = String::new();
        let inner_indent = format!("{}{}", indent, config.indent);

        if self.fields.is_empty() {
            writeln!(output, "{}entity {}", indent, self.name).unwrap();
        } else {
            writeln!(output, "{}entity {} {{", indent, self.name).unwrap();
            for field in &self.fields {
                writeln!(
                    output,
                    "{}{}: {}",
                    inner_indent,
                    field.name,
                    format_type_expr(&field.type_expr)
                )
                .unwrap();
            }
            writeln!(output, "{}}}", indent).unwrap();
        }

        output
    }
}

impl PrettyPrintIndented for MorphismDecl {
    fn pretty_print_indented(&self, indent: &str, _config: &PrettyConfig) -> String {
        let mut output = String::new();

        write!(
            output,
            "{}{}: {} -> {}",
            indent,
            self.name,
            format_type_expr(&self.source),
            format_type_expr(&self.target)
        )
        .unwrap();

        if !self.annotations.is_empty() {
            write!(output, " [").unwrap();
            let anns: Vec<_> = self
                .annotations
                .iter()
                .map(|a| {
                    if let Some(v) = &a.value {
                        format!("{}={}", a.name, v)
                    } else {
                        a.name.clone()
                    }
                })
                .collect();
            write!(output, "{}", anns.join(", ")).unwrap();
            write!(output, "]").unwrap();
        }

        writeln!(output).unwrap();
        output
    }
}

impl PrettyPrintIndented for AggregateDecl {
    fn pretty_print_indented(&self, indent: &str, config: &PrettyConfig) -> String {
        let mut output = String::new();
        let inner_indent = format!("{}{}", indent, config.indent);

        writeln!(output, "{}aggregate {} {{", indent, self.name).unwrap();

        if let Some(root) = &self.root {
            writeln!(output, "{}root: {}", inner_indent, root).unwrap();
        }

        if !self.contains.is_empty() {
            writeln!(
                output,
                "{}contains: [{}]",
                inner_indent,
                self.contains.join(", ")
            )
            .unwrap();
        }

        for inv in &self.invariants {
            writeln!(
                output,
                "{}invariant: {}",
                inner_indent,
                format_expr(&inv.expression)
            )
            .unwrap();
        }

        writeln!(output, "{}}}", indent).unwrap();
        output
    }
}

impl PrettyPrintIndented for ValueObjectDecl {
    fn pretty_print_indented(&self, indent: &str, config: &PrettyConfig) -> String {
        let mut output = String::new();
        let inner_indent = format!("{}{}", indent, config.indent);

        writeln!(output, "{}value {} {{", indent, self.name).unwrap();
        for field in &self.fields {
            writeln!(
                output,
                "{}{}: {}",
                inner_indent,
                field.name,
                format_type_expr(&field.type_expr)
            )
            .unwrap();
        }
        writeln!(output, "{}}}", indent).unwrap();
        output
    }
}

impl PrettyPrintIndented for EnumDecl {
    fn pretty_print_indented(&self, indent: &str, _config: &PrettyConfig) -> String {
        let mut output = String::new();

        write!(output, "{}enum {} = ", indent, self.name).unwrap();

        let variants: Vec<_> = self
            .variants
            .iter()
            .map(|v| {
                if v.payload.is_empty() {
                    v.name.clone()
                } else {
                    let types: Vec<_> = v.payload.iter().map(format_type_expr).collect();
                    format!("{}({})", v.name, types.join(", "))
                }
            })
            .collect();

        writeln!(output, "{}", variants.join(" | ")).unwrap();
        output
    }
}

impl PrettyPrintIndented for EquationDecl {
    fn pretty_print_indented(&self, indent: &str, _config: &PrettyConfig) -> String {
        let mut output = String::new();

        write!(output, "{}equation ", indent).unwrap();
        if let Some(name) = &self.name {
            write!(output, "{}: ", name).unwrap();
        }
        writeln!(
            output,
            "{} = {}",
            format_path(&self.lhs),
            format_path(&self.rhs)
        )
        .unwrap();

        output
    }
}

/// Format a type expression to a string.
pub fn format_type_expr(type_expr: &TypeExpr) -> String {
    match type_expr {
        TypeExpr::Simple(name) => name.clone(),
        TypeExpr::Generic { name, args } => {
            let args_str: Vec<_> = args.iter().map(format_type_expr).collect();
            format!("{}<{}>", name, args_str.join(", "))
        }
        TypeExpr::Optional(inner) => format!("{}?", format_type_expr(inner)),
    }
}

/// Format a path to a string.
pub fn format_path(path: &Path) -> String {
    path.components.join(".")
}

/// Format an expression to a string.
pub fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        Expr::String(s) => format!("\"{}\"", s),
        Expr::Path(path) => format_path(path),
        Expr::BinaryOp { left, op, right } => {
            let op_str = match op {
                BinaryOperator::Add => "+",
                BinaryOperator::Sub => "-",
                BinaryOperator::Mul => "*",
                BinaryOperator::Div => "/",
                BinaryOperator::Mod => "%",
                BinaryOperator::Eq => "=",
                BinaryOperator::Ne => "!=",
                BinaryOperator::Lt => "<",
                BinaryOperator::Le => "<=",
                BinaryOperator::Gt => ">",
                BinaryOperator::Ge => ">=",
            };
            format!("{} {} {}", format_expr(left), op_str, format_expr(right))
        }
        Expr::UnaryOp { op, operand } => {
            let op_str = match op {
                UnaryOperator::Not => "!",
                UnaryOperator::Neg => "-",
            };
            format!("{}{}", op_str, format_expr(operand))
        }
        Expr::FunctionCall { name, args } => {
            let args_str: Vec<_> = args.iter().map(format_expr).collect();
            format!("{}({})", name, args_str.join(", "))
        }
        Expr::Index { expr, index } => {
            format!("{}[{}]", format_expr(expr), format_expr(index))
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

impl Display for ContextDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

impl Display for ContextMapDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

impl Display for TypeExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_type_expr(self))
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_path(self))
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_expr(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_file;

    #[test]
    fn test_pretty_print_simple_context() {
        let source = r#"
            context Commerce {
                objects { Customer, Order }
            }
        "#;
        let file = parse_file(source).unwrap();
        let output = file.pretty_print();

        assert!(output.contains("context Commerce {"));
        assert!(output.contains("objects { Customer, Order }"));
    }

    #[test]
    fn test_pretty_print_context_with_morphisms() {
        let source = r#"
            context Commerce {
                objects { Customer, Order }
                morphisms {
                    placedBy: Order -> Customer
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let output = file.pretty_print();

        assert!(output.contains("morphisms {"));
        assert!(output.contains("placedBy: Order -> Customer"));
    }

    #[test]
    fn test_pretty_print_value_object() {
        let source = r#"
            context Commerce {
                value Money {
                    amount: Decimal
                    currency: Currency
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let output = file.pretty_print();

        assert!(output.contains("value Money {"));
        assert!(output.contains("amount: Decimal"));
        assert!(output.contains("currency: Currency"));
    }

    #[test]
    fn test_pretty_print_enum() {
        let source = r#"
            context Commerce {
                enum OrderStatus = Pending | Confirmed | Shipped
            }
        "#;
        let file = parse_file(source).unwrap();
        let output = file.pretty_print();

        assert!(output.contains("enum OrderStatus = Pending | Confirmed | Shipped"));
    }

    #[test]
    fn test_pretty_print_context_map() {
        let source = r#"
            map CommerceToShipping: Commerce -> Shipping {
                pattern: CustomerSupplier
                mappings {
                    Order -> Shipment
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let output = file.pretty_print();

        assert!(output.contains("map CommerceToShipping: Commerce -> Shipping {"));
        assert!(output.contains("pattern: CustomerSupplier"));
        assert!(output.contains("Order -> Shipment"));
    }

    #[test]
    fn test_format_type_expr_simple() {
        let type_expr = TypeExpr::simple("Customer");
        assert_eq!(format_type_expr(&type_expr), "Customer");
    }

    #[test]
    fn test_format_type_expr_generic() {
        let type_expr = TypeExpr::generic("List", TypeExpr::simple("Order"));
        assert_eq!(format_type_expr(&type_expr), "List<Order>");
    }

    #[test]
    fn test_format_type_expr_optional() {
        let type_expr = TypeExpr::optional(TypeExpr::simple("Customer"));
        assert_eq!(format_type_expr(&type_expr), "Customer?");
    }

    #[test]
    fn test_format_expr_binary() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Path(Path::single("a"))),
            op: BinaryOperator::Add,
            right: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(format_expr(&expr), "a + 5");
    }

    #[test]
    fn test_format_expr_function_call() {
        let expr = Expr::FunctionCall {
            name: "sum".to_string(),
            args: vec![Expr::Path(Path::new(vec![
                "items".to_string(),
                "price".to_string(),
            ]))],
        };
        assert_eq!(format_expr(&expr), "sum(items.price)");
    }

    #[test]
    fn test_display_type_expr() {
        let type_expr = TypeExpr::generic("Map", TypeExpr::simple("Key"));
        assert_eq!(format!("{}", type_expr), "Map<Key>");
    }

    #[test]
    fn test_display_path() {
        let path = Path::new(vec!["order".to_string(), "items".to_string(), "price".to_string()]);
        assert_eq!(format!("{}", path), "order.items.price");
    }
}
