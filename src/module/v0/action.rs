
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use derive_more::{Display, Error, From};

use crate::{
  record::{Apply, UnfrozenReference},
};

use super::unfrozen::{Export, Module, Parameter};


#[derive(Debug, Serialize, Deserialize)]
pub struct SetExecutable(pub Uuid);

#[derive(Display, Debug, Error)]
pub enum SetExecutableError {}

impl Apply<SetExecutable> for Module
{
  type Error = RenameExportError;

  fn apply(&mut self, action: &SetExecutable) -> Result<(), Self::Error> {
    self.executable = Some(action.0);
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddExport(pub Export);

#[derive(Display, Debug, Error)]
pub enum AddExportError
{
  #[display(fmt = "Export name already exists in module")]
  NameAlreadyExists,
}

impl Apply<AddExport> for Module
{
  type Error = AddExportError;

  fn apply(&mut self, action: &AddExport) -> Result<(), Self::Error> {
    if !self.has_export_named(&action.0.name) {
      return Err(AddExportError::NameAlreadyExists);
    }

    self.exports.insert(Uuid::new_v4(), action.0.clone());

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveExport(pub Uuid);

#[derive(Display, Debug, Error)]
pub enum RemoveExportError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
}

impl Apply<RemoveExport> for Module
{
  type Error = RemoveExportError;

  fn apply(&mut self, action: &RemoveExport) -> Result<(), Self::Error> {
    if !self.has_export(&action.0) {
      return Err(RemoveExportError::ExportNotFound)?;
    }

    self.remove_export(&action.0);

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameExport {
  pub id: Uuid,
  pub name: String,
}

#[derive(Display, Debug, Error)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AppendFunctionParameter {
  pub export: Uuid,
  pub parameter: Parameter,
}

#[derive(Display, Debug, Error)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveFunctionParameter {
  pub export: Uuid,
  pub parameter: Uuid,
}

#[derive(Display, Debug, Error)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFunctionParameterName {
  pub export: Uuid,
  pub parameter: Uuid,
  pub name: String,
}

#[derive(Display, Debug, Error)]
pub enum SetFunctionParameterNameError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export is of the wrong type for this operation")]
  WrongType,
  #[display(fmt = "Parameter not found in export")]
  ParameterNotFound,
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

    let parameter = func
      .parameter_mut(&action.parameter)
      .ok_or(SetFunctionParameterNameError::ParameterNotFound)?;

    parameter.name = action.name.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFunctionParameterType {
  pub export: Uuid,
  pub parameter: Uuid,
  pub type_ref: UnfrozenReference,
}

#[derive(Display, Debug, Error)]
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

    parameter.type_ref = action.type_ref.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFunctionParameterMutability {
  pub export: Uuid,
  pub parameter: Uuid,
  pub mutable: bool,
}

#[derive(Display, Debug, Error)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFunctionReturnType {
  pub export: Uuid,
  pub type_ref: UnfrozenReference,
}

#[derive(Display, Debug, Error)]
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

    func.return_type_ref = action.type_ref.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
  AddExport(AddExport),
  RemoveExport(RemoveExport),
  AppendFunctionParameter(AppendFunctionParameter),
  RemoveFunctionParameter(RemoveFunctionParameter),
  SetFunctionParameterName(SetFunctionParameterName),
  SetFunctionParameterType(SetFunctionParameterType),
  SetFunctionParameterMutability(SetFunctionParameterMutability),
  SetFunctionReturnType(SetFunctionReturnType),
}

#[derive(Display, Debug, Error, From)]
pub enum ActionError
{
  AddExport(AddExportError),
  RemoveExport(RemoveExportError),
  AppendFunctionParameter(AppendFunctionParameterError),
  RemoveFunctionParameter(RemoveFunctionParameterError),
  SetFunctionParameterName(SetFunctionParameterNameError),
  SetFunctionParameterType(SetFunctionParameterTypeError),
  SetFunctionParameterMutability(SetFunctionParameterMutabilityError),
  SetFunctionReturnType(SetFunctionReturnTypeError),
}

impl Apply<Action> for Module
{
  type Error = ActionError;

  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::AddExport(action) => self.apply(action)?,
      Action::RemoveExport(action) => self.apply(action)?,
      Action::AppendFunctionParameter(action) => self.apply(action)?,
      Action::RemoveFunctionParameter(action) => self.apply(action)?,
      Action::SetFunctionParameterName(action) => self.apply(action)?,
      Action::SetFunctionParameterType(action) => self.apply(action)?,
      Action::SetFunctionParameterMutability(action) => self.apply(action)?,
      Action::SetFunctionReturnType(action) => self.apply(action)?,
    }

    Ok(())
  }
}
