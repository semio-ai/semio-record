use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Mass {
  Grams(f64),
  Kilograms(f64),
  Pounds(f64),
  Ounces(f64),
}