use serde::{Serialize, Deserialize};

use crate::math::Quaternion;

use super::{AxisAngle, Euler};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Rotation {
  AxisAngle(AxisAngle),
  Euler(Euler),
  Quaternion(Quaternion),
}