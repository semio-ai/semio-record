use derive_more::From;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Centimeters {
  pub value: f64,
}

impl From<Meters> for Centimeters {
  fn from(other: Meters) -> Self {
    Self {
      value: other.value * 100.0,
    }
  }
}

impl From<Inches> for Centimeters {
  fn from(other: Inches) -> Self {
    Self {
      value: other.value * 2.54,
    }
  }
}

impl From<Feet> for Centimeters {
  fn from(other: Feet) -> Self {
    Self {
      value: other.value * 30.48,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Meters {
  pub value: f64,
}

impl From<Centimeters> for Meters {
  fn from(other: Centimeters) -> Self {
    Self {
      value: other.value / 100.0,
    }
  }
}

impl From<Inches> for Meters {
  fn from(other: Inches) -> Self {
    Self {
      value: other.value / 39.37,
    }
  }
}

impl From<Feet> for Meters {
  fn from(other: Feet) -> Self {
    Self {
      value: other.value / 3.281,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Inches {
  pub value: f64,
}

impl From<Centimeters> for Inches {
  fn from(other: Centimeters) -> Self {
    Self {
      value: other.value / 2.54,
    }
  }
}

impl From<Meters> for Inches {
  fn from(other: Meters) -> Self {
    Self {
      value: other.value * 39.37,
    }
  }
}

impl From<Feet> for Inches {
  fn from(other: Feet) -> Self {
    Self {
      value: other.value * 12.0,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Feet {
  pub value: f64,
}

impl From<Centimeters> for Feet {
  fn from(other: Centimeters) -> Self {
    Self {
      value: other.value / 30.48,
    }
  }
}

impl From<Meters> for Feet {
  fn from(other: Meters) -> Self {
    Self {
      value: other.value * 3.281,
    }
  }
}

impl From<Inches> for Feet {
  fn from(other: Inches) -> Self {
    Self {
      value: other.value / 12.0,
    }
  }
} 

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLUnion, From)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Distance {
  Centimeters(Centimeters),
  Meters(Meters),
  Inches(Inches),
  Feet(Feet),
}