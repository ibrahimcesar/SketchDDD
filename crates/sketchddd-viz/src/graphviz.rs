//! Graphviz DOT format generation.

use sketchddd_core::BoundedContext;
use crate::VizError;

/// Generate Graphviz DOT from a bounded context.
pub fn generate(context: &BoundedContext) -> Result<String, VizError> {
    let mut output = String::new();

    output.push_str(&format!("digraph {} {{\n", context.name()));
    output.push_str("  rankdir=LR;\n");
    output.push_str("  node [shape=box];\n\n");

    // Add objects as nodes
    for object in context.graph().objects() {
        let shape = if context.is_entity(object.id) {
            "box"
        } else if context.is_value_object(object.id) {
            "ellipse"
        } else {
            "box"
        };
        output.push_str(&format!(
            "  {} [label=\"{}\" shape={}];\n",
            object.name, object.name, shape
        ));
    }

    output.push_str("\n");

    // Add morphisms as edges
    for morphism in context.graph().morphisms() {
        if let (Some(source), Some(target)) = (
            context.graph().get_object(morphism.source),
            context.graph().get_object(morphism.target),
        ) {
            output.push_str(&format!(
                "  {} -> {} [label=\"{}\"];\n",
                source.name, target.name, morphism.name
            ));
        }
    }

    output.push_str("}\n");

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_empty_context() {
        let context = BoundedContext::new("Test");
        let result = generate(&context).unwrap();
        assert!(result.contains("digraph Test"));
    }
}
