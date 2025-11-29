//! Bounded Context as a DDD-specific wrapper around Sketch.

use crate::sketch::{ColimitCocone, Graph, LimitCone, MorphismId, ObjectId, PathEquation, Sketch};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    /// Identity morphisms for entities (Entity -> identity morphism)
    entity_identities: HashMap<ObjectId, MorphismId>,

    /// Value objects within this context (objects with structural equality)
    value_objects: Vec<ObjectId>,

    /// Aggregate roots
    aggregate_roots: Vec<ObjectId>,

    /// Invariants (equalizers) in this context
    invariants: Vec<Invariant>,
}

/// An invariant expressed as an equalizer.
///
/// In category theory, an equalizer of two morphisms f, g : A → B is
/// an object E with a morphism e : E → A such that f ∘ e = g ∘ e.
/// This represents a business rule that constrains valid states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invariant {
    /// Name of the invariant
    pub name: String,

    /// The equalizer object
    pub equalizer: ObjectId,

    /// The morphism from equalizer to the constrained object
    pub inclusion: MorphismId,

    /// First morphism being equalized
    pub morphism_f: MorphismId,

    /// Second morphism being equalized
    pub morphism_g: MorphismId,

    /// Human-readable description of the constraint
    pub description: Option<String>,
}

impl BoundedContext {
    /// Create a new bounded context with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            sketch: Sketch::new(name),
            entities: Vec::new(),
            entity_identities: HashMap::new(),
            value_objects: Vec::new(),
            aggregate_roots: Vec::new(),
            invariants: Vec::new(),
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
    /// through time and across different representations. In category theory,
    /// this is represented by an object with an explicit identity morphism.
    pub fn add_entity(&mut self, name: impl Into<String>) -> ObjectId {
        let id = self.sketch.add_object(name);
        // Add identity morphism for the entity (categorical representation of "identity")
        let identity = self.sketch.graph.add_identity_morphism(id);
        self.entities.push(id);
        self.entity_identities.insert(id, identity);
        id
    }

    /// Get the identity morphism for an entity.
    pub fn get_entity_identity(&self, entity: ObjectId) -> Option<MorphismId> {
        self.entity_identities.get(&entity).copied()
    }

    /// Add a value object to this context.
    ///
    /// A value object is defined entirely by its attributes and has
    /// no conceptual identity. Two value objects with the same
    /// attributes are considered equal. In category theory, this is
    /// represented as a limit with structural equality semantics.
    pub fn add_value_object(&mut self, name: impl Into<String>) -> ObjectId {
        let name_str = name.into();
        let id = self.sketch.add_object(&name_str);
        // Create a limit cone representing the value object's structural definition
        let limit = LimitCone::value_object(&name_str, id);
        self.sketch.add_limit(limit);
        self.value_objects.push(id);
        id
    }

    /// Add a value object with explicit components.
    ///
    /// This creates a value object as a product type of its components,
    /// with structural equality based on all component values.
    pub fn add_value_object_with_components(
        &mut self,
        name: impl Into<String>,
        component_types: &[ObjectId],
    ) -> ObjectId {
        let name_str = name.into();
        let id = self.sketch.add_object(&name_str);
        let mut limit = LimitCone::value_object(&name_str, id);

        // Add projections to component types
        for (i, &component) in component_types.iter().enumerate() {
            let proj_name = format!("proj_{}", i);
            let morphism = self.sketch.graph.add_morphism(&proj_name, id, component);
            limit.add_projection(morphism, component);
        }

        self.sketch.add_limit(limit);
        self.value_objects.push(id);
        id
    }

