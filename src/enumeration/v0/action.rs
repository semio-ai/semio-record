use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ty::UnfrozenTy, record::Apply, action::{SetName, SetNameError, SetParent, SetParentError}, acl::action::{Action as AclAction, ActionError as AclActionError}};

use derive_more::{Display, From, Error};

use super::unfrozen::{EnumerationVariant, Enumeration};

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct AddVariant  {
  pub id: Uuid,
  pub variant: EnumerationVariant
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum AddVariantError {
  #[display(fmt = "ID already exists")]
  IdAlreadyExists,

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

    if self.variants.contains_key(&action.id) {
      return Err(AddVariantError::IdAlreadyExists);
    }
    
    for variant in self.variants.values() {
      if variant.name == action.variant.name {
        return Err(AddVariantError::NameAlreadyExists);
      }
    }

    self.variants.insert(action.id, action.variant.clone());

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct RemoveVariant  {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
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

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct SetVariantName  {
  pub id: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
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

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct SetVariantType  {
  pub id: Uuid,
  pub ty: UnfrozenTy,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
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

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum Action {
  SetName(SetName),
  SetParent(SetParent),
  AddVariant(AddVariant),
  RemoveVariant(RemoveVariant),
  SetVariantName(SetVariantName),
  SetVariantType(SetVariantType),
  Acl(AclAction),
}

#[derive(Display, Debug, Error, From, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum ActionError {
  SetName(SetNameError),
  SetParent(SetParentError),
  AddVariant(AddVariantError),
  RemoveVariant(RemoveVariantError),
  SetVariantName(SetVariantNameError),
  SetVariantType(SetVariantTypeError),
  Acl(AclActionError),
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
      Action::SetParent(action) => self.apply(action)?,
      Action::AddVariant(action) => self.apply(action)?,
      Action::RemoveVariant(action) => self.apply(action)?,
      Action::SetVariantName(action) => self.apply(action)?,
      Action::SetVariantType(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }

    Ok(())
  }
}

