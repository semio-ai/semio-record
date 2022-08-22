use derive_more::{Display, From};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Kilograms {
  pub value: f64,
}

impl From<Grams> for Kilograms {
  fn from(other: Grams) -> Self {
    Self {
      value: other.value / 1000.0,
    }
  }
}

impl From<Pounds> for Kilograms {
  fn from(other: Pounds) -> Self {
    Self {
      value: other.value / 2.2046,
    }
  }
}

impl From<Ounces> for Kilograms {
  fn from(other: Ounces) -> Self {
    Self {
      value: other.value / 35.274,
    }
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Grams {
  pub value: f64,
}

impl From<Kilograms> for Grams {
  fn from(other: Kilograms) -> Self {
    Self {
      value: other.value * 1000.0,
    }
  }
}

impl From<Pounds> for Grams {
  fn from(other: Pounds) -> Self {
    Self {
      value: other.value * 453.592,
    }
  }
}

impl From<Ounces> for Grams {
  fn from(other: Ounces) -> Self {
    Self {
      value: other.value * 28.35,
    }
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Pounds {
  pub value: f64,
}

impl From<Kilograms> for Pounds {
  fn from(other: Kilograms) -> Self {
    Self {
      value: other.value * 2.2046,
    }
  }
}

impl From<Grams> for Pounds {
  fn from(other: Grams) -> Self {
    Self {
      value: other.value / 453.592,
    }
  }
}

impl From<Ounces> for Pounds {
  fn from(other: Ounces) -> Self {
    Self {
      value: other.value / 16.0,
    }
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Ounces {
  pub value: f64,
}

impl From<Kilograms> for Ounces {
  fn from(other: Kilograms) -> Self {
    Self {
      value: other.value * 35.274,
    }
  }
}

impl From<Grams> for Ounces {
  fn from(other: Grams) -> Self {
    Self {
      value: other.value / 28.35,
    }
  }
}

impl From<Pounds> for Ounces {
  fn from(other: Pounds) -> Self {
    Self {
      value: other.value * 16.0,
    }
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLUnion, From)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Mass {
  Kilograms(Kilograms),
  Grams(Grams),
  Pounds(Pounds),
  Ounces(Ounces),
}
