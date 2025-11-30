//! AST to Semantic Model transformation.
//!
//! This module transforms the parsed AST into the semantic model defined in
//! `sketchddd-core`. The transformation validates references and constructs
//! the categorical representation of the domain model.

use std::collections::HashMap;

use sketchddd_core::{
    BoundedContext, NamedContextMap, NamedMorphismMapping, NamedObjectMapping, RelationshipPattern,
};

use crate::ast::{
    AggregateDecl, ContextDecl, ContextMapDecl, EnumDecl, EquationDecl, File,
    MorphismDecl, ValueObjectDecl,
};
use crate::error::ParseError;

/// Result of transforming an AST to a semantic model.
#[derive(Debug)]
pub struct TransformResult {
    /// Bounded contexts extracted from the file
    pub contexts: Vec<BoundedContext>,
    /// Context maps between contexts (using named mappings)
    pub context_maps: Vec<NamedContextMap>,
    /// Warnings encountered during transformation
    pub warnings: Vec<TransformWarning>,
}

/// A warning encountered during transformation.
#[derive(Debug, Clone)]
pub struct TransformWarning {
    /// Warning message
    pub message: String,
    /// Line number where the warning occurred
    pub line: Option<u32>,
    /// Column number where the warning occurred
    pub column: Option<u32>,
}

impl TransformWarning {
    /// Create a new warning with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    /// Add location information to the warning.
    pub fn with_location(mut self, line: u32, column: u32) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}

/// Transform an AST File into a semantic model.
pub fn transform(file: &File) -> Result<TransformResult, ParseError> {
    let mut result = TransformResult {
        contexts: Vec::new(),
        context_maps: Vec::new(),
        warnings: Vec::new(),
    };

    // First pass: transform all contexts
    let mut context_lookup: HashMap<String, usize> = HashMap::new();

    for context_decl in &file.contexts {
        let ctx = transform_context(context_decl, &mut result.warnings)?;
        context_lookup.insert(ctx.name().to_string(), result.contexts.len());
        result.contexts.push(ctx);
    }

    // Second pass: transform context maps
    for map_decl in &file.context_maps {
        let ctx_map = transform_context_map(map_decl, &context_lookup, &mut result.warnings)?;
        result.context_maps.push(ctx_map);
    }

    Ok(result)
}

/// Transform a single context declaration into a BoundedContext.
fn transform_context(
    decl: &ContextDecl,
    warnings: &mut Vec<TransformWarning>,
) -> Result<BoundedContext, ParseError> {
    let mut ctx = BoundedContext::new(&decl.name);

    // Track object names to IDs for morphism resolution
    let mut object_lookup: HashMap<String, sketchddd_core::sketch::ObjectId> = HashMap::new();

    // 1. Add all declared objects first
    for obj in &decl.objects {
        let id = ctx.sketch_mut().add_object(&obj.name);
        object_lookup.insert(obj.name.clone(), id);
    }

    // 2. Add entities (objects with identity)
    for entity in &decl.entities {
        let id = ctx.add_entity(&entity.name);
        object_lookup.insert(entity.name.clone(), id);
        // Note: Entity fields could create additional morphisms if needed
    }

    // 3. Add value objects
    for vo in &decl.value_objects {
        let id = transform_value_object(&mut ctx, vo, &object_lookup, warnings)?;
        object_lookup.insert(vo.name.clone(), id);
    }

    // 4. Add enums (sum types)
    for enum_decl in &decl.enums {
        let id = transform_enum(&mut ctx, enum_decl)?;
        object_lookup.insert(enum_decl.name.clone(), id);
    }

    // 5. Add morphisms
    for morph in &decl.morphisms {
        transform_morphism(&mut ctx, morph, &mut object_lookup, warnings)?;
    }

    // 6. Define aggregates
    for agg in &decl.aggregates {
        transform_aggregate(&mut ctx, agg, &object_lookup, warnings)?;
    }

    // 7. Add equations (business rules)
    for eq in &decl.equations {
        transform_equation(&mut ctx, eq, &object_lookup, warnings)?;
    }

    Ok(ctx)
}

