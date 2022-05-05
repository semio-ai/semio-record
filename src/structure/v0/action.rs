use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ty::UnfrozenTy, record::Apply, action::{SetName, SetNameError, SetParent, SetParentError}, acl::action::{Action as AclAction, ActionError as AclActionError}};

use derive_more::{Display, From, Error};

use super::unfrozen::{StructureField, Structure};

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct AddField {
  pub id: Uuid,
  pub field: StructureField
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum AddFieldError {
  #[display(fmt = "ID already exists")]
  IdAlreadyExists,

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

    if self.fields.contains_key(&action.id) {
      return Err(AddFieldError::IdAlreadyExists);
    }
    
    for field in self.fields.values() {
      if field.name == action.field.name {
        return Err(AddFieldError::NameAlreadyExists);
      }
    }

    self.fields.insert(action.id, action.field.clone());

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct RemoveField  {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum RemoveFieldError {
  #[display(fmt = "Field does not exist")]
  FieldDoesNotExist,
}

impl Apply<RemoveField> for Structure {
  type Error = RemoveFieldError;

  fn apply(&mut self, action: &RemoveField) -> Result<(), Self::Error> {
    if let None = self.fields.remove(&action.id) {
      Err(RemoveFieldError::FieldDoesNotExist)
    } else {
      Ok(())
    }
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct SetFieldName  {
  pub id: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum SetFieldNameError {
  #[display(fmt = "Field does not exist")]
  FieldDoesNotExist,
  #[display(fmt = "Name already taken")]
  NameTaken,
  #[display(fmt = "Name is too short")]
  NameTooShort,
}

impl Apply<SetFieldName> for Structure {
  type Error = SetFieldNameError;

  fn apply(&mut self, action: &SetFieldName) -> Result<(), Self::Error> {
    if action.name.is_empty() {
      return Err(SetFieldNameError::NameTooShort);
    }
    
    if let Some(_) = self.field_named(&action.name) {
      return Err(SetFieldNameError::NameTaken);
    }

    if let Some(field) = self.fields.get_mut(&action.id) {
      field.name = action.name.clone();
      Ok(())
    } else {
      Err(SetFieldNameError::FieldDoesNotExist)
    }
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct SetFieldType  {
  pub id: Uuid,
  pub ty: UnfrozenTy,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum SetFieldTypeError {
  #[display(fmt = "Field does not exist")]
  FieldDoesNotExist,
}

impl Apply<SetFieldType> for Structure {
  type Error = SetFieldTypeError;

  fn apply(&mut self, action: &SetFieldType) -> Result<(), Self::Error> {
    if let Some(field) = self.fields.get_mut(&action.id) {
      field.ty = action.ty.clone();
      Ok(())
    } else {
      Err(SetFieldTypeError::FieldDoesNotExist)
    }
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum Action {
  SetName(SetName),
  SetParent(SetParent),
  AddField(AddField),
  RemoveField(RemoveField),
  SetFieldName(SetFieldName),
  SetFieldType(SetFieldType),
  Acl(AclAction),
}

#[derive(Display, Debug, Error, From, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum ActionError {
  SetName(SetNameError),
  SetParent(SetParentError),
  AddField(AddFieldError),
  RemoveField(RemoveFieldError),
  SetFieldName(SetFieldNameError),
  SetFieldType(SetFieldTypeError),
  Acl(AclActionError)
}

impl Apply<Action> for Structure {
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
      Action::AddField(action) => self.apply(action)?,
      Action::RemoveField(action) => self.apply(action)?,
      Action::SetFieldName(action) => self.apply(action)?,
      Action::SetFieldType(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }

    Ok(())
  }
}

