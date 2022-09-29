use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unit_Angle", tag = "type", content = "value", rename_all = "camelCase")]
pub enum Angle {
  Radians(f64),
  Degrees(f64),
}