/// Transform a value object declaration.
fn transform_value_object(
    ctx: &mut BoundedContext,
    vo: &ValueObjectDecl,
    object_lookup: &HashMap<String, sketchddd_core::sketch::ObjectId>,
    warnings: &mut Vec<TransformWarning>,
) -> Result<sketchddd_core::sketch::ObjectId, ParseError> {
    // Get component types from fields
    let mut component_ids = Vec::new();

    for field in &vo.fields {
        let type_name = field.type_expr.base_name();
        if let Some(&id) = object_lookup.get(type_name) {
            component_ids.push(id);
        } else {
            // Type not found - add as a new object
            warnings.push(
                TransformWarning::new(format!(
                    "Type '{}' for field '{}' in value object '{}' not declared, adding implicitly",
                    type_name, field.name, vo.name
                ))
                .with_location(field.span.line, field.span.column),
            );
        }
    }

    if component_ids.is_empty() {
        // Simple value object without explicit components
        Ok(ctx.add_value_object(&vo.name))
    } else {
        Ok(ctx.add_value_object_with_components(&vo.name, &component_ids))
    }
}

/// Transform an enum declaration.
fn transform_enum(
    ctx: &mut BoundedContext,
    enum_decl: &EnumDecl,
) -> Result<sketchddd_core::sketch::ObjectId, ParseError> {
    let variants: Vec<String> = enum_decl.variants.iter().map(|v| v.name.clone()).collect();
    Ok(ctx.add_enum(&enum_decl.name, variants))
}

/// Transform a morphism declaration.
fn transform_morphism(
    ctx: &mut BoundedContext,
    morph: &MorphismDecl,
    object_lookup: &mut HashMap<String, sketchddd_core::sketch::ObjectId>,
    warnings: &mut Vec<TransformWarning>,
) -> Result<sketchddd_core::sketch::MorphismId, ParseError> {
    // Resolve or create source type
    let source_name = morph.source.base_name();
    let source_id = resolve_or_create_object(ctx, source_name, object_lookup, warnings, &morph.span);

    // Resolve or create target type
    let target_name = morph.target.base_name();
    let target_id = resolve_or_create_object(ctx, target_name, object_lookup, warnings, &morph.span);

    // Add the morphism
    let morph_id = ctx
        .sketch_mut()
        .graph
        .add_morphism(&morph.name, source_id, target_id);

    Ok(morph_id)
}

/// Resolve an object by name or create it if it doesn't exist.
fn resolve_or_create_object(
    ctx: &mut BoundedContext,
    name: &str,
    object_lookup: &mut HashMap<String, sketchddd_core::sketch::ObjectId>,
    warnings: &mut Vec<TransformWarning>,
    span: &crate::ast::Span,
) -> sketchddd_core::sketch::ObjectId {
    if let Some(&id) = object_lookup.get(name) {
        id
    } else {
        warnings.push(
            TransformWarning::new(format!(
                "Object '{}' referenced but not declared, adding implicitly",
                name
            ))
            .with_location(span.line, span.column),
        );
        let id = ctx.sketch_mut().add_object(name);
        object_lookup.insert(name.to_string(), id);
        id
    }
}

/// Transform an aggregate declaration.
fn transform_aggregate(
    ctx: &mut BoundedContext,
    agg: &AggregateDecl,
    object_lookup: &HashMap<String, sketchddd_core::sketch::ObjectId>,
    warnings: &mut Vec<TransformWarning>,
) -> Result<(), ParseError> {
    // Get the root object
    let root_name = agg.root.as_ref().unwrap_or(&agg.name);
    let root_id = object_lookup.get(root_name).ok_or_else(|| {
        ParseError::new(format!(
            "Aggregate root '{}' not found in context",
            root_name
        ))
        .with_location(agg.span.line, agg.span.column)
    })?;

    // Get contained objects
    let mut member_ids = Vec::new();
    for member_name in &agg.contains {
        if let Some(&id) = object_lookup.get(member_name) {
            member_ids.push(id);
        } else {
            warnings.push(
                TransformWarning::new(format!(
                    "Aggregate member '{}' not found in context",
                    member_name
                ))
                .with_location(agg.span.line, agg.span.column),
            );
        }
    }

    ctx.define_aggregate_with_members(&agg.name, *root_id, &member_ids);

    Ok(())
}

/// Transform an equation declaration.
fn transform_equation(
    ctx: &mut BoundedContext,
    eq: &EquationDecl,
    object_lookup: &HashMap<String, sketchddd_core::sketch::ObjectId>,
    warnings: &mut Vec<TransformWarning>,
) -> Result<(), ParseError> {
    // Convert AST paths to semantic model paths
    let lhs = transform_path(&eq.lhs, object_lookup, warnings)?;
    let rhs = transform_path(&eq.rhs, object_lookup, warnings)?;

    // Create path equation
    let equation = sketchddd_core::sketch::PathEquation::new(
        eq.name.as_deref().unwrap_or(""),
        lhs,
        rhs,
    );

    ctx.add_path_equation(eq.name.as_deref().unwrap_or("anonymous"), equation);

    Ok(())
}

