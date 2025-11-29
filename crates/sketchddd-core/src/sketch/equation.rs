//! Path equations expressing business rules.

use super::{MorphismId, ObjectId};
use serde::{Deserialize, Serialize};

/// A path through the graph, represented as a sequence of morphisms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Path {
    /// Starting object
    pub source: ObjectId,

    /// Sequence of morphisms to follow
    pub morphisms: Vec<MorphismId>,

    /// Ending object (computed from following the path)
    pub target: ObjectId,
}

impl Path {
    /// Create a new path starting from a source object.
    pub fn new(source: ObjectId, target: ObjectId, morphisms: Vec<MorphismId>) -> Self {
        Self {
            source,
            morphisms,
            target,
        }
    }

    /// Create an identity path (no morphisms).
    pub fn identity(object: ObjectId) -> Self {
        Self {
            source: object,
            morphisms: Vec::new(),
            target: object,
        }
    }

    /// Check if this is an identity path.
    pub fn is_identity(&self) -> bool {
        self.morphisms.is_empty() && self.source == self.target
    }

    /// Get the length of the path (number of morphisms).
    pub fn len(&self) -> usize {
        self.morphisms.len()
    }

    /// Check if the path is empty (identity).
    pub fn is_empty(&self) -> bool {
        self.morphisms.is_empty()
    }
}

/// An equation asserting that two paths are equal.
///
/// This represents business rules like:
/// `totalPrice = sum . map(price) . items`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathEquation {
    /// Name or description of the equation
    pub name: String,

    /// Left-hand side path
    pub lhs: Path,

    /// Right-hand side path
    pub rhs: Path,
}

impl PathEquation {
    /// Create a new equation asserting two paths are equal.
    pub fn new(name: impl Into<String>, lhs: Path, rhs: Path) -> Self {
        Self {
            name: name.into(),
            lhs,
            rhs,
        }
    }

    /// Check if the equation is well-formed (same source and target).
    pub fn is_well_formed(&self) -> bool {
        self.lhs.source == self.rhs.source && self.lhs.target == self.rhs.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_path() {
        let obj = ObjectId(0);
        let path = Path::identity(obj);

        assert!(path.is_identity());
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
    }

    #[test]
    fn test_path_with_morphisms() {
        let source = ObjectId(0);
        let target = ObjectId(1);
        let path = Path::new(source, target, vec![MorphismId(0)]);

        assert!(!path.is_identity());
        assert_eq!(path.len(), 1);
    }

    #[test]
    fn test_well_formed_equation() {
        let obj = ObjectId(0);
        let lhs = Path::identity(obj);
        let rhs = Path::identity(obj);
        let eq = PathEquation::new("identity", lhs, rhs);

        assert!(eq.is_well_formed());
    }
}
