use wasm_bindgen::prelude::*;

/// Parse a usage spec KDL string, returning a serialized Spec as a JS object.
#[wasm_bindgen]
pub fn parse_spec(kdl: &str) -> Result<JsValue, JsError> {
    let spec: usage::Spec = kdl
        .parse()
        .map_err(|e: usage::error::UsageErr| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&spec).map_err(|e| JsError::new(&e.to_string()))
}