/// Transform an AST path to a semantic model path.
fn transform_path(
    path: &crate::ast::Path,
    object_lookup: &HashMap<String, sketchddd_core::sketch::ObjectId>,
    _warnings: &mut Vec<TransformWarning>,
) -> Result<sketchddd_core::sketch::Path, ParseError> {
    if path.components.is_empty() {
        return Err(ParseError::new("Empty path in equation"));
    }

    // The first component should be an object
    let first = &path.components[0];
    let start_id = object_lookup.get(first).ok_or_else(|| {
        ParseError::new(format!("Object '{}' not found for path start", first))
    })?;

    // For now, create an identity path from the start object
    // TODO: Resolve morphism paths properly when we have morphism lookup
    Ok(sketchddd_core::sketch::Path::identity(*start_id))
}

/// Transform a context map declaration.
fn transform_context_map(
    map_decl: &ContextMapDecl,
    context_lookup: &HashMap<String, usize>,
    warnings: &mut Vec<TransformWarning>,
) -> Result<NamedContextMap, ParseError> {
    // Validate source context exists
    if !context_lookup.contains_key(&map_decl.source_context) {
        warnings.push(
            TransformWarning::new(format!(
                "Source context '{}' not found in file",
                map_decl.source_context
            ))
            .with_location(map_decl.span.line, map_decl.span.column),
        );
    }

    // Validate target context exists
    if !context_lookup.contains_key(&map_decl.target_context) {
        warnings.push(
            TransformWarning::new(format!(
                "Target context '{}' not found in file",
                map_decl.target_context
            ))
            .with_location(map_decl.span.line, map_decl.span.column),
        );
    }

    // Parse the relationship pattern
    let pattern = map_decl
        .pattern
        .as_ref()
        .map(|p| parse_relationship_pattern(p))
        .transpose()?
        .unwrap_or(RelationshipPattern::Partnership);

    // Create the context map with named mappings
    let mut ctx_map = NamedContextMap::new(
        &map_decl.name,
        &map_decl.source_context,
        &map_decl.target_context,
        pattern,
    );

    // Add object mappings
    for obj_map in &map_decl.object_mappings {
        ctx_map.add_object_mapping(NamedObjectMapping {
            source: obj_map.source.clone(),
            target: obj_map.target.clone(),
            description: obj_map.description.clone(),
        });
    }

    // Add morphism mappings
    for morph_map in &map_decl.morphism_mappings {
        ctx_map.add_morphism_mapping(NamedMorphismMapping {
            source: morph_map.source.clone(),
            target: morph_map.target.clone(),
            description: morph_map.description.clone(),
        });
    }

    Ok(ctx_map)
}

