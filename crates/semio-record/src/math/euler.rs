use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "math_Euler_Order", rename_all = "lowercase")]
pub enum EulerOrder {
  XYZ,
  XZY,
  YXZ,
  YZX,
  ZXY,
  ZYX,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "math_Euler")]
pub struct Euler {
  pub x: f64,
  pub y: f64,
  pub z: f64,

  pub order: EulerOrder,
}