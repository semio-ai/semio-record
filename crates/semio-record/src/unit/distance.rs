use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DistanceKind {
  Meters,
  Centimeters,
  Millimeters,
  Feet,
  Inches,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Distance {
  Meters(f64),
  Centimeters(f64),
  Millimeters(f64),
  Feet(f64),
  Inches(f64),
}