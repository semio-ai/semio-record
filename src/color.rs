use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject, GraphQLUnion};

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Rgb {
  pub r: f64,
  pub g: f64,
  pub b: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Rgba {
  pub r: f64,
  pub g: f64,
  pub b: f64,
  pub a: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Hsl {
  pub h: f64,
  pub s: f64,
  pub l: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Hsla {
  pub h: f64,
  pub s: f64,
  pub l: f64,
  pub a: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLUnion)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Color {
  Rgb(Rgb),
  Rgba(Rgba),
  Hsl(Hsl),
  Hsla(Hsla),
}