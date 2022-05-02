use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject, GraphQLUnion};

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Rgb {
  r: f64,
  g: f64,
  b: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Rgba {
  r: f64,
  g: f64,
  b: f64,
  a: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Hsl {
  h: f64,
  s: f64,
  l: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Hsla {
  h: f64,
  s: f64,
  l: f64,
  a: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLUnion)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Color {
  Rgb(Rgb),
  Rgba(Rgba),
  Hsl(Hsl),
  Hsla(Hsla),
}