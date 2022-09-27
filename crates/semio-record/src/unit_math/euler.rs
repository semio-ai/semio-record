use serde::{Serialize, Deserialize};

use crate::{unit::Angle, math::EulerOrder};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "unit_math_Euler")]
pub struct Euler {
  pub x: Angle,
  pub y: Angle,
  pub z: Angle,

  pub order: EulerOrder,
}