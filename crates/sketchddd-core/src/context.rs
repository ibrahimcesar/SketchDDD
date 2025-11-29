//! Bounded Context as a DDD-specific wrapper around Sketch.

use crate::sketch::{ColimitCocone, Graph, LimitCone, ObjectId, PathEquation, Sketch};
use serde::{Deserialize, Serialize};

/// A bounded context in Domain-Driven Design terms.
///
/// This wraps a Sketch with DDD-specific semantics and convenience methods.
/// A bounded context represents a linguistic boundary within which terms
/// have specific, consistent meanings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundedContext {
    /// The underlying sketch
    sketch: Sketch,

    /// Entities within this context (objects with identity)
    entities: Vec<ObjectId>,

    /// Value objects within this context (objects with structural equality)
    value_objects: Vec<ObjectId>,

    /// Aggregate roots
    aggregate_roots: Vec<ObjectId>,
}

impl BoundedContext {
    /// Create a new bounded context with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            sketch: Sketch::new(name),
            entities: Vec::new(),
            value_objects: Vec::new(),
            aggregate_roots: Vec::new(),
        }
    }

    /// Get the name of this context.
    pub fn name(&self) -> &str {
        &self.sketch.name
    }

    /// Get the underlying sketch.
    pub fn sketch(&self) -> &Sketch {
        &self.sketch
    }

    /// Get a mutable reference to the underlying sketch.
    pub fn sketch_mut(&mut self) -> &mut Sketch {
        &mut self.sketch
    }

    /// Get the graph of this context.
    pub fn graph(&self) -> &Graph {
        &self.sketch.graph
    }

    /// Add an entity to this context.
    ///
    /// An entity is an object with a unique identity that persists
    /// through time and across different representations.
    pub fn add_entity(&mut self, name: impl Into<String>) -> ObjectId {
        let id = self.sketch.add_object(name);
        self.entities.push(id);
        id
    }

    /// Add a value object to this context.
    ///
    /// A value object is defined entirely by its attributes and has
    /// no conceptual identity. Two value objects with the same
    /// attributes are considered equal.
    pub fn add_value_object(&mut self, name: impl Into<String>) -> ObjectId {
        let id = self.sketch.add_object(name);
        self.value_objects.push(id);
        id
    }

    /// Check if an object is an entity.
    pub fn is_entity(&self, id: ObjectId) -> bool {
        self.entities.contains(&id)
    }

    /// Check if an object is a value object.
    pub fn is_value_object(&self, id: ObjectId) -> bool {
        self.value_objects.contains(&id)
    }

    /// Define an aggregate with its root and contained objects.
    pub fn define_aggregate(
        &mut self,
        name: impl Into<String>,
        root: ObjectId,
    ) -> &mut LimitCone {
        self.aggregate_roots.push(root);
        let limit = LimitCone::aggregate(name, root, root);
        self.sketch.add_limit(limit);
        self.sketch.limits.last_mut().unwrap()
    }

    /// Add an enumeration to this context.
    pub fn add_enum(&mut self, name: impl Into<String>, variants: Vec<String>) -> ObjectId {
        let name_str = name.into();
        let id = self.sketch.add_object(&name_str);
        let colimit = ColimitCocone::enumeration(name_str, id, variants);
        self.sketch.add_colimit(colimit);
        id
    }

    /// Add a business rule (path equation).
    pub fn add_invariant(&mut self, name: impl Into<String>, equation: PathEquation) {
        let mut eq = equation;
        eq.name = name.into();
        self.sketch.add_equation(eq);
    }

    /// Get all entities in this context.
    pub fn entities(&self) -> &[ObjectId] {
        &self.entities
    }

    /// Get all value objects in this context.
    pub fn value_objects(&self) -> &[ObjectId] {
        &self.value_objects
    }

    /// Get all aggregate roots in this context.
    pub fn aggregate_roots(&self) -> &[ObjectId] {
        &self.aggregate_roots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_context() {
        let ctx = BoundedContext::new("Commerce");
        assert_eq!(ctx.name(), "Commerce");
    }

    #[test]
    fn test_add_entity() {
        let mut ctx = BoundedContext::new("Commerce");
        let customer = ctx.add_entity("Customer");

        assert!(ctx.is_entity(customer));
        assert!(!ctx.is_value_object(customer));
    }

    #[test]
    fn test_add_value_object() {
        let mut ctx = BoundedContext::new("Commerce");
        let money = ctx.add_value_object("Money");

        assert!(ctx.is_value_object(money));
        assert!(!ctx.is_entity(money));
    }

    #[test]
    fn test_add_enum() {
        let mut ctx = BoundedContext::new("Commerce");
        let _status = ctx.add_enum(
            "OrderStatus",
            vec!["Pending".into(), "Confirmed".into(), "Shipped".into()],
        );

        assert_eq!(ctx.sketch().colimits.len(), 1);
    }
}
