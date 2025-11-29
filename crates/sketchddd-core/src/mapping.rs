//! Context maps as sketch morphisms between bounded contexts.

use crate::sketch::ObjectId;
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMapping {
    /// Object in the source context
    pub source: ObjectId,

    /// Object in the target context
    pub target: ObjectId,

    /// Optional description of the mapping
    pub description: Option<String>,
}

/// A context map describing the relationship between two bounded contexts.
///
/// In category theory terms, this is a sketch morphism (functor)
/// that maps objects and morphisms from one sketch to another.
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

    /// Object mappings
    pub object_mappings: Vec<ObjectMapping>,
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_symmetric_relationship() {
        let map = ContextMap::new(
            "SharedIdentity",
            "UserManagement",
            "Authentication",
            RelationshipPattern::SharedKernel,
        );

        assert!(map.is_symmetric());
    }

    #[test]
    fn test_add_mappings() {
        let mut map = ContextMap::new(
            "CommerceToShipping",
            "Commerce",
            "Shipping",
            RelationshipPattern::CustomerSupplier,
        );

        map.map_object(ObjectId(0), ObjectId(10));
        map.map_object_with_description(ObjectId(1), ObjectId(11), "Order -> Shipment");

        assert_eq!(map.object_mappings.len(), 2);
    }
}
