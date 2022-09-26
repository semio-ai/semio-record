use serde::{Serialize, Deserialize};
use juniper::GraphQLObject;

mod vector2;
mod vector3;
mod euler;
mod axis_angle;
mod reference_frame;
mod quaternion;

pub use vector2::*;
pub use vector3::*;
pub use euler::*;
pub use axis_angle::*;
pub use reference_frame::*;
pub use quaternion::*;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone, JsonSchema)]
pub struct MultiBezier2Chunk {
  p0: Vector2,
  p1: Vector2,
  p2: Vector2,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone, JsonSchema)]
pub struct MultiBezier2 {
  chunks: Vec<MultiBezier2Chunk>,
  last: Vector2
}