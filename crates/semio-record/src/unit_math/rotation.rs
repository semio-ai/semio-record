use serde::{Serialize, Deserialize};

use crate::math::Quaternion;

use super::{AxisAngle, Euler};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "unit_math_Rotation", tag = "type", rename_all = "camelCase")]
pub enum Rotation {
  AxisAngle(AxisAngle),
  Euler(Euler),
  Quaternion(Quaternion),
}