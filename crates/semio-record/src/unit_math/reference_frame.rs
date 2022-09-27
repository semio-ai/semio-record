use super::{Vector3, Rotation};
use crate::math::Vector3 as RawVector3;

use serde::{Serialize, Deserialize};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "unit_math_ReferenceFrame")]
pub struct ReferenceFrame {
  pub position: Vector3,
  pub orientation: Rotation,
  pub scale: RawVector3,
}