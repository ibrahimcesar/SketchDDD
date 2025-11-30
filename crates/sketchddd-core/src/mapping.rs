//! Context maps as sketch morphisms between bounded contexts.
//!
//! In category theory, a context map is a **sketch morphism** (functor) that maps
//! objects and morphisms from one sketch to another while preserving structure.
//! For a functor F: C → D to be valid, it must satisfy:
//!
//! 1. **Object mapping**: F maps objects in C to objects in D
//! 2. **Morphism mapping**: F maps morphisms f: A → B to F(f): F(A) → F(B)
//! 3. **Identity preservation**: F(id_A) = id_{F(A)}
//! 4. **Composition preservation**: F(g ∘ f) = F(g) ∘ F(f)

use crate::sketch::{MorphismId, ObjectId};
use serde::{Deserialize, Serialize};

/// The type of relationship between two bounded contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipPattern {
    /// Both teams collaborate closely and evolve together
    Partnership,

    /// Upstream context provides, downstream consumes
    CustomerSupplier,

    /// Downstream adopts upstream's model exactly
    Conformist,

    /// Downstream translates upstream's model through a layer
    AntiCorruptionLayer,

    /// No integration needed between contexts
    SeparateWays,

    /// Shared formal language for integration
    PublishedLanguage,

    /// Context exposes services for others to consume
    OpenHostService,

    /// Two contexts share a common subset
    SharedKernel,
}

/// A mapping of a single object from source to target context.
///
/// In categorical terms, this represents the object part of a functor:
/// F_0: Obj(C) → Obj(D)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectMapping {
    /// Object in the source context
    pub source: ObjectId,

    /// Object in the target context
    pub target: ObjectId,

    /// Optional description of the mapping
    pub description: Option<String>,
}

/// A mapping of a single morphism from source to target context.
///
/// In categorical terms, this represents the morphism part of a functor:
/// F_1: Mor(C) → Mor(D)
///
/// For a valid functor, if f: A → B in C, then F(f): F(A) → F(B) in D.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MorphismMapping {
    /// Morphism in the source context
    pub source: MorphismId,

    /// Morphism in the target context
    pub target: MorphismId,

    /// Optional description of the mapping
    pub description: Option<String>,
}

/// A context map describing the relationship between two bounded contexts.
///
/// In category theory terms, this is a sketch morphism (functor)
/// that maps objects and morphisms from one sketch to another while
/// preserving the categorical structure.
///
/// # Functorial Laws
///
/// A valid context map must satisfy:
/// - **Identity**: Identity morphisms map to identity morphisms
/// - **Composition**: F(g ∘ f) = F(g) ∘ F(f)
/// - **Source/Target**: If f: A → B, then F(f): F(A) → F(B)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMap {
    /// Name of this context map
    pub name: String,

    /// Name of the source (upstream) context
    pub source_context: String,

    /// Name of the target (downstream) context
    pub target_context: String,

    /// The relationship pattern
    pub pattern: RelationshipPattern,

    /// Object mappings (F_0: Obj(C) → Obj(D))
    pub object_mappings: Vec<ObjectMapping>,

    /// Morphism mappings (F_1: Mor(C) → Mor(D))
    pub morphism_mappings: Vec<MorphismMapping>,
}

