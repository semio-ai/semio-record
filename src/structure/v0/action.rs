use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ty::UnfrozenTy, record::Apply};

use derive_more::{Display, From, Error};

use super::unfrozen::{StructureField, Structure};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct AddField  {
  pub field: StructureField
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum)]
pub enum AddFieldError {
  #[display(fmt = "Name already exists")]
  NameAlreadyExists,

  #[display(fmt = "Name is too short")]
  NameTooShort,
}

impl Apply<AddField> for Structure {
  type Error = AddFieldError;

  fn apply(&mut self, action: &AddField) -> Result<(), Self::Error> {
    if action.field.name.is_empty() {
      return Err(AddFieldError::NameTooShort);
    }
    
    for field in self.fields.values() {
      if field.name == action.field.name {
        return Err(AddFieldError::NameAlreadyExists);
      }
    }


    Ok(())
  }
}
