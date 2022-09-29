use serde::{Serialize, Deserialize};

use crate::unit::Distance;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unit_math_Vector3")]
pub struct Vector3 {
  pub x: Distance,
  pub y: Distance,
  pub z: Distance,
}