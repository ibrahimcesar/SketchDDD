//! # SketchDDD WASM
//!
//! WebAssembly bindings for use in the browser-based visual builder.

use wasm_bindgen::prelude::*;
use sketchddd_core::{BoundedContext, validation};

/// Initialize the WASM module.
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Create a new bounded context.
#[wasm_bindgen]
pub fn create_context(name: &str) -> JsValue {
    let context = BoundedContext::new(name);
    serde_wasm_bindgen::to_value(&context).unwrap_or(JsValue::NULL)
}

/// Parse a SketchDDD source file.
#[wasm_bindgen]
pub fn parse(source: &str) -> Result<JsValue, JsValue> {
    match sketchddd_parser::parse(source) {
        Ok(contexts) => {
            serde_wasm_bindgen::to_value(&contexts)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

/// Validate a bounded context and return any errors.
#[wasm_bindgen]
pub fn validate(context_json: &str) -> Result<JsValue, JsValue> {
    let context: BoundedContext = serde_json::from_str(context_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let result = validation::validate_sketch(context.sketch());

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get the version of the WASM module.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
