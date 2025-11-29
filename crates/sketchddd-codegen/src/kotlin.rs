//! Kotlin code generation.

use sketchddd_core::BoundedContext;
use crate::CodegenError;

/// Generate Kotlin code from a bounded context.
pub fn generate(context: &BoundedContext) -> Result<String, CodegenError> {
    let mut output = String::new();

    output.push_str(&format!("// Generated from {} bounded context\n\n", context.name()));
    output.push_str(&format!("package {}\n\n", context.name().to_lowercase()));
    output.push_str("import kotlinx.serialization.Serializable\n");
    output.push_str("import java.util.UUID\n\n");

    // Generate entities
    for entity_id in context.entities() {
        if let Some(entity) = context.graph().get_object(*entity_id) {
            output.push_str(&format!(
                "/** Entity: {} */\n@Serializable\ndata class {}(\n    val id: {}Id\n)\n\n",
                entity.name, entity.name, entity.name
            ));
            output.push_str(&format!(
                "@JvmInline\nvalue class {}Id(val value: UUID)\n\n",
                entity.name
            ));
        }
    }

    // Generate value objects
    for vo_id in context.value_objects() {
        if let Some(vo) = context.graph().get_object(*vo_id) {
            output.push_str(&format!(
                "/** Value Object: {} */\n@Serializable\ndata class {}(\n    // TODO: Add fields\n)\n\n",
                vo.name, vo.name
            ));
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_empty_context() {
        let context = BoundedContext::new("Test");
        let result = generate(&context).unwrap();
        assert!(result.contains("Generated from Test"));
    }
}
