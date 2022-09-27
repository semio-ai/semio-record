use serde::{Serialize, Deserialize};
use juniper::GraphQLObject;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject, JsonSchema)]
#[serde(rename = "math_Vector2")]
pub struct Vector2 {
  pub x: f64,
  pub y: f64,
}