impl ContextMap {
    /// Create a new context map.
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
        pattern: RelationshipPattern,
    ) -> Self {
        Self {
            name: name.into(),
            source_context: source.into(),
            target_context: target.into(),
            pattern,
            object_mappings: Vec::new(),
            morphism_mappings: Vec::new(),
        }
    }

    /// Add a mapping between objects.
    pub fn map_object(&mut self, source: ObjectId, target: ObjectId) {
        self.object_mappings.push(ObjectMapping {
            source,
            target,
            description: None,
        });
    }

    /// Add a mapping with description.
    pub fn map_object_with_description(
        &mut self,
        source: ObjectId,
        target: ObjectId,
        description: impl Into<String>,
    ) {
        self.object_mappings.push(ObjectMapping {
            source,
            target,
            description: Some(description.into()),
        });
    }

    /// Check if the source context is upstream (provider).
    pub fn source_is_upstream(&self) -> bool {
        matches!(
            self.pattern,
            RelationshipPattern::CustomerSupplier
                | RelationshipPattern::Conformist
                | RelationshipPattern::AntiCorruptionLayer
                | RelationshipPattern::OpenHostService
        )
    }

    /// Check if this is a symmetric relationship.
    pub fn is_symmetric(&self) -> bool {
        matches!(
            self.pattern,
            RelationshipPattern::Partnership | RelationshipPattern::SharedKernel
        )
    }

    /// Add a mapping between morphisms.
    pub fn map_morphism(&mut self, source: MorphismId, target: MorphismId) {
        self.morphism_mappings.push(MorphismMapping {
            source,
            target,
            description: None,
        });
    }

    /// Add a morphism mapping with description.
    pub fn map_morphism_with_description(
        &mut self,
        source: MorphismId,
        target: MorphismId,
        description: impl Into<String>,
    ) {
        self.morphism_mappings.push(MorphismMapping {
            source,
            target,
            description: Some(description.into()),
        });
    }

    /// Get the mapped object for a source object, if it exists.
    pub fn get_object_mapping(&self, source: ObjectId) -> Option<ObjectId> {
        self.object_mappings
            .iter()
            .find(|m| m.source == source)
            .map(|m| m.target)
    }

    /// Get the mapped morphism for a source morphism, if it exists.
    pub fn get_morphism_mapping(&self, source: MorphismId) -> Option<MorphismId> {
        self.morphism_mappings
            .iter()
            .find(|m| m.source == source)
            .map(|m| m.target)
    }

    /// Check if this relationship requires translation (ACL).
    pub fn requires_translation(&self) -> bool {
        matches!(self.pattern, RelationshipPattern::AntiCorruptionLayer)
    }

    /// Check if this is an integration relationship (not SeparateWays).
    pub fn has_integration(&self) -> bool {
        !matches!(self.pattern, RelationshipPattern::SeparateWays)
    }

    /// Get the directionality description for this pattern.
    pub fn directionality(&self) -> &'static str {
        match self.pattern {
            RelationshipPattern::Partnership => "bidirectional",
            RelationshipPattern::CustomerSupplier => "upstream → downstream",
            RelationshipPattern::Conformist => "upstream → downstream",
            RelationshipPattern::AntiCorruptionLayer => "upstream → downstream (translated)",
            RelationshipPattern::SeparateWays => "none",
            RelationshipPattern::PublishedLanguage => "upstream → downstream (via shared language)",
            RelationshipPattern::OpenHostService => "upstream → downstream (via services)",
            RelationshipPattern::SharedKernel => "bidirectional (shared)",
        }
    }
}

/// Errors that can occur during functorial consistency checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctorError {
    /// A morphism's source object is not mapped.
    UnmappedSource {
        morphism: MorphismId,
        source_object: ObjectId,
    },

    /// A morphism's target object is not mapped.
    UnmappedTarget {
        morphism: MorphismId,
        target_object: ObjectId,
    },

    /// The mapped morphism has incorrect source.
    InconsistentSource {
        source_morphism: MorphismId,
        expected_target_source: ObjectId,
        actual_target_source: ObjectId,
    },

    /// The mapped morphism has incorrect target.
    InconsistentTarget {
        source_morphism: MorphismId,
        expected_target_target: ObjectId,
        actual_target_target: ObjectId,
    },

    /// An identity morphism is not mapped to an identity morphism.
    IdentityNotPreserved {
        source_identity: MorphismId,
        target_morphism: MorphismId,
    },
}

