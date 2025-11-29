//! Graph structures for representing objects and morphisms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for an object in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectId(pub(crate) u32);

/// Unique identifier for a morphism in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MorphismId(pub(crate) u32);

/// An object (node) in the graph, representing a domain concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    /// Unique identifier
    pub id: ObjectId,

    /// Name of the object (e.g., "Customer", "Order")
    pub name: String,

    /// Optional description
    pub description: Option<String>,
}

/// A morphism (edge) in the graph, representing a relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Morphism {
    /// Unique identifier
    pub id: MorphismId,

    /// Name of the morphism (e.g., "placedBy", "items")
    pub name: String,

    /// Source object
    pub source: ObjectId,

    /// Target object
    pub target: ObjectId,

    /// Optional description
    pub description: Option<String>,
}

/// A directed graph of objects and morphisms.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    objects: HashMap<ObjectId, Object>,
    morphisms: HashMap<MorphismId, Morphism>,
    next_object_id: u32,
    next_morphism_id: u32,
}

impl Graph {
    /// Create a new empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an object to the graph.
    pub fn add_object(&mut self, name: impl Into<String>) -> ObjectId {
        let id = ObjectId(self.next_object_id);
        self.next_object_id += 1;

        let object = Object {
            id,
            name: name.into(),
            description: None,
        };

        self.objects.insert(id, object);
        id
    }

    /// Add a morphism between two objects.
    pub fn add_morphism(
        &mut self,
        name: impl Into<String>,
        source: ObjectId,
        target: ObjectId,
    ) -> MorphismId {
        let id = MorphismId(self.next_morphism_id);
        self.next_morphism_id += 1;

        let morphism = Morphism {
            id,
            name: name.into(),
            source,
            target,
            description: None,
        };

        self.morphisms.insert(id, morphism);
        id
    }

    /// Get an object by its ID.
    pub fn get_object(&self, id: ObjectId) -> Option<&Object> {
        self.objects.get(&id)
    }

    /// Get a morphism by its ID.
    pub fn get_morphism(&self, id: MorphismId) -> Option<&Morphism> {
        self.morphisms.get(&id)
    }

    /// Get all objects.
    pub fn objects(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    /// Get all morphisms.
    pub fn morphisms(&self) -> impl Iterator<Item = &Morphism> {
        self.morphisms.values()
    }

    /// Find an object by name.
    pub fn find_object_by_name(&self, name: &str) -> Option<&Object> {
        self.objects.values().find(|o| o.name == name)
    }

    /// Find a morphism by name.
    pub fn find_morphism_by_name(&self, name: &str) -> Option<&Morphism> {
        self.morphisms.values().find(|m| m.name == name)
    }

    /// Get all morphisms originating from an object.
    pub fn outgoing_morphisms(&self, source: ObjectId) -> impl Iterator<Item = &Morphism> {
        self.morphisms.values().filter(move |m| m.source == source)
    }

    /// Get all morphisms targeting an object.
    pub fn incoming_morphisms(&self, target: ObjectId) -> impl Iterator<Item = &Morphism> {
        self.morphisms.values().filter(move |m| m.target == target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = Graph::new();
        assert_eq!(graph.objects().count(), 0);
        assert_eq!(graph.morphisms().count(), 0);
    }

    #[test]
    fn test_add_objects() {
        let mut graph = Graph::new();
        let customer = graph.add_object("Customer");
        let order = graph.add_object("Order");

        assert_eq!(graph.objects().count(), 2);
        assert_eq!(graph.get_object(customer).unwrap().name, "Customer");
        assert_eq!(graph.get_object(order).unwrap().name, "Order");
    }

    #[test]
    fn test_add_morphisms() {
        let mut graph = Graph::new();
        let customer = graph.add_object("Customer");
        let order = graph.add_object("Order");
        let placed_by = graph.add_morphism("placedBy", order, customer);

        assert_eq!(graph.morphisms().count(), 1);
        let m = graph.get_morphism(placed_by).unwrap();
        assert_eq!(m.name, "placedBy");
        assert_eq!(m.source, order);
        assert_eq!(m.target, customer);
    }

    #[test]
    fn test_find_by_name() {
        let mut graph = Graph::new();
        graph.add_object("Customer");

        assert!(graph.find_object_by_name("Customer").is_some());
        assert!(graph.find_object_by_name("NotFound").is_none());
    }
}
