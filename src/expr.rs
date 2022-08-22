use std::fmt::Debug;

use derive_more::From;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Number {
  pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Variable {
  pub identifier: String,
}

macro_rules! binary_op {
  ($id:ident) => {
    #[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
    pub struct $id {
      pub left: Box<Expr>,
      pub right: Box<Expr>,
    }
  };
}

macro_rules! unary_op {
  ($id:ident) => {
    #[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
    pub struct $id {
      pub value: Box<Expr>,
    }
  };
}

macro_rules! constant {
  ($id:ident) => {
    #[derive(Debug, Copy, Clone, Serialize, Deserialize, GraphQLObject)]
    pub struct $id {
      pub _dummy: i32
    }
  };
}

binary_op!(Add);
binary_op!(Subtract);
binary_op!(Multiply);
binary_op!(Divide);
binary_op!(Power);
unary_op!(Negate);
binary_op!(Logarithm);
constant!(Pi);
constant!(Tau);
constant!(E);
unary_op!(Sine);
unary_op!(Cosine);
unary_op!(Abs);
unary_op!(Floor);
unary_op!(Ceil);
unary_op!(Round);

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLUnion, From)]
#[serde(tag = "type")]
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