impl std::fmt::Display for FunctorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctorError::UnmappedSource { morphism, source_object } => {
                write!(
                    f,
                    "Morphism {:?} has source object {:?} which is not mapped",
                    morphism, source_object
                )
            }
            FunctorError::UnmappedTarget { morphism, target_object } => {
                write!(
                    f,
                    "Morphism {:?} has target object {:?} which is not mapped",
                    morphism, target_object
                )
            }
            FunctorError::InconsistentSource {
                source_morphism,
                expected_target_source,
                actual_target_source,
            } => {
                write!(
                    f,
                    "Mapped morphism for {:?} has source {:?} but expected {:?}",
                    source_morphism, actual_target_source, expected_target_source
                )
            }
            FunctorError::InconsistentTarget {
                source_morphism,
                expected_target_target,
                actual_target_target,
            } => {
                write!(
                    f,
                    "Mapped morphism for {:?} has target {:?} but expected {:?}",
                    source_morphism, actual_target_target, expected_target_target
                )
            }
            FunctorError::IdentityNotPreserved {
                source_identity,
                target_morphism,
            } => {
                write!(
                    f,
                    "Identity morphism {:?} is mapped to non-identity {:?}",
                    source_identity, target_morphism
                )
            }
        }
    }
}

impl std::error::Error for FunctorError {}

/// Result of checking functorial consistency.
#[derive(Debug, Clone)]
pub struct FunctorCheckResult {
    /// Whether the mapping is consistent (no errors).
    pub is_valid: bool,

    /// Errors found during validation.
    pub errors: Vec<FunctorError>,
}

impl FunctorCheckResult {
    /// Create a valid result with no errors.
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    /// Create an invalid result with the given errors.
    pub fn invalid(errors: Vec<FunctorError>) -> Self {
        Self {
            is_valid: false,
            errors,
        }
    }
}

use crate::sketch::Graph;

