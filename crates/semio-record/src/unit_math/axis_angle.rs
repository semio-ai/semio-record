use serde::{Serialize, Deserialize};

use crate::unit::Angle;

use super::Vector3;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct AxisAngle {
  pub axis: Vector3,
  pub angle: Angle,
}