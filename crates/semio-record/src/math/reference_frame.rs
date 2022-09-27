use serde::{Serialize, Deserialize};

use super::{Vector3, Quaternion};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "math_ReferenceFrame")]
pub struct ReferenceFrame {
  pub position: Vector3,
  pub orientation: Quaternion,
  pub scale: Vector3,
}