    /// Get the limit cone for a value object.
    pub fn get_value_object_limit(&self, value_object: ObjectId) -> Option<&LimitCone> {
        self.sketch
            .limits
            .iter()
            .find(|l| !l.is_aggregate && l.apex == value_object)
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
    ///
    /// An aggregate is a cluster of domain objects that can be treated as a unit.
    /// In category theory, this is represented as a limit cone where the apex
    /// is the aggregate and projections point to contained entities/value objects.
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

    /// Define an aggregate with its root and member entities.
    ///
    /// Creates the aggregate as a limit cone with the root as apex and
    /// projections to all member entities, representing the aggregate boundary.
    pub fn define_aggregate_with_members(
        &mut self,
        name: impl Into<String>,
        root: ObjectId,
        members: &[ObjectId],
    ) -> &mut LimitCone {
        self.aggregate_roots.push(root);
        let name_str = name.into();
        let mut limit = LimitCone::aggregate(&name_str, root, root);

        // Add projections to member entities
        for &member in members {
            if let Some(obj) = self.sketch.graph.get_object(member) {
                let proj_name = format!("{}_{}", name_str, obj.name);
                let morphism = self.sketch.graph.add_morphism(&proj_name, root, member);
                limit.add_projection(morphism, member);
            }
        }

        self.sketch.add_limit(limit);
        self.sketch.limits.last_mut().unwrap()
    }

    /// Get the aggregate limit cone for a given root.
    pub fn get_aggregate(&self, root: ObjectId) -> Option<&LimitCone> {
        self.sketch
            .limits
            .iter()
            .find(|l| l.is_aggregate && l.root == Some(root))
    }

    /// Check if an object is an aggregate root.
    pub fn is_aggregate_root(&self, id: ObjectId) -> bool {
        self.aggregate_roots.contains(&id)
    }

    /// Add an enumeration to this context.
    ///
    /// An enumeration is represented as a colimit (coproduct/sum type) where
    /// each variant is an injection into the sum.
    pub fn add_enum(&mut self, name: impl Into<String>, variants: Vec<String>) -> ObjectId {
        let name_str = name.into();
        let id = self.sketch.add_object(&name_str);
        let colimit = ColimitCocone::enumeration(name_str, id, variants);
        self.sketch.add_colimit(colimit);
        id
    }

    /// Add a sum type with variant objects.
    ///
    /// Unlike simple enumerations, sum types can have different data for each variant.
    /// This is the categorical coproduct.
    pub fn add_sum_type(
        &mut self,
        name: impl Into<String>,
        variants: Vec<(String, ObjectId)>,
    ) -> ObjectId {
        let name_str = name.into();
        let id = self.sketch.add_object(&name_str);
        let mut colimit = ColimitCocone::new(&name_str, id);

        for (variant_name, variant_type) in variants {
            colimit.add_variant(variant_name, variant_type);
        }

        self.sketch.add_colimit(colimit);
        id
    }

    /// Get the colimit cocone for an enumeration or sum type.
    pub fn get_enum_colimit(&self, enum_id: ObjectId) -> Option<&ColimitCocone> {
        self.sketch.colimits.iter().find(|c| c.apex == enum_id)
    }

    /// Add a business rule as a path equation.
    pub fn add_path_equation(&mut self, name: impl Into<String>, equation: PathEquation) {
        let mut eq = equation;
        eq.name = name.into();
        self.sketch.add_equation(eq);
    }

    /// Add an invariant as an equalizer.
    ///
    /// An equalizer represents a constraint where two paths must be equal.
    /// In DDD terms, this is a business rule that restricts valid states.
    ///
    /// # Arguments
    /// * `name` - Name of the invariant
    /// * `source` - The object being constrained
    /// * `f` - First morphism from source
    /// * `g` - Second morphism from source (must have same target as f)
    /// * `description` - Human-readable description of the constraint
    pub fn add_equalizer_invariant(
        &mut self,
        name: impl Into<String>,
        source: ObjectId,
        f: MorphismId,
        g: MorphismId,
        description: Option<String>,
    ) -> ObjectId {
        let name_str = name.into();
        // Create the equalizer object
        let equalizer = self.sketch.add_object(format!("Eq_{}", name_str));
        // Create the inclusion morphism from equalizer to source
        let inclusion = self.sketch.graph.add_morphism(
            format!("incl_{}", name_str),
            equalizer,
            source,
        );

        let invariant = Invariant {
            name: name_str,
            equalizer,
            inclusion,
            morphism_f: f,
            morphism_g: g,
            description,
        };

        self.invariants.push(invariant);
        equalizer
    }

    /// Get all invariants in this context.
    pub fn invariants(&self) -> &[Invariant] {
        &self.invariants
    }

    /// Add a business rule (path equation) - deprecated, use add_path_equation.
    #[deprecated(since = "0.1.0", note = "Use add_path_equation instead")]
    pub fn add_invariant(&mut self, name: impl Into<String>, equation: PathEquation) {
        self.add_path_equation(name, equation);
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
    use crate::sketch::Path;

    #[test]
    fn test_create_context() {
        let ctx = BoundedContext::new("Commerce");
        assert_eq!(ctx.name(), "Commerce");
    }

    // ========== Entity Tests ==========

    #[test]
    fn test_add_entity() {
        let mut ctx = BoundedContext::new("Commerce");
        let customer = ctx.add_entity("Customer");

        assert!(ctx.is_entity(customer));
        assert!(!ctx.is_value_object(customer));
    }

    #[test]
    fn test_entity_has_identity_morphism() {
        let mut ctx = BoundedContext::new("Commerce");
        let customer = ctx.add_entity("Customer");

        // Entity should have an identity morphism
        let identity = ctx.get_entity_identity(customer);
        assert!(identity.is_some());

        // The identity morphism should exist in the graph
        let morph = ctx.graph().get_morphism(identity.unwrap());
        assert!(morph.is_some());
        let morph = morph.unwrap();
        assert!(morph.is_identity);
        assert_eq!(morph.source, customer);
        assert_eq!(morph.target, customer);
        assert_eq!(morph.name, "id_Customer");
    }

    #[test]
    fn test_multiple_entities_have_separate_identities() {
        let mut ctx = BoundedContext::new("Commerce");
        let customer = ctx.add_entity("Customer");
        let order = ctx.add_entity("Order");

        let customer_id = ctx.get_entity_identity(customer).unwrap();
        let order_id = ctx.get_entity_identity(order).unwrap();

        assert_ne!(customer_id, order_id);
    }

    // ========== Value Object Tests ==========

    #[test]
    fn test_add_value_object() {
        let mut ctx = BoundedContext::new("Commerce");
        let money = ctx.add_value_object("Money");

        assert!(ctx.is_value_object(money));
        assert!(!ctx.is_entity(money));
    }

    #[test]
    fn test_value_object_has_limit_cone() {
        let mut ctx = BoundedContext::new("Commerce");
        let money = ctx.add_value_object("Money");

        let limit = ctx.get_value_object_limit(money);
        assert!(limit.is_some());
        let limit = limit.unwrap();
        assert!(!limit.is_aggregate);
        assert_eq!(limit.apex, money);
    }

    #[test]
    fn test_value_object_with_components() {
        let mut ctx = BoundedContext::new("Commerce");

        // Add primitive types
        let amount = ctx.sketch_mut().add_object("Decimal");
        let currency = ctx.sketch_mut().add_object("Currency");

        // Create Money as a value object with components
        let money = ctx.add_value_object_with_components("Money", &[amount, currency]);

        assert!(ctx.is_value_object(money));

        let limit = ctx.get_value_object_limit(money).unwrap();
        assert_eq!(limit.projections.len(), 2);
        assert_eq!(limit.component_objects().count(), 2);
    }

    // ========== Aggregate Tests ==========

    #[test]
    fn test_define_aggregate() {
        let mut ctx = BoundedContext::new("Commerce");
        let order = ctx.add_entity("Order");
        ctx.define_aggregate("OrderAggregate", order);

        assert!(ctx.is_aggregate_root(order));
        assert_eq!(ctx.aggregate_roots().len(), 1);
    }

    #[test]
    fn test_aggregate_with_members() {
        let mut ctx = BoundedContext::new("Commerce");
        let order = ctx.add_entity("Order");
        let line_item = ctx.add_entity("LineItem");
        let shipping = ctx.add_value_object("ShippingInfo");

        ctx.define_aggregate_with_members("OrderAggregate", order, &[line_item, shipping]);

        let aggregate = ctx.get_aggregate(order);
        assert!(aggregate.is_some());
        let aggregate = aggregate.unwrap();
        assert!(aggregate.is_aggregate);
        assert_eq!(aggregate.root, Some(order));
        assert_eq!(aggregate.projections.len(), 2);
    }

    // ========== Enumeration Tests ==========

    #[test]
    fn test_add_enum() {
        let mut ctx = BoundedContext::new("Commerce");
        let status = ctx.add_enum(
            "OrderStatus",
            vec!["Pending".into(), "Confirmed".into(), "Shipped".into()],
        );

        assert_eq!(ctx.sketch().colimits.len(), 1);

        let colimit = ctx.get_enum_colimit(status);
        assert!(colimit.is_some());
        let colimit = colimit.unwrap();
        let variants: Vec<_> = colimit.variant_names().collect();
        assert_eq!(variants, vec!["Pending", "Confirmed", "Shipped"]);
    }

    #[test]
    fn test_add_sum_type() {
        let mut ctx = BoundedContext::new("Commerce");

        // Create variant types
        let pending = ctx.sketch_mut().add_object("Pending");
        let confirmed = ctx.sketch_mut().add_object("Confirmed");
        let shipped = ctx.sketch_mut().add_object("Shipped");

        let status = ctx.add_sum_type(
            "OrderStatus",
            vec![
                ("Pending".into(), pending),
                ("Confirmed".into(), confirmed),
                ("Shipped".into(), shipped),
            ],
        );

        let colimit = ctx.get_enum_colimit(status).unwrap();
        assert_eq!(colimit.injections.len(), 3);
    }

    // ========== Invariant Tests ==========

    #[test]
    fn test_add_equalizer_invariant() {
        let mut ctx = BoundedContext::new("Commerce");

        // Create objects
        let order = ctx.add_entity("Order");
        let computed_total = ctx.sketch_mut().add_object("ComputedTotal");
        let stored_total = ctx.sketch_mut().add_object("StoredTotal");

        // Create morphisms representing two ways to get a total
        let f = ctx.sketch_mut().graph.add_morphism("computeTotal", order, computed_total);
        let g = ctx.sketch_mut().graph.add_morphism("storedTotal", order, stored_total);

        // Add invariant: computed total must equal stored total
        let _eq = ctx.add_equalizer_invariant(
            "TotalConsistency",
            order,
            f,
            g,
            Some("The computed total must match the stored total".into()),
        );

        assert_eq!(ctx.invariants().len(), 1);
        let inv = &ctx.invariants()[0];
        assert_eq!(inv.name, "TotalConsistency");
        assert_eq!(inv.morphism_f, f);
        assert_eq!(inv.morphism_g, g);
    }

    #[test]
    fn test_add_path_equation() {
        let mut ctx = BoundedContext::new("Commerce");
        let order = ctx.add_entity("Order");

        let path1 = Path::identity(order);
        let path2 = Path::identity(order);
        let equation = PathEquation::new("IdentityRule", path1, path2);

        #[allow(deprecated)]
        ctx.add_invariant("Test", equation);

        assert_eq!(ctx.sketch().equations.len(), 1);
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_commerce_domain_model() {
        let mut ctx = BoundedContext::new("Commerce");

        // Entities
        let customer = ctx.add_entity("Customer");
        let order = ctx.add_entity("Order");
        let line_item = ctx.add_entity("LineItem");
        let product = ctx.add_entity("Product");

        // Value objects
        let money = ctx.add_value_object("Money");
        let address = ctx.add_value_object("Address");

        // Enumeration
        let _status = ctx.add_enum(
            "OrderStatus",
            vec![
                "Pending".into(),
                "Confirmed".into(),
                "Shipped".into(),
                "Delivered".into(),
            ],
        );

        // Aggregate
        ctx.define_aggregate_with_members("OrderAggregate", order, &[line_item]);

        // Morphisms (relationships)
        ctx.sketch_mut().graph.add_morphism("placedBy", order, customer);
        ctx.sketch_mut().graph.add_morphism("shippingAddress", order, address);
        ctx.sketch_mut().graph.add_morphism("total", order, money);
        ctx.sketch_mut().graph.add_morphism("product", line_item, product);
        ctx.sketch_mut().graph.add_morphism("price", line_item, money);

        // Verify structure
        assert_eq!(ctx.entities().len(), 4);
        assert_eq!(ctx.value_objects().len(), 2);
        assert_eq!(ctx.aggregate_roots().len(), 1);
        assert_eq!(ctx.sketch().colimits.len(), 1);

        // All entities should have identity morphisms
        for entity in ctx.entities() {
            assert!(ctx.get_entity_identity(*entity).is_some());
        }

        // Value objects should have limit cones
        for vo in ctx.value_objects() {
            assert!(ctx.get_value_object_limit(*vo).is_some());
        }
    }
}
