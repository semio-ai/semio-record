use serde::{Serialize, Deserialize};
use juniper::GraphQLObject;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject, JsonSchema)]
#[serde(rename = "math_Vector3")]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}