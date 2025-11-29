//! # SketchDDD Parser
//!
//! Parser for the SketchDDD domain-specific language.
//!
//! ## Example
//!
//! ```text
//! context Commerce {
//!   objects { Customer, Order, LineItem, Product, Money }
//!
//!   morphisms {
//!     placedBy: Order -> Customer
//!     items: Order -> List<LineItem>
//!     product: LineItem -> Product
//!     price: LineItem -> Money
//!   }
//!
//!   aggregate Order {
//!     root: Order
//!     contains: [LineItem]
//!     invariant: totalPrice = sum(items.price)
//!   }
//!
//!   value Money {
//!     amount: Decimal
//!     currency: Currency
//!   }
//!
//!   enum OrderStatus = Pending | Confirmed | Shipped | Cancelled
//! }
//! ```

mod grammar;
mod ast;
mod error;

pub use ast::*;
pub use error::ParseError;

use pest::Parser;

/// Parse a SketchDDD source file.
pub fn parse(source: &str) -> Result<Vec<ContextDecl>, ParseError> {
    // TODO: Implement full parsing
    let _ = source;
    Ok(Vec::new())
}

/// Parse a single context definition.
pub fn parse_context(source: &str) -> Result<ContextDecl, ParseError> {
    let contexts = parse(source)?;
    contexts
        .into_iter()
        .next()
        .ok_or_else(|| ParseError::new("No context found in source"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let result = parse("");
        assert!(result.is_ok());
    }
}
