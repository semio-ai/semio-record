use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "unit_Angle", tag = "type", content = "value", rename_all = "camelCase")]
pub enum Angle {
  Radians(f64),
  Degrees(f64),
}