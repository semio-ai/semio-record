use serde::{Serialize, Deserialize};

use crate::math::Quaternion;

use super::{AxisAngle, Euler};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unit_math_Rotation", tag = "type", rename_all = "camelCase")]
pub enum Rotation {
  AxisAngle(AxisAngle),
  Euler(Euler),
  Quaternion(Quaternion),
}