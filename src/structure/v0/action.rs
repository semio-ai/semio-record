use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ty::UnfrozenTy, record::Apply};

use derive_more::{Display, From, Error};

use super::unfrozen::{StructureField, Structure};

#[derive(Debug, Serialize, Deserialize, Display, GraphQLObject)]
pub struct AddField  {
  pub field: StructureField
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum)]
pub enum AddFieldError {
  #[display(fmt = "Name is too short")]
  NameTooShort,
}

impl Apply<AddField> for Structure {
  type Error = AddFieldError;

  fn apply(&mut self, action: &AddField) -> Result<(), Self::Error> {
    if action.token_secret.len() < 64 {
      return Err(SetTokenSecretError::TooShort);
    }

    self.token_secret = action.token_secret.clone();

    Ok(())
  }
}
