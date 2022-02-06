use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use derive_more::{Display, Error};

use crate::{
  entity::{EntityResolver, UnresolvedReference},
  id::Id,
  patch::Apply,
};

use super::{Export, ExportKind, Function, Module, Parameter};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnresolvedParameter {
  pub name: String,
  pub type_ref: UnresolvedReference,
  pub mutable: bool,
}

impl UnresolvedParameter {
  pub async fn resolve<R: EntityResolver>(&self, resolver: &mut R) -> Result<Parameter, R::Error> {
    Ok(Parameter {
      name: self.name.clone(),
      type_ref: resolver.resolve_entity(&self.type_ref).await?,
      mutable: self.mutable,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnresolvedFunction {
  pub parameters: HashMap<Uuid, UnresolvedParameter>,
  pub parameter_ordering: Vec<Uuid>,
  pub return_type_ref: UnresolvedReference,
}

impl UnresolvedFunction {
  pub async fn resolve<R: EntityResolver>(&self, resolver: &mut R) -> Result<Function, R::Error> {
    let mut parameters = HashMap::new();
    for (id, parameter) in &self.parameters {
      parameters.insert(id.clone(), parameter.resolve(resolver).await?);
    }

    Ok(Function {
      parameters,
      parameter_ordering: self.parameter_ordering.clone(),
      return_type_ref: resolver.resolve_entity(&self.return_type_ref).await?,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum UnresolvedExportKind {
  Function(UnresolvedFunction),
}

impl UnresolvedExportKind {
  pub async fn resolve<R: EntityResolver>(&self, resolver: &mut R) -> Result<ExportKind, R::Error> {
    match self {
      Self::Function(function) => Ok(ExportKind::Function(function.resolve(resolver).await?)),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnresolvedExport {
  pub name: String,
  pub kind: UnresolvedExportKind,
}

impl UnresolvedExport {
  pub async fn resolve<R: EntityResolver>(&self, resolver: &mut R) -> Result<Export, R::Error> {
    Ok(Export {
      name: self.name.clone(),
      kind: self.kind.resolve(resolver).await?,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetExecutable(Uuid);

#[derive(Display, Debug, Error)]
pub enum SetExecutableError {}

#[async_trait]
impl<C> Apply<SetExecutable, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), RenameExportError>;

  async fn apply(&mut self, _: &mut C, action: &SetExecutable) -> Self::Result {
    self.executable = Some(action.0);
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddExport(pub UnresolvedExport);

#[derive(Display, Debug, Error)]
pub enum AddExportError<C>
where
  C: EntityResolver,
{
  ResolverError(C::Error),
  #[display(fmt = "Export name already exists in module")]
  NameAlreadyExists,
}

#[async_trait]
impl<C> Apply<AddExport, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), AddExportError<C>>;

  async fn apply(&mut self, context: &mut C, action: &AddExport) -> Self::Result {
    if !self.contains_export(&Id::Name(action.0.name.clone())) {
      return Err(AddExportError::NameAlreadyExists);
    }

    let export = action
      .0
      .resolve(context)
      .await
      .map_err(|e| AddExportError::ResolverError(e))?;

    self.exports.insert(Uuid::new_v4(), export);

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveExport(pub Id);

#[derive(Display, Debug, Error)]
pub enum RemoveExportError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
}

#[async_trait]
impl<C> Apply<RemoveExport, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), RemoveExportError>;

  async fn apply(&mut self, _: &mut C, action: &RemoveExport) -> Self::Result {
    let id = self
      .get_export_id(&action.0)
      .ok_or(RemoveExportError::ExportNotFound)?;

    self.exports.remove(&id);

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameExport {
  pub id: Id,
  pub name: String,
}

#[derive(Display, Debug, Error)]
pub enum RenameExportError {
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export name already exists in module")]
  NameAlreadyExists,
}

#[async_trait]
impl<C> Apply<RenameExport, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), RenameExportError>;

  async fn apply(&mut self, _: &mut C, action: &RenameExport) -> Self::Result {
    let id = self
      .get_export_id(&action.id)
      .ok_or(RenameExportError::ExportNotFound)?;

    // Check if the new name is already taken.
    if self
      .exports
      .iter()
      .any(|(export_id, export)| *export_id != id && export.name == action.name)
    {
      return Err(RenameExportError::NameAlreadyExists);
    }

    // We've already verified this ID exists in exports.
    let mut export = self.exports.get_mut(&id).unwrap();

    export.name = action.name.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppendFunctionParameter {
  pub export: Id,
  pub parameter: UnresolvedParameter,
}

#[derive(Display, Debug, Error)]
pub enum AppendFunctionParameterError<C>
where
  C: EntityResolver,
{
  ResolverError(C::Error),
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export is of the wrong type for this operation")]
  WrongType,
  #[display(fmt = "Parameter name already exists in function")]
  NameAlreadyExists,
}

#[async_trait]
impl<C> Apply<AppendFunctionParameter, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), AppendFunctionParameterError<C>>;

  async fn apply(&mut self, context: &mut C, action: &AppendFunctionParameter) -> Self::Result {
    let export = self
      .get_export_mut(&action.export)
      .ok_or(AppendFunctionParameterError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(AppendFunctionParameterError::WrongType)?;

    if func.contains_parameter(&Id::Name(action.parameter.name.clone())) {
      return Err(AppendFunctionParameterError::NameAlreadyExists);
    }

    let parameter = action
      .parameter
      .resolve(context)
      .await
      .map_err(|e| AppendFunctionParameterError::ResolverError(e))?;

    let parameter_id = Uuid::new_v4();
    func.parameters.insert(parameter_id.clone(), parameter);
    func.parameter_ordering.push(parameter_id);

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveFunctionParameter {
  pub export: Id,
  pub parameter: Id,
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

#[async_trait]
impl<C> Apply<RemoveFunctionParameter, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<Parameter, RemoveFunctionParameterError>;

  async fn apply(&mut self, _: &mut C, action: &RemoveFunctionParameter) -> Self::Result {
    let export = self
      .get_export_mut(&action.export)
      .ok_or(RemoveFunctionParameterError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(RemoveFunctionParameterError::WrongType)?;

    let ret = func
      .remove_parameter(&action.parameter)
      .ok_or(RemoveFunctionParameterError::ParameterNotFound)?;

    Ok(ret)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFunctionParameterName {
  pub export: Id,
  pub parameter: Id,
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

#[async_trait]
impl<C> Apply<SetFunctionParameterName, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), SetFunctionParameterNameError>;

  async fn apply(&mut self, _: &mut C, action: &SetFunctionParameterName) -> Self::Result {
    let export = self
      .get_export_mut(&action.export)
      .ok_or(SetFunctionParameterNameError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(SetFunctionParameterNameError::WrongType)?;

    let parameter = func
      .get_parameter_mut(&action.parameter)
      .ok_or(SetFunctionParameterNameError::ParameterNotFound)?;

    parameter.name = action.name.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFunctionParameterType {
  pub export: Id,
  pub parameter: Id,
  pub type_ref: UnresolvedReference,
}

#[derive(Display, Debug, Error)]
pub enum SetFunctionParameterTypeError<C>
where
  C: EntityResolver,
{
  ResolverError(C::Error),
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
  #[display(fmt = "Export is of the wrong type for this operation")]
  WrongType,
  #[display(fmt = "Parameter not found in export")]
  ParameterNotFound,
}

#[async_trait]
impl<C> Apply<SetFunctionParameterType, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), SetFunctionParameterTypeError<C>>;

  async fn apply(&mut self, context: &mut C, action: &SetFunctionParameterType) -> Self::Result {
    let export = self
      .get_export_mut(&action.export)
      .ok_or(SetFunctionParameterTypeError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(SetFunctionParameterTypeError::WrongType)?;

    let parameter = func
      .get_parameter_mut(&action.parameter)
      .ok_or(SetFunctionParameterTypeError::ParameterNotFound)?;

    let type_ref = context
      .resolve_entity(&action.type_ref)
      .await
      .map_err(|e| SetFunctionParameterTypeError::ResolverError(e))?;

    parameter.type_ref = type_ref;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFunctionParameterMutability {
  pub export: Id,
  pub parameter: Id,
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

#[async_trait]
impl<C> Apply<SetFunctionParameterMutability, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), SetFunctionParameterMutabilityError>;

  async fn apply(&mut self, _: &mut C, action: &SetFunctionParameterMutability) -> Self::Result {
    let export = self
      .get_export_mut(&action.export)
      .ok_or(SetFunctionParameterMutabilityError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(SetFunctionParameterMutabilityError::WrongType)?;

    let parameter = func
      .get_parameter_mut(&action.parameter)
      .ok_or(SetFunctionParameterMutabilityError::ParameterNotFound)?;

    parameter.mutable = action.mutable;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFunctionReturnType {
  pub export: Id,
  pub type_ref: UnresolvedReference,
}

#[derive(Display, Debug, Error)]
pub enum SetFunctionReturnTypeError<C>
where
  C: EntityResolver,
{
  ResolverError(C::Error),
  #[display(fmt = "Export not found in module")]
  ExportNotFound,
}

#[async_trait]
impl<C> Apply<SetFunctionReturnType, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), SetFunctionReturnTypeError<C>>;

  async fn apply(&mut self, context: &mut C, action: &SetFunctionReturnType) -> Self::Result {
    let export = self
      .get_export_mut(&action.export)
      .ok_or(SetFunctionReturnTypeError::ExportNotFound)?;

    let func = export
      .kind
      .as_function_mut()
      .ok_or(SetFunctionReturnTypeError::ExportNotFound)?;

    let type_ref = context
      .resolve_entity(&action.type_ref)
      .await
      .map_err(|e| SetFunctionReturnTypeError::ResolverError(e))?;

    func.return_type_ref = type_ref;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Display, Debug, Error)]
pub enum ActionError<C>
where
  C: EntityResolver,
{
  AddExport(AddExportError<C>),
  RemoveExport(RemoveExportError),
  AppendFunctionParameter(AppendFunctionParameterError<C>),
  RemoveFunctionParameter(RemoveFunctionParameterError),
  SetFunctionParameterName(SetFunctionParameterNameError),
  SetFunctionParameterType(SetFunctionParameterTypeError<C>),
  SetFunctionParameterMutability(SetFunctionParameterMutabilityError),
  SetFunctionReturnType(SetFunctionReturnTypeError<C>),
}

#[async_trait]
impl<C> Apply<Action, C> for Module
where
  C: EntityResolver + Send,
{
  type Result = Result<(), ActionError<C>>;

  async fn apply(&mut self, context: &mut C, action: &Action) -> Self::Result {
    match action {
      Action::AddExport(action) => self
        .apply(context, action)
        .await
        .map_err(ActionError::AddExport),
      Action::RemoveExport(action) => self
        .apply(context, action)
        .await
        .map_err(ActionError::RemoveExport),
      Action::AppendFunctionParameter(action) => self
        .apply(context, action)
        .await
        .map_err(ActionError::AppendFunctionParameter),
      Action::RemoveFunctionParameter(action) => self
        .apply(context, action)
        .await
        .map_err(ActionError::RemoveFunctionParameter)
        .map(|_| ()),
      Action::SetFunctionParameterName(action) => self
        .apply(context, action)
        .await
        .map_err(ActionError::SetFunctionParameterName),
      Action::SetFunctionParameterType(action) => self
        .apply(context, action)
        .await
        .map_err(ActionError::SetFunctionParameterType),
      Action::SetFunctionParameterMutability(action) => self
        .apply(context, action)
        .await
        .map_err(ActionError::SetFunctionParameterMutability),
      Action::SetFunctionReturnType(action) => self
        .apply(context, action)
        .await
        .map_err(ActionError::SetFunctionReturnType),
    }
  }
}
