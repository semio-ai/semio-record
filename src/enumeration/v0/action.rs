use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ty::UnfrozenTy, record::Apply, action::{SetName, SetNameError}};

use derive_more::{Display, From, Error};

use super::unfrozen::{EnumerationVariant, Enumeration};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct AddVariant  {
  pub variant: EnumerationVariant
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum)]
pub enum AddVariantError {
  #[display(fmt = "Name already exists")]
  NameAlreadyExists,

  #[display(fmt = "Name is too short")]
  NameTooShort,
}

impl Apply<AddVariant> for Enumeration {
  type Error = AddVariantError;

  fn apply(&mut self, action: &AddVariant) -> Result<(), Self::Error> {
    if action.variant.name.is_empty() {
      return Err(AddVariantError::NameTooShort);
    }
    
    for variant in self.variants.values() {
      if variant.name == action.variant.name {
        return Err(AddVariantError::NameAlreadyExists);
      }
    }


    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct RemoveVariant  {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum)]
pub enum RemoveVariantError {
  #[display(fmt = "Variant does not exist")]
  VariantDoesNotExist,
}

impl Apply<RemoveVariant> for Enumeration {
  type Error = RemoveVariantError;

  fn apply(&mut self, action: &RemoveVariant) -> Result<(), Self::Error> {
    if let None = self.variants.remove(&action.id) {
      Err(RemoveVariantError::VariantDoesNotExist)
    } else {
      Ok(())
    }
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct SetVariantName  {
  pub id: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum)]
pub enum SetVariantNameError {
  #[display(fmt = "Variant does not exist")]
  VariantDoesNotExist,
  #[display(fmt = "Name already taken")]
  NameTaken,
  #[display(fmt = "Name is too short")]
  NameTooShort,
}

impl Apply<SetVariantName> for Enumeration {
  type Error = SetVariantNameError;

  fn apply(&mut self, action: &SetVariantName) -> Result<(), Self::Error> {
    if action.name.is_empty() {
      return Err(SetVariantNameError::NameTooShort);
    }
    
    if let Some(_) = self.variant_named(&action.name) {
      return Err(SetVariantNameError::NameTaken);
    }

    if let Some(variant) = self.variants.get_mut(&action.id) {
      variant.name = action.name.clone();
      Ok(())
    } else {
      Err(SetVariantNameError::VariantDoesNotExist)
    }
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct SetVariantType  {
  pub id: Uuid,
  pub ty: UnfrozenTy,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum)]
pub enum SetVariantTypeError {
  #[display(fmt = "Variant does not exist")]
  VariantDoesNotExist,
}

impl Apply<SetVariantType> for Enumeration {
  type Error = SetVariantTypeError;

  fn apply(&mut self, action: &SetVariantType) -> Result<(), Self::Error> {
    if let Some(variant) = self.variants.get_mut(&action.id) {
      variant.ty = action.ty.clone();
      Ok(())
    } else {
      Err(SetVariantTypeError::VariantDoesNotExist)
    }
  }
}

#[derive(Debug, Serialize, Deserialize, From, GraphQLUnion)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
  SetName(SetName),
  AddVariant(AddVariant),
  RemoveVariant(RemoveVariant),
  SetVariantName(SetVariantName),
  SetVariantType(SetVariantType),
}

#[derive(Display, Debug, Error, From)]
pub enum ActionError {
  SetName(SetNameError),
  AddVariant(AddVariantError),
  RemoveVariant(RemoveVariantError),
  SetVariantName(SetVariantNameError),
  SetVariantType(SetVariantTypeError),
}

impl Apply<Action> for Enumeration {
  type Error = ActionError;

  fn mutates_name(action: &Action) -> bool {
    match action {
      Action::SetName(_) => true,
      _ => false,
    }
  }
  
  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetName(action) => self.apply(action)?,
      Action::AddVariant(action) => self.apply(action)?,
      Action::RemoveVariant(action) => self.apply(action)?,
      Action::SetVariantName(action) => self.apply(action)?,
      Action::SetVariantType(action) => self.apply(action)?,
    }

    Ok(())
  }
}

