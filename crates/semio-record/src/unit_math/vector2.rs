use serde::{Serialize, Deserialize};

use crate::unit::Distance;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unit_math_Vector2")]
pub struct Vector2 {
  pub x: Distance,
  pub y: Distance,
}