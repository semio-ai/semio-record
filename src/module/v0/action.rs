
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use derive_more::{Display, Error, From};

use crate::{
  record::{Apply}, action::{SetName, SetNameError, SetParentError, SetParent}, ty::UnfrozenTy,
  acl::action::{Action as AclAction, ActionError as AclActionError},
};

use super::unfrozen::{Export, Module, Parameter};


#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct SetExecutable {
  pub blob_id: Uuid,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum SetExecutableError {
  _Dummy
}

impl Apply<SetExecutable> for Module
{
  type Error = SetExecutableError;

  fn apply(&mut self, action: &SetExecutable) -> Result<(), Self::Error> {
    self.executable = Some(action.blob_id);
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct AddExport {
  pub export: Export
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum AddExportError
{
  #[display(fmt = "Export name already exists in module")]
  NameAlreadyExists,
}

impl Apply<AddExport> for Module
{
  type Error = AddExportError;

  fn apply(&mut self, action: &AddExport) -> Result<(), Self::Error> {
    if self.has_export_named(&action.export.name) {
      return Err(AddExportError::NameAlreadyExists);
    }

    self.exports.insert(Uuid::new_v4(), action.export.clone());

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct RemoveExport {
  pub id: Uuid,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum RemoveExportError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
}

impl Apply<RemoveExport> for Module
{
  type Error = RemoveExportError;

  fn apply(&mut self, action: &RemoveExport) -> Result<(), Self::Error> {
    if !self.has_export(&action.id) {
      return Err(RemoveExportError::ExportNotFound)?;
    }

    self.remove_export(&action.id);

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct RenameExport {
  pub id: Uuid,
  pub name: String,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum RenameExportError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export name already exists in module")]
  NameAlreadyExists,
}

impl Apply<RenameExport> for Module
{
  type Error = RenameExportError;

  fn apply(&mut self, action: &RenameExport) -> Result<(), Self::Error> {
    // Check if the new name is already taken.
    if let Some(id) = self.export_id(&action.name)
    {
      if *id != action.id {
        return Err(RenameExportError::NameAlreadyExists);
      }
    }

    // We've already verified this ID exists in exports.
    let mut export = self.export_mut(&action.id).unwrap();

    export.name = action.name.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct AppendFunctionParameter {
  pub export: Uuid,
  pub parameter: Parameter,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum AppendFunctionParameterError
{
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export is of the wrong type for this operation")]
  WrongType,
  #[display(fmt = "Parameter name already exists in function")]
  NameAlreadyExists,
}

impl Apply<AppendFunctionParameter> for Module
{
  type Error = AppendFunctionParameterError;

  fn apply(&mut self, action: &AppendFunctionParameter) -> Result<(), Self::Error> {

    let export = self
      .export_mut(&action.export)
      .ok_or(AppendFunctionParameterError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(AppendFunctionParameterError::WrongType)?;

    if func.has_parameter_named(&action.parameter.name) {
      return Err(AppendFunctionParameterError::NameAlreadyExists);
    }

    func.append_parameter(action.parameter.clone());

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct RemoveFunctionParameter {
  pub export: Uuid,
  pub parameter: Uuid,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum RemoveFunctionParameterError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export is of the wrong type for this operation")]
  WrongType,
  #[display(fmt = "Parameter not found in export")]
  ParameterNotFound,
}

impl Apply<RemoveFunctionParameter> for Module
{
  type Error = RemoveFunctionParameterError;

  fn apply(&mut self, action: &RemoveFunctionParameter) -> Result<(), Self::Error> {
    let export = self
      .export_mut(&action.export)
      .ok_or(RemoveFunctionParameterError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(RemoveFunctionParameterError::WrongType)?;

    let _ = func
      .remove_parameter(&action.parameter)
      .ok_or(RemoveFunctionParameterError::ParameterNotFound)?;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct SetFunctionParameterName {
  pub export: Uuid,
  pub parameter: Uuid,
  pub name: String,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum SetFunctionParameterNameError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export is of the wrong type for this operation")]
  WrongType,
  #[display(fmt = "Parameter not found in export")]
  ParameterNotFound,
  #[display(fmt = "Name already taken")]
  NameTaken,
}

impl Apply<SetFunctionParameterName> for Module
{
  type Error = SetFunctionParameterNameError;

  fn apply(&mut self, action: &SetFunctionParameterName) -> Result<(), Self::Error> {
    let export = self
      .export_mut(&action.export)
      .ok_or(SetFunctionParameterNameError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(SetFunctionParameterNameError::WrongType)?;

    if let Some(_) = func.parameter_named(&action.name) {
      return Err(SetFunctionParameterNameError::NameTaken);
    }
      
    let parameter = func
      .parameter_mut(&action.parameter)
      .ok_or(SetFunctionParameterNameError::ParameterNotFound)?;

    parameter.name = action.name.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct SetFunctionParameterType {
  pub export: Uuid,
  pub parameter: Uuid,
  pub ty: UnfrozenTy,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum SetFunctionParameterTypeError
{
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export is of the wrong type for this operation")]
  WrongType,
  #[display(fmt = "Parameter not found in export")]
  ParameterNotFound,
}

impl Apply<SetFunctionParameterType> for Module
{
  type Error = SetFunctionParameterTypeError;

  fn apply(&mut self, action: &SetFunctionParameterType) -> Result<(), Self::Error> {
    let export = self
      .export_mut(&action.export)
      .ok_or(SetFunctionParameterTypeError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(SetFunctionParameterTypeError::WrongType)?;

    let parameter = func
      .parameter_mut(&action.parameter)
      .ok_or(SetFunctionParameterTypeError::ParameterNotFound)?;

    parameter.ty = action.ty.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct SetFunctionParameterMutability {
  pub export: Uuid,
  pub parameter: Uuid,
  pub mutable: bool,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum SetFunctionParameterMutabilityError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export is of the wrong type for this operation")]
  WrongType,
  #[display(fmt = "Parameter not found in export")]
  ParameterNotFound,
}

impl Apply<SetFunctionParameterMutability> for Module
{
  type Error = SetFunctionParameterMutabilityError;

  fn apply(&mut self, action: &SetFunctionParameterMutability) -> Result<(), Self::Error> {
    let export = self
      .export_mut(&action.export)
      .ok_or(SetFunctionParameterMutabilityError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(SetFunctionParameterMutabilityError::WrongType)?;

    let parameter = func
      .parameter_mut(&action.parameter)
      .ok_or(SetFunctionParameterMutabilityError::ParameterNotFound)?;

    parameter.mutable = action.mutable;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct SetFunctionReturnType {
  pub export: Uuid,
  pub ty: UnfrozenTy,
}

#[derive(Display, Debug, Error, GraphQLEnum)]
pub enum SetFunctionReturnTypeError
{
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
}

impl Apply<SetFunctionReturnType> for Module
{
  type Error = SetFunctionReturnTypeError;

  fn apply(&mut self, action: &SetFunctionReturnType) -> Result<(), Self::Error> {
    let export = self
      .export_mut(&action.export)
      .ok_or(SetFunctionReturnTypeError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(SetFunctionReturnTypeError::ExportNotFound)?;

    func.return_ty = action.ty.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
  SetParent(SetParent),
  SetName(SetName),
  SetExecutable(SetExecutable),
  AddExport(AddExport),
  RemoveExport(RemoveExport),
  AppendFunctionParameter(AppendFunctionParameter),
  RemoveFunctionParameter(RemoveFunctionParameter),
  SetFunctionParameterName(SetFunctionParameterName),
  SetFunctionParameterType(SetFunctionParameterType),
  SetFunctionParameterMutability(SetFunctionParameterMutability),
  SetFunctionReturnType(SetFunctionReturnType),
  Acl(AclAction),
}

#[derive(Display, Debug, Error, From)]
pub enum ActionError
{
  SetParent(SetParentError),
  SetName(SetNameError),
  SetExecutable(SetExecutableError),
  AddExport(AddExportError),
  RemoveExport(RemoveExportError),
  AppendFunctionParameter(AppendFunctionParameterError),
  RemoveFunctionParameter(RemoveFunctionParameterError),
  SetFunctionParameterName(SetFunctionParameterNameError),
  SetFunctionParameterType(SetFunctionParameterTypeError),
  SetFunctionParameterMutability(SetFunctionParameterMutabilityError),
  SetFunctionReturnType(SetFunctionReturnTypeError),
  Acl(AclActionError),
}

impl Apply<Action> for Module
{
  type Error = ActionError;

  fn mutates_name(action: &Action) -> bool {
    match action {
      Action::SetName(_) => true,
      _ => false,
    }
  }

  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetParent(action) => self.apply(action)?,
      Action::SetName(action) => self.apply(action)?,
      Action::SetExecutable(action) => self.apply(action)?,
      Action::AddExport(action) => self.apply(action)?,
      Action::RemoveExport(action) => self.apply(action)?,
      Action::AppendFunctionParameter(action) => self.apply(action)?,
      Action::RemoveFunctionParameter(action) => self.apply(action)?,
      Action::SetFunctionParameterName(action) => self.apply(action)?,
      Action::SetFunctionParameterType(action) => self.apply(action)?,
      Action::SetFunctionParameterMutability(action) => self.apply(action)?,
      Action::SetFunctionReturnType(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }

    Ok(())
  }
}
