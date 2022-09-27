
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use derive_more::{Display, Error, From};

use schemars::JsonSchema;

use crate::{
  record::{Apply, UnfrozenReference}, action::{SetName, SetNameError, SetParentError, SetParent}, ty::UnfrozenTy,
  acl::action::{Action as AclAction, ActionError as AclActionError},
};

use super::unfrozen::{Export, Module, Parameter};


#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_SetExecutable")]
pub struct SetExecutable {
  pub blob_id: Uuid,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_SetExecutableError", tag = "type", rename_all = "snake_case")]
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_AddExport")]
pub struct AddExport {
  pub id: Uuid,
  pub export: Export
}

#[derive(Display, Clone,Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_AddExportError", tag = "type", rename_all = "snake_case")]
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

    self.exports.insert(action.id, action.export.clone());

    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
pub struct RemoveExport {
  pub id: Uuid,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_RenameExport")]
pub struct RenameExport {
  pub id: Uuid,
  pub name: String,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_RenameExportError", tag = "type", rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_AppendFunctionParameter")]
pub struct AppendFunctionParameter {
  pub export: Uuid,
  pub parameter_id: Uuid,
  pub parameter: Parameter,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_AppendFunctionParameterError", tag = "type", rename_all = "camelCase")]
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

    func.append_parameter(action.parameter_id, action.parameter.clone());

    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_RemoveFunctionParameter")]
pub struct RemoveFunctionParameter {
  pub export: Uuid,
  pub parameter: Uuid,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_RemoveFunctionParameterError", tag = "type", rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_SetFunctionParameterName", rename_all = "camelCase")]
pub struct SetFunctionParameterName {
  pub export: Uuid,
  pub parameter: Uuid,
  pub name: String,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_SetFunctionParameterNameError", tag = "type", rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_SetFunctionParameterType")]
pub struct SetFunctionParameterType {
  pub export: Uuid,
  pub parameter: Uuid,
  pub ty: UnfrozenTy,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_SetFunctionParameterTypeError", tag = "type", rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_SetFunctionParameterMutability")]
pub struct SetFunctionParameterMutability {
  pub export: Uuid,
  pub parameter: Uuid,
  pub mutable: bool,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_SetFunctionParameterMutabilityError", tag = "type", rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_SetFunctionReturnType")]
pub struct SetFunctionReturnType {
  pub export: Uuid,
  pub ty: UnfrozenTy,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_SetFunctionReturnTypeError", tag = "type", rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
#[serde(rename = "module_v0_Action_AddDependency")]
pub struct AddDependency {
  pub dependency: UnfrozenReference,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "module_v0_Action_AddDependencyError", tag = "type", rename_all = "camelCase")]
pub enum AddDependencyError
{
  #[display(fmt = "Dependency already exists")]
  AlreadyExists,
  #[display(fmt = "Too many dependencies")]
  TooMany,
}

impl Apply<AddDependency> for Module
{
  type Error = AddDependencyError;

  fn apply(&mut self, action: &AddDependency) -> Result<(), Self::Error> {
    if self.dependencies.len() >= 32 {
      return Err(AddDependencyError::TooMany);
    }

    if self.dependencies.iter().any(|d| d.id == action.dependency.id) {
      return Err(AddDependencyError::AlreadyExists);
    }

    self.dependencies.push(action.dependency.clone());

    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, JsonSchema)]
pub struct RemoveDependency {
  pub id: Uuid,
}

#[derive(Display, Clone, Debug, Error, GraphQLEnum, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum RemoveDependencyError {
  #[display(fmt = "Dependency not found")]
  NotFound,
}

impl Apply<RemoveDependency> for Module
{
  type Error = RemoveDependencyError;

  fn apply(&mut self, action: &RemoveDependency) -> Result<(), Self::Error> {
    let index = self
      .dependencies
      .iter()
      .position(|d| d.id == action.id)
      .ok_or(RemoveDependencyError::NotFound)?;

    self.dependencies.remove(index);

    Ok(())
  }
}


#[derive(Debug, Clone, Serialize, Deserialize, From, JsonSchema)]
#[serde(rename = "module_v0_Action", tag = "type", rename_all = "snake_case", content = "value")]
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
  AddDependency(AddDependency),
  RemoveDependency(RemoveDependency),
  Acl(AclAction),
}

#[derive(Display, Clone, Debug, Error, Serialize, Deserialize, From, JsonSchema)]
#[serde(rename = "module_v0_ActionError", tag = "type", rename_all = "camelCase")]
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
  AddDependency(AddDependencyError),
  RemoveDependency(RemoveDependencyError),
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
      Action::AddDependency(action) => self.apply(action)?,
      Action::RemoveDependency(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }

    Ok(())
  }
}