/// Parse a relationship pattern string into the enum.
fn parse_relationship_pattern(pattern: &str) -> Result<RelationshipPattern, ParseError> {
    match pattern {
        "Partnership" => Ok(RelationshipPattern::Partnership),
        "CustomerSupplier" => Ok(RelationshipPattern::CustomerSupplier),
        "Conformist" => Ok(RelationshipPattern::Conformist),
        "AntiCorruptionLayer" | "ACL" => Ok(RelationshipPattern::AntiCorruptionLayer),
        "SeparateWays" => Ok(RelationshipPattern::SeparateWays),
        "PublishedLanguage" => Ok(RelationshipPattern::PublishedLanguage),
        "OpenHostService" | "OHS" => Ok(RelationshipPattern::OpenHostService),
        "SharedKernel" => Ok(RelationshipPattern::SharedKernel),
        _ => Err(ParseError::new(format!(
            "Unknown relationship pattern: '{}'",
            pattern
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_file;

    #[test]
    fn test_transform_empty_context() {
        let source = r#"
            context Commerce {
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 1);
        assert_eq!(result.contexts[0].name(), "Commerce");
    }

    #[test]
    fn test_transform_context_with_objects() {
        let source = r#"
            context Commerce {
                objects { Customer, Order, LineItem }
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 1);
        let ctx = &result.contexts[0];
        assert_eq!(ctx.graph().objects().count(), 3);
    }

    #[test]
    fn test_transform_context_with_entities() {
        let source = r#"
            context Commerce {
                entity Customer {
                    id: UUID
                    name: String
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 1);
        let ctx = &result.contexts[0];
        assert_eq!(ctx.entities().len(), 1);
        // Entity should have an identity morphism
        assert!(ctx.get_entity_identity(ctx.entities()[0]).is_some());
    }

    #[test]
    fn test_transform_context_with_value_objects() {
        let source = r#"
            context Commerce {
                objects { Decimal, Currency }
                value Money {
                    amount: Decimal
                    currency: Currency
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 1);
        let ctx = &result.contexts[0];
        assert_eq!(ctx.value_objects().len(), 1);
    }

    #[test]
    fn test_transform_context_with_enum() {
        let source = r#"
            context Commerce {
                enum OrderStatus = Pending | Confirmed | Shipped
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 1);
        let ctx = &result.contexts[0];
        // Should have the enum as a colimit
        assert_eq!(ctx.sketch().colimits.len(), 1);
    }

    #[test]
    fn test_transform_context_with_morphisms() {
        let source = r#"
            context Commerce {
                objects { Customer, Order }
                morphisms {
                    placedBy: Order -> Customer
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 1);
        let ctx = &result.contexts[0];
        // 2 objects + 1 morphism (placedBy)
        assert_eq!(ctx.graph().morphisms().count(), 1);
    }

    #[test]
    fn test_transform_context_with_aggregate() {
        let source = r#"
            context Commerce {
                entity Order
                entity LineItem
                aggregate OrderAggregate {
                    root: Order
                    contains: [LineItem]
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 1);
        let ctx = &result.contexts[0];
        assert_eq!(ctx.aggregate_roots().len(), 1);
    }

    #[test]
    fn test_transform_context_map() {
        let source = r#"
            context Commerce {
                objects { Order, Customer }
            }

            context Shipping {
                objects { Shipment, Recipient }
            }

            map CommerceToShipping: Commerce -> Shipping {
                pattern: CustomerSupplier
                mappings {
                    Order -> Shipment
                    Customer -> Recipient
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 2);
        assert_eq!(result.context_maps.len(), 1);

        let map = &result.context_maps[0];
        assert_eq!(map.name(), "CommerceToShipping");
        assert_eq!(map.source_context(), "Commerce");
        assert_eq!(map.target_context(), "Shipping");
        assert_eq!(map.pattern(), RelationshipPattern::CustomerSupplier);
        assert_eq!(map.object_mappings().len(), 2);
    }

    #[test]
    fn test_transform_full_example() {
        let source = r#"
            context Commerce {
                objects { Product }

                entity Customer
                entity Order
                entity LineItem

                morphisms {
                    placedBy: Order -> Customer
                    items: Order -> LineItem
                    product: LineItem -> Product
                }

                value Money {
                    amount: Decimal
                    currency: Currency
                }

                enum OrderStatus = Pending | Confirmed | Shipped | Cancelled

                aggregate OrderAggregate {
                    root: Order
                    contains: [LineItem]
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        assert_eq!(result.contexts.len(), 1);
        let ctx = &result.contexts[0];

        // Check entities
        assert_eq!(ctx.entities().len(), 3); // Customer, Order, LineItem

        // Check value objects
        assert_eq!(ctx.value_objects().len(), 1); // Money

        // Check aggregates
        assert_eq!(ctx.aggregate_roots().len(), 1); // Order

        // Check colimits (enums)
        assert_eq!(ctx.sketch().colimits.len(), 1); // OrderStatus
    }

    #[test]
    fn test_transform_implicit_object_warning() {
        let source = r#"
            context Commerce {
                morphisms {
                    placedBy: Order -> Customer
                }
            }
        "#;
        let file = parse_file(source).unwrap();
        let result = transform(&file).unwrap();

        // Should have warnings about implicit objects
        assert!(!result.warnings.is_empty());
        assert!(result.warnings.iter().any(|w| w.message.contains("Order")));
        assert!(result.warnings.iter().any(|w| w.message.contains("Customer")));
    }

    #[test]
    fn test_transform_all_relationship_patterns() {
        let patterns = [
            "Partnership",
            "CustomerSupplier",
            "Conformist",
            "AntiCorruptionLayer",
            "SeparateWays",
            "PublishedLanguage",
            "OpenHostService",
            "SharedKernel",
        ];

        for pattern in patterns {
            let source = format!(
                r#"
                map TestMap: A -> B {{
                    pattern: {}
                }}
            "#,
                pattern
            );
            let file = parse_file(&source).unwrap();
            let result = transform(&file).unwrap();
            assert_eq!(result.context_maps.len(), 1);
        }
    }
}
