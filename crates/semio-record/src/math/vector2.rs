use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "math_Vector2")]
pub struct Vector2 {
  pub x: f64,
  pub y: f64,
}