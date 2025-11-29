//! Pest grammar for SketchDDD DSL.

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SketchDDDParser;
