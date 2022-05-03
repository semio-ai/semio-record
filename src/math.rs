use serde::{Serialize, Deserialize};
use juniper::GraphQLObject;

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Vector2 {
  x: f64,
  y: f64,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct MultiBezier2Chunk {
  p0: Vector2,
  p1: Vector2,
  p3: Vector2,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct MultiBezier2 {
  chunks: Vec<MultiBezier2Chunk>,
  last: Vector2
}