use std::borrow::Cow;

use derive_more::From;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Degrees {
  pub value: f64,
}

impl From<Radians> for Degrees {
  fn from(other: Radians) -> Self {
    Self {
      value: other.value * 360.0 / std::f64::consts::TAU,
    }
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Radians {
  pub value: f64,
}

impl From<Degrees> for Radians {
  fn from(other: Degrees) -> Self {
    Self {
      value: other.value * std::f64::consts::TAU / 360.0,
    }
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLUnion, From)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Angle {
  Degrees(Degrees),
  Radians(Radians),
}

impl Angle {
  pub fn as_radians(&self) -> Radians {
    match self {
      Angle::Degrees(degrees) => (*degrees).into(),
      Angle::Radians(radians) => radians.clone(),
    }
  }

  pub fn as_degrees(&self) -> Degrees {
    match self {
      Angle::Degrees(degrees) => degrees.clone(),
      Angle::Radians(radians) => (*radians).into(),
    }
  }

  pub fn as_radians_angle(&self) -> Self {
    Self::Radians(self.as_radians())
  }

  pub fn as_degrees_angle(&self) -> Self {
    Self::Degrees(self.as_degrees())
  }
}