use serde::{Serialize, Deserialize};

use super::{Vector3, Quaternion};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "math_ReferenceFrame")]
pub struct ReferenceFrame {
  pub position: Vector3,
  pub orientation: Quaternion,
  pub scale: Vector3,
}