/// Check functorial consistency of a context map against source and target graphs.
///
/// This verifies that the mapping satisfies the functor laws:
/// 1. For each mapped morphism f: A → B, we have F(A) and F(B) defined
/// 2. F(f): F(A) → F(B) (source/target preservation)
/// 3. Identity morphisms map to identity morphisms (if both are mapped)
///
/// # Arguments
///
/// * `context_map` - The context map to validate
/// * `source_graph` - The graph of the source bounded context
/// * `target_graph` - The graph of the target bounded context
///
/// # Returns
///
/// A `FunctorCheckResult` indicating whether the mapping is consistent.
pub fn check_functorial_consistency(
    context_map: &ContextMap,
    source_graph: &Graph,
    target_graph: &Graph,
) -> FunctorCheckResult {
    let mut errors = Vec::new();

    // Check each morphism mapping
    for mapping in &context_map.morphism_mappings {
        // Get the source morphism
        let Some(source_morphism) = source_graph.get_morphism(mapping.source) else {
            continue; // Skip if source morphism doesn't exist (could be separate validation)
        };

        // Get the target morphism
        let Some(target_morphism) = target_graph.get_morphism(mapping.target) else {
            continue; // Skip if target morphism doesn't exist
        };

        // Check that source object is mapped
        let mapped_source = context_map.get_object_mapping(source_morphism.source);
        if mapped_source.is_none() {
            errors.push(FunctorError::UnmappedSource {
                morphism: mapping.source,
                source_object: source_morphism.source,
            });
        }

        // Check that target object is mapped
        let mapped_target = context_map.get_object_mapping(source_morphism.target);
        if mapped_target.is_none() {
            errors.push(FunctorError::UnmappedTarget {
                morphism: mapping.source,
                target_object: source_morphism.target,
            });
        }

        // Check source/target preservation: F(f): F(A) → F(B)
        if let Some(expected_source) = mapped_source {
            if target_morphism.source != expected_source {
                errors.push(FunctorError::InconsistentSource {
                    source_morphism: mapping.source,
                    expected_target_source: expected_source,
                    actual_target_source: target_morphism.source,
                });
            }
        }

        if let Some(expected_target) = mapped_target {
            if target_morphism.target != expected_target {
                errors.push(FunctorError::InconsistentTarget {
                    source_morphism: mapping.source,
                    expected_target_target: expected_target,
                    actual_target_target: target_morphism.target,
                });
            }
        }

        // Check identity preservation
        if source_morphism.is_identity && !target_morphism.is_identity {
            errors.push(FunctorError::IdentityNotPreserved {
                source_identity: mapping.source,
                target_morphism: mapping.target,
            });
        }
    }

    if errors.is_empty() {
        FunctorCheckResult::valid()
    } else {
        FunctorCheckResult::invalid(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sketch::Graph;

    // =============================================================
    // Tests for all 8 DDD Relationship Patterns
    // =============================================================

    #[test]
    fn test_partnership_pattern() {
        let map = ContextMap::new(
            "SalesMarketing",
            "Sales",
            "Marketing",
            RelationshipPattern::Partnership,
        );

        assert_eq!(map.pattern, RelationshipPattern::Partnership);
        assert!(map.is_symmetric());
        assert!(!map.source_is_upstream());
        assert!(map.has_integration());
        assert_eq!(map.directionality(), "bidirectional");
    }

    #[test]
    fn test_customer_supplier_pattern() {
        let map = ContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );

        assert_eq!(map.pattern, RelationshipPattern::CustomerSupplier);
        assert!(!map.is_symmetric());
        assert!(map.source_is_upstream());
        assert!(map.has_integration());
        assert!(!map.requires_translation());
        assert_eq!(map.directionality(), "upstream → downstream");
    }

    #[test]
    fn test_conformist_pattern() {
        let map = ContextMap::new(
            "ReportingToCore",
            "CoreDomain",
            "Reporting",
            RelationshipPattern::Conformist,
        );

        assert_eq!(map.pattern, RelationshipPattern::Conformist);
        assert!(!map.is_symmetric());
        assert!(map.source_is_upstream());
        assert!(map.has_integration());
        assert!(!map.requires_translation());
        assert_eq!(map.directionality(), "upstream → downstream");
    }

    #[test]
    fn test_anti_corruption_layer_pattern() {
        let map = ContextMap::new(
            "LegacyIntegration",
            "LegacySystem",
            "NewSystem",
            RelationshipPattern::AntiCorruptionLayer,
        );

        assert_eq!(map.pattern, RelationshipPattern::AntiCorruptionLayer);
        assert!(!map.is_symmetric());
        assert!(map.source_is_upstream());
        assert!(map.has_integration());
        assert!(map.requires_translation());
        assert_eq!(map.directionality(), "upstream → downstream (translated)");
    }

    #[test]
    fn test_separate_ways_pattern() {
        let map = ContextMap::new(
            "IndependentContexts",
            "ContextA",
            "ContextB",
            RelationshipPattern::SeparateWays,
        );

        assert_eq!(map.pattern, RelationshipPattern::SeparateWays);
        assert!(!map.is_symmetric());
        assert!(!map.source_is_upstream());
        assert!(!map.has_integration());
        assert!(!map.requires_translation());
        assert_eq!(map.directionality(), "none");
    }

    #[test]
    fn test_published_language_pattern() {
        let map = ContextMap::new(
            "APIIntegration",
            "CoreAPI",
            "Consumer",
            RelationshipPattern::PublishedLanguage,
        );

        assert_eq!(map.pattern, RelationshipPattern::PublishedLanguage);
        assert!(!map.is_symmetric());
        assert!(!map.source_is_upstream());
        assert!(map.has_integration());
        assert!(!map.requires_translation());
        assert_eq!(
            map.directionality(),
            "upstream → downstream (via shared language)"
        );
    }

    #[test]
    fn test_open_host_service_pattern() {
        let map = ContextMap::new(
            "ServiceExposure",
            "ServiceProvider",
            "ServiceConsumer",
            RelationshipPattern::OpenHostService,
        );

        assert_eq!(map.pattern, RelationshipPattern::OpenHostService);
        assert!(!map.is_symmetric());
        assert!(map.source_is_upstream());
        assert!(map.has_integration());
        assert!(!map.requires_translation());
        assert_eq!(map.directionality(), "upstream → downstream (via services)");
    }

    #[test]
    fn test_shared_kernel_pattern() {
        let map = ContextMap::new(
            "SharedIdentity",
            "UserManagement",
            "Authentication",
            RelationshipPattern::SharedKernel,
        );

        assert_eq!(map.pattern, RelationshipPattern::SharedKernel);
        assert!(map.is_symmetric());
        assert!(!map.source_is_upstream());
        assert!(map.has_integration());
        assert!(!map.requires_translation());
        assert_eq!(map.directionality(), "bidirectional (shared)");
    }

    // =============================================================
    // Tests for Object and Morphism Mappings
    // =============================================================

    #[test]
    fn test_add_object_mappings() {
        let mut map = ContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );

        map.map_object(ObjectId(0), ObjectId(10));
        map.map_object_with_description(ObjectId(1), ObjectId(11), "Order -> Shipment");

        assert_eq!(map.object_mappings.len(), 2);
        assert_eq!(map.get_object_mapping(ObjectId(0)), Some(ObjectId(10)));
        assert_eq!(map.get_object_mapping(ObjectId(1)), Some(ObjectId(11)));
        assert_eq!(map.get_object_mapping(ObjectId(99)), None);
    }

    #[test]
    fn test_add_morphism_mappings() {
        let mut map = ContextMap::new(
            "OrderToFulfillment",
            "OrderContext",
            "FulfillmentContext",
            RelationshipPattern::CustomerSupplier,
        );

        map.map_morphism(MorphismId(0), MorphismId(10));
        map.map_morphism_with_description(
            MorphismId(1),
            MorphismId(11),
            "placedBy -> assignedTo",
        );

        assert_eq!(map.morphism_mappings.len(), 2);
        assert_eq!(
            map.get_morphism_mapping(MorphismId(0)),
            Some(MorphismId(10))
        );
        assert_eq!(
            map.get_morphism_mapping(MorphismId(1)),
            Some(MorphismId(11))
        );
        assert_eq!(map.get_morphism_mapping(MorphismId(99)), None);
    }

    // =============================================================
    // Tests for Functorial Consistency Checks
    // =============================================================

    fn create_simple_source_graph() -> Graph {
        let mut graph = Graph::new();
        let a = graph.add_object("A");
        let b = graph.add_object("B");
        graph.add_morphism("f", a, b);
        graph.add_identity_morphism(a);
        graph
    }

    fn create_simple_target_graph() -> Graph {
        let mut graph = Graph::new();
        let fa = graph.add_object("FA");
        let fb = graph.add_object("FB");
        graph.add_morphism("Ff", fa, fb);
        graph.add_identity_morphism(fa);
        graph
    }

    #[test]
    fn test_valid_functorial_mapping() {
        let source = create_simple_source_graph();
        let target = create_simple_target_graph();

        let mut map = ContextMap::new(
            "ValidMapping",
            "Source",
            "Target",
            RelationshipPattern::Conformist,
        );

        // Map objects: A -> FA, B -> FB
        map.map_object(ObjectId(0), ObjectId(0)); // A -> FA
        map.map_object(ObjectId(1), ObjectId(1)); // B -> FB

        // Map morphism: f -> Ff
        map.map_morphism(MorphismId(0), MorphismId(0)); // f -> Ff

        // Map identity: id_A -> id_FA
        map.map_morphism(MorphismId(1), MorphismId(1)); // id_A -> id_FA

        let result = check_functorial_consistency(&map, &source, &target);
        assert!(result.is_valid, "Expected valid result: {:?}", result.errors);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_unmapped_source_object() {
        let source = create_simple_source_graph();
        let target = create_simple_target_graph();

        let mut map = ContextMap::new(
            "PartialMapping",
            "Source",
            "Target",
            RelationshipPattern::Conformist,
        );

        // Only map B -> FB, not A
        map.map_object(ObjectId(1), ObjectId(1)); // B -> FB

        // Try to map morphism f: A -> B, but A is not mapped
        map.map_morphism(MorphismId(0), MorphismId(0));

        let result = check_functorial_consistency(&map, &source, &target);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            FunctorError::UnmappedSource {
                morphism: MorphismId(0),
                source_object: ObjectId(0)
            }
        )));
    }

    #[test]
    fn test_unmapped_target_object() {
        let source = create_simple_source_graph();
        let target = create_simple_target_graph();

        let mut map = ContextMap::new(
            "PartialMapping",
            "Source",
            "Target",
            RelationshipPattern::Conformist,
        );

        // Only map A -> FA, not B
        map.map_object(ObjectId(0), ObjectId(0)); // A -> FA

        // Try to map morphism f: A -> B, but B is not mapped
        map.map_morphism(MorphismId(0), MorphismId(0));

        let result = check_functorial_consistency(&map, &source, &target);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            FunctorError::UnmappedTarget {
                morphism: MorphismId(0),
                target_object: ObjectId(1)
            }
        )));
    }

    #[test]
    fn test_inconsistent_source() {
        let source = create_simple_source_graph();

        // Create a target graph where morphism has different source
        let mut target = Graph::new();
        let _fa = target.add_object("FA");
        let fb = target.add_object("FB");
        let fc = target.add_object("FC");
        target.add_morphism("Ff", fc, fb); // Ff: FC -> FB (wrong source)

        let mut map = ContextMap::new(
            "InconsistentMapping",
            "Source",
            "Target",
            RelationshipPattern::Conformist,
        );

        // Map objects correctly
        map.map_object(ObjectId(0), ObjectId(0)); // A -> FA
        map.map_object(ObjectId(1), ObjectId(1)); // B -> FB

        // Map morphism f: A->B to Ff: FC->FB (source mismatch)
        map.map_morphism(MorphismId(0), MorphismId(0));

        let result = check_functorial_consistency(&map, &source, &target);
        assert!(!result.is_valid);
        assert!(result
            .errors
            .iter()
            .any(|e| matches!(e, FunctorError::InconsistentSource { .. })));
    }

    #[test]
    fn test_identity_not_preserved() {
        let source = create_simple_source_graph();

        // Create target with non-identity morphism
        let mut target = Graph::new();
        let fa = target.add_object("FA");
        let fb = target.add_object("FB");
        target.add_morphism("not_identity", fa, fb); // Regular morphism, not identity

        let mut map = ContextMap::new(
            "IdentityViolation",
            "Source",
            "Target",
            RelationshipPattern::Conformist,
        );

        map.map_object(ObjectId(0), ObjectId(0)); // A -> FA

        // Map identity morphism to non-identity
        map.map_morphism(MorphismId(1), MorphismId(0)); // id_A -> not_identity

        let result = check_functorial_consistency(&map, &source, &target);
        assert!(!result.is_valid);
        assert!(result
            .errors
            .iter()
            .any(|e| matches!(e, FunctorError::IdentityNotPreserved { .. })));
    }

    #[test]
    fn test_empty_mapping_is_valid() {
        let source = create_simple_source_graph();
        let target = create_simple_target_graph();

        let map = ContextMap::new(
            "EmptyMapping",
            "Source",
            "Target",
            RelationshipPattern::SeparateWays,
        );

        // No mappings - vacuously valid
        let result = check_functorial_consistency(&map, &source, &target);
        assert!(result.is_valid);
    }

    // =============================================================
    // Tests for Context Map Creation
    // =============================================================

    #[test]
    fn test_create_context_map() {
        let map = ContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );

        assert_eq!(map.name, "CommerceToShipping");
        assert_eq!(map.source_context, "Commerce");
        assert_eq!(map.target_context, "Shipping");
        assert!(map.source_is_upstream());
        assert!(map.object_mappings.is_empty());
        assert!(map.morphism_mappings.is_empty());
    }

    #[test]
    fn test_mapping_description() {
        let mut map = ContextMap::new(
            "TestMap",
            "Source",
            "Target",
            RelationshipPattern::Conformist,
        );

        map.map_object_with_description(
            ObjectId(0),
            ObjectId(10),
            "Maps Order to ShippingOrder",
        );

        assert_eq!(
            map.object_mappings[0].description,
            Some("Maps Order to ShippingOrder".to_string())
        );
    }
}
