use serde::{Serialize, Deserialize};

use crate::unit::Angle;

use super::Vector3;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unit_math_AxisAngle")]
pub struct AxisAngle {
  pub axis: Vector3,
  pub angle: Angle,
}