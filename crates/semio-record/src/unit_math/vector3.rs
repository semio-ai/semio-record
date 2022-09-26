use serde::{Serialize, Deserialize};

use crate::unit::Distance;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Vector3 {
  pub x: Distance,
  pub y: Distance,
  pub z: Distance,
}