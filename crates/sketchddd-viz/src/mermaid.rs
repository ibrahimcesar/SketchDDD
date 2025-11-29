//! Mermaid diagram format generation.

use sketchddd_core::BoundedContext;
use crate::VizError;

/// Generate Mermaid diagram from a bounded context.
pub fn generate(context: &BoundedContext) -> Result<String, VizError> {
    let mut output = String::new();

    output.push_str("```mermaid\n");
    output.push_str("classDiagram\n");
    output.push_str(&format!("    %% {}\n\n", context.name()));

    // Add objects as classes
    for object in context.graph().objects() {
        let stereotype = if context.is_entity(object.id) {
            "<<Entity>>"
        } else if context.is_value_object(object.id) {
            "<<ValueObject>>"
        } else {
            ""
        };

        if !stereotype.is_empty() {
            output.push_str(&format!("    class {} {{\n", object.name));
            output.push_str(&format!("        {}\n", stereotype));
            output.push_str("    }\n");
        } else {
            output.push_str(&format!("    class {}\n", object.name));
        }
    }

    output.push_str("\n");

    // Add morphisms as relationships
    for morphism in context.graph().morphisms() {
        if let (Some(source), Some(target)) = (
            context.graph().get_object(morphism.source),
            context.graph().get_object(morphism.target),
        ) {
            output.push_str(&format!(
                "    {} --> {} : {}\n",
                source.name, target.name, morphism.name
            ));
        }
    }

    output.push_str("```\n");

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_empty_context() {
        let context = BoundedContext::new("Test");
        let result = generate(&context).unwrap();
        assert!(result.contains("classDiagram"));
    }
}
