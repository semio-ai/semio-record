use serde::{Serialize, Deserialize};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "math_Quaternion")]
pub struct Quaternion {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}