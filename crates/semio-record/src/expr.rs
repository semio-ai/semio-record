use std::fmt::Debug;

use derive_more::From;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Expr_Number")]
pub struct Number {
  pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Expr_Variable")]
pub struct Variable {
  pub identifier: String,
}

macro_rules! binary_op {
  ($id:ident, $name:literal) => {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    #[serde(rename = $name)]
    pub struct $id {
      pub left: Box<Expr>,
      pub right: Box<Expr>,
    }
  };
}

macro_rules! unary_op {
  ($id:ident, $name:literal) => {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    #[serde(rename = $name)]
    pub struct $id {
      pub value: Box<Expr>,
    }
  };
}

macro_rules! constant {
  ($id:ident, $name:literal) => {
    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
    #[serde(rename = $name)]
    pub struct $id {
      pub _dummy: i32
    }
  };
}

binary_op!(Add, "Expr_Add");
binary_op!(Subtract, "Expr_Subtract");
binary_op!(Multiply, "Expr_Multiply");
binary_op!(Divide, "Expr_Divide");
binary_op!(Power, "Expr_Power");
unary_op!(Negate, "Expr_Negate");
binary_op!(Logarithm, "Expr_Logarithm");
constant!(Pi, "Expr_Pi");
constant!(Tau, "Expr_Tau");
constant!(E, "Expr_E");
unary_op!(Sine, "Expr_Sine");
unary_op!(Cosine, "Expr_Cosine");
unary_op!(Abs, "Expr_Abs");
unary_op!(Floor, "Expr_Floor");
unary_op!(Ceil, "Expr_Ceil");
unary_op!(Round, "Expr_Round");

#[derive(Debug, Clone, Serialize, Deserialize, From)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Expr", tag = "type")]
pub enum Expr {
  Number(Number),
  Variable(Variable),
  Add(Add),
  Subtract(Subtract),
  Multiply(Multiply),
  Divide(Divide),
  Power(Power),
  Negate(Negate),
  Logarithm(Logarithm),
  Pi(Pi),
  Tau(Tau),
  E(E),
  Sine(Sine),
  Cosine(Cosine),
  Abs(Abs),
  Floor(Floor),
  Ceil(Ceil),
  Round(Round),
}

impl From<f64> for Expr {
  fn from(other: f64) -> Self {
    Self::Number(Number { value: other })
  }
}