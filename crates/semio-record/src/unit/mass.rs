use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unit_Mass", tag = "type", content = "value", rename_all = "camelCase")]
pub enum Mass {
  Grams(f64),
  Kilograms(f64),
  Pounds(f64),
  Ounces(f64),
}