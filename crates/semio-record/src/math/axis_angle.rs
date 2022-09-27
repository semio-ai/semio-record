use super::Vector3;

use serde::{Serialize, Deserialize};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "math_AxisAngle")]
pub struct AxisAngle {
  pub axis: Vector3,
  pub angle: f64,
}