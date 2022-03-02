use serde::{Deserialize, Serialize};

use derive_more::{Display, Error};
use uuid::Uuid;

use crate::record::Apply;

pub trait Name {
  fn name(&self) -> &str;
  fn set_name(&mut self, name: String);
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Display)]
pub struct SetName {
  pub name: String,
}

impl SetName {
  pub fn new(name: String) -> Self {
    Self { name }
  }
}

#[derive(Debug, Display, Error, GraphQLEnum)]
pub enum SetNameError {
  #[display(fmt = "Name too short")]
  NameTooShort,
  #[display(fmt = "Name too long")]
  NameTooLong,
  #[display(fmt = "Name contains invalid characters")]
  Invalid,
}

impl<N: Name> Apply<SetName> for N {
  type Error = SetNameError;

  fn mutates_name(_action: &SetName) -> bool {
    true
  }

  fn apply(&mut self, action: &SetName) -> Result<(), Self::Error> {
    if action.name.len() < 3 {
      return Err(SetNameError::NameTooShort);
    }

    if action.name.len() > 32 {
      return Err(SetNameError::NameTooLong);
    }

    if !action.name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
      return Err(SetNameError::Invalid);
    }

    self.set_name(action.name.clone());

    Ok(())
  }
}

macro_rules! name {
  ($t:tt) => {
    impl crate::action::Name for $t {
      fn name(&self) -> &str {
        &self.name
      }

      fn set_name(&mut self, name: String) {
        self.name = name;
      }
    }
  };
}

pub(crate) use name;

pub trait Parent {
  fn parent(&self) -> &Uuid;
  fn set_parent(&mut self, parent: Uuid);
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct SetParent {
  pub id: Uuid,
}

impl SetParent {
  pub fn new(id: Uuid) -> Self {
    Self { id }
  }
}

#[derive(Debug, Display, Error, GraphQLEnum)]
pub enum SetParentError {
  #[display(fmt = "Parent is not a valid UUID")]
  Invalid,
}

impl<P: Parent> Apply<SetParent> for P {
  type Error = SetParentError;

  fn apply(&mut self, action: &SetParent) -> Result<(), Self::Error> {
    if !action.id.is_nil() {
      return Err(SetParentError::Invalid);
    }

    self.set_parent(action.id);

    Ok(())
  }
}

macro_rules! parent {
  ($t:tt) => {
    impl crate::action::Parent for $t {
      fn parent(&self) -> &uuid::Uuid {
        &self.parent
      }

      fn set_parent(&mut self, parent: uuid::Uuid) {
        self.parent = parent;
      }
    }
  };
}

pub(crate) use parent;