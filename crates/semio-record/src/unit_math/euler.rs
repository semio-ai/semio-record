use serde::{Serialize, Deserialize};

use crate::{unit::Angle, math::EulerOrder};



#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unit_math_Euler")]
pub struct Euler {
  pub x: Angle,
  pub y: Angle,
  pub z: Angle,

  pub order: EulerOrder,
}