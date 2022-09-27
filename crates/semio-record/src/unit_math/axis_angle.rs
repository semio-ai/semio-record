use serde::{Serialize, Deserialize};

use crate::unit::Angle;

use super::Vector3;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "unit_math_AxisAngle")]
pub struct AxisAngle {
  pub axis: Vector3,
  pub angle: Angle,
}