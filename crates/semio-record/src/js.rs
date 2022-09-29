use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn apply_raw(ty: i16, schema_version: i16, data: String, action: String) -> Result<String, JsValue> {
  crate::apply_raw(ty, schema_version, data.as_bytes(), action.as_bytes())
    .map(|v| String::from_utf8(v).unwrap())
    .map_err(|e| e.to_string().into())
}

#[wasm_bindgen]
pub fn apply_public_raw(ty: i16, schema_version: i16, data: String, action: String) -> Result<String, JsValue> {
  crate::apply_public_raw(ty, schema_version, data.as_bytes(), action.as_bytes())
    .map(|v| String::from_utf8(v).unwrap())
    .map_err(|e| e.to_string().into())
}

#[wasm_bindgen]
pub fn apply_private_raw(ty: i16, schema_version: i16, data: String, action: String) -> Result<String, JsValue> {
  crate::apply_private_raw(ty, schema_version, data.as_bytes(), action.as_bytes())
    .map(|v| String::from_utf8(v).unwrap())
    .map_err(|e| e.to_string().into())
}