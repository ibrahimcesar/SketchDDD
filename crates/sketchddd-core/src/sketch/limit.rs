//! Limit cones for aggregates and value objects.

use super::{MorphismId, ObjectId};
use serde::{Deserialize, Serialize};

/// A projection from the apex of a limit cone to a component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projection {
    /// The morphism representing this projection
    pub morphism: MorphismId,

    /// The target object of the projection
    pub target: ObjectId,
}

/// A limit cone representing an aggregate or value object.
///
/// In category theory, a limit is a universal construction that
/// "combines" objects in a specific way. For DDD:
/// - **Aggregates**: The apex is the aggregate root, projections
///   point to contained entities
/// - **Value Objects**: The apex represents the value object,
///   projections point to component types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitCone {
    /// Name of the limit cone
    pub name: String,

    /// The apex object (root of aggregate or the value object itself)
    pub apex: ObjectId,

    /// Projections to component objects
    pub projections: Vec<Projection>,

    /// Whether this represents an aggregate (true) or value object (false)
    pub is_aggregate: bool,

    /// For aggregates: the designated root entity
    pub root: Option<ObjectId>,
}

impl LimitCone {
    /// Create a new aggregate limit cone.
    pub fn aggregate(name: impl Into<String>, apex: ObjectId, root: ObjectId) -> Self {
        Self {
            name: name.into(),
            apex,
            projections: Vec::new(),
            is_aggregate: true,
            root: Some(root),
        }
    }

    /// Create a new value object limit cone.
    pub fn value_object(name: impl Into<String>, apex: ObjectId) -> Self {
        Self {
            name: name.into(),
            apex,
            projections: Vec::new(),
            is_aggregate: false,
            root: None,
        }
    }

    /// Add a projection to the limit cone.
    pub fn add_projection(&mut self, morphism: MorphismId, target: ObjectId) {
        self.projections.push(Projection { morphism, target });
    }

    /// Get all objects that are part of this limit.
    pub fn component_objects(&self) -> impl Iterator<Item = ObjectId> + '_ {
        self.projections.iter().map(|p| p.target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_creation() {
        let apex = ObjectId(0);
        let root = ObjectId(0);
        let agg = LimitCone::aggregate("OrderAggregate", apex, root);

        assert!(agg.is_aggregate);
        assert_eq!(agg.root, Some(root));
        assert_eq!(agg.name, "OrderAggregate");
    }

    #[test]
    fn test_value_object_creation() {
        let apex = ObjectId(0);
        let vo = LimitCone::value_object("Money", apex);

        assert!(!vo.is_aggregate);
        assert_eq!(vo.root, None);
        assert_eq!(vo.name, "Money");
    }

    #[test]
    fn test_add_projections() {
        let apex = ObjectId(0);
        let mut vo = LimitCone::value_object("Money", apex);

        vo.add_projection(MorphismId(0), ObjectId(1));
        vo.add_projection(MorphismId(1), ObjectId(2));

        assert_eq!(vo.projections.len(), 2);
        assert_eq!(vo.component_objects().count(), 2);
    }
}
