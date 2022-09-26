use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Angle {
  Radians(f64),
  Degrees(f64),
}