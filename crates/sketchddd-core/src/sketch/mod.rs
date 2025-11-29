//! Sketch data structures representing categorical semantics.
//!
//! A sketch is the fundamental structure in SketchDDD, providing
//! precise mathematical definitions for domain models.

mod graph;
mod equation;
mod limit;
mod colimit;

pub use graph::{Graph, Object, Morphism, ObjectId, MorphismId};
pub use equation::{PathEquation, Path};
pub use limit::{LimitCone, Projection};
pub use colimit::{ColimitCocone, Injection};

use serde::{Deserialize, Serialize};

/// A sketch `S = (G, E, L, C)` representing a domain model.
///
/// This is the core data structure that unifies all DDD concepts
/// into a single categorical framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sketch {
    /// The name of this sketch
    pub name: String,

    /// The underlying graph of objects and morphisms
    pub graph: Graph,

    /// Path equations expressing business rules
    pub equations: Vec<PathEquation>,

    /// Limit cones (aggregates, value objects)
    pub limits: Vec<LimitCone>,

    /// Colimit cocones (sum types, enumerations)
    pub colimits: Vec<ColimitCocone>,
}

impl Sketch {
    /// Create a new empty sketch with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            graph: Graph::new(),
            equations: Vec::new(),
            limits: Vec::new(),
            colimits: Vec::new(),
        }
    }

    /// Add an object to the sketch's graph.
    pub fn add_object(&mut self, name: impl Into<String>) -> ObjectId {
        self.graph.add_object(name)
    }

    /// Add a morphism between objects.
    pub fn add_morphism(
        &mut self,
        name: impl Into<String>,
        source: ObjectId,
        target: ObjectId,
    ) -> MorphismId {
        self.graph.add_morphism(name, source, target)
    }

    /// Add a path equation (business rule).
    pub fn add_equation(&mut self, equation: PathEquation) {
        self.equations.push(equation);
    }

    /// Add a limit cone (aggregate or value object).
    pub fn add_limit(&mut self, limit: LimitCone) {
        self.limits.push(limit);
    }

    /// Add a colimit cocone (sum type or enumeration).
    pub fn add_colimit(&mut self, colimit: ColimitCocone) {
        self.colimits.push(colimit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sketch() {
        let sketch = Sketch::new("Commerce");
        assert_eq!(sketch.name, "Commerce");
        assert_eq!(sketch.graph.objects().count(), 0);
    }

    #[test]
    fn test_add_objects_and_morphisms() {
        let mut sketch = Sketch::new("Commerce");

        let order = sketch.add_object("Order");
        let customer = sketch.add_object("Customer");
        let _placed_by = sketch.add_morphism("placedBy", order, customer);

        assert_eq!(sketch.graph.objects().count(), 2);
        assert_eq!(sketch.graph.morphisms().count(), 1);
    }
}
