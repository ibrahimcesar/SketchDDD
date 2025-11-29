//! Colimit cocones for sum types and enumerations.

use super::ObjectId;
use serde::{Deserialize, Serialize};

/// An injection from a variant into the colimit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Injection {
    /// Name of this variant
    pub name: String,

    /// The source object (variant type)
    pub source: ObjectId,
}

/// A colimit cocone representing a sum type or enumeration.
///
/// In category theory, a colimit is a universal construction that
/// represents "choice" between objects. For DDD:
/// - **Enumerations**: Simple sum types with named variants
/// - **Sum Types**: More complex discriminated unions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColimitCocone {
    /// Name of the colimit (e.g., "OrderStatus")
    pub name: String,

    /// The apex object (the sum type itself)
    pub apex: ObjectId,

    /// Injections from variant types
    pub injections: Vec<Injection>,
}

impl ColimitCocone {
    /// Create a new colimit cocone.
    pub fn new(name: impl Into<String>, apex: ObjectId) -> Self {
        Self {
            name: name.into(),
            apex,
            injections: Vec::new(),
        }
    }

    /// Add a variant to the colimit.
    pub fn add_variant(&mut self, name: impl Into<String>, source: ObjectId) {
        self.injections.push(Injection {
            name: name.into(),
            source,
        });
    }

    /// Create an enumeration with simple named variants.
    ///
    /// For simple enums where variants don't carry data,
    /// the source objects can be unit types.
    pub fn enumeration(name: impl Into<String>, apex: ObjectId, variants: Vec<String>) -> Self {
        let mut cocone = Self::new(name, apex);
        for variant in variants {
            // For simple enums, we use the apex as the source
            // (representing unit-like variants)
            cocone.injections.push(Injection {
                name: variant,
                source: apex,
            });
        }
        cocone
    }

    /// Get the names of all variants.
    pub fn variant_names(&self) -> impl Iterator<Item = &str> {
        self.injections.iter().map(|i| i.name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colimit_creation() {
        let apex = ObjectId(0);
        let colimit = ColimitCocone::new("OrderStatus", apex);

        assert_eq!(colimit.name, "OrderStatus");
        assert!(colimit.injections.is_empty());
    }

    #[test]
    fn test_add_variants() {
        let apex = ObjectId(0);
        let mut colimit = ColimitCocone::new("OrderStatus", apex);

        colimit.add_variant("Pending", ObjectId(1));
        colimit.add_variant("Confirmed", ObjectId(2));

        assert_eq!(colimit.injections.len(), 2);
    }

    #[test]
    fn test_enumeration() {
        let apex = ObjectId(0);
        let status = ColimitCocone::enumeration(
            "OrderStatus",
            apex,
            vec!["Pending".into(), "Confirmed".into(), "Shipped".into()],
        );

        let names: Vec<_> = status.variant_names().collect();
        assert_eq!(names, vec!["Pending", "Confirmed", "Shipped"]);
    }
}
