use serde::{Serialize, Deserialize};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum EulerOrder {
  XYZ,
  XZY,
  YXZ,
  YZX,
  ZXY,
  ZYX,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Euler {
  pub x: f64,
  pub y: f64,
  pub z: f64,

  pub order: EulerOrder,
}