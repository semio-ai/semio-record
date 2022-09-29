use super::Vector3;

use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "math_AxisAngle")]
pub struct AxisAngle {
  pub axis: Vector3,
  pub angle: f64,
}