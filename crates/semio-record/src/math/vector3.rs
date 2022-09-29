use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "math_Vector3")]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}