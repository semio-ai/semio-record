use std::{collections::{HashMap, HashSet}};

use async_trait::async_trait;
use juniper::{GraphQLUnion, GraphQLObject, FromInputValue, ScalarValue, InputValue, marker::IsInputType};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{record::{UnfrozenReference, Freeze, Freezer, View, Unfrozen}, blob::BlobDependencies, unfrozen::impl_unfrozen, action::{name, parent}, ty::UnfrozenTy, acl::{Acl, action::with_acl}};

use super::{frozen, action::Action};

use derive_more::From;

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Parameter {
  pub name: String,
  #[serde(rename = "type")]
  #[graphql(name = "type")]
  pub ty: UnfrozenTy,
  pub mutable: bool,
}

impl<S: ScalarValue> IsInputType<S> for Parameter {}


impl <S: ScalarValue> FromInputValue<S> for Parameter {
  fn from_input_value(v: &InputValue<S>) -> Option<Self> {
    match v {
      InputValue::Object(object) => {
        let mut name = None;
        let mut ty = None;
        let mut mutable = None;
        for (key, value) in object {
          match key.item.as_str() {
            "name" => {
              name = Some(value.item.as_string_value()?.to_string());
            }
            "type" => {
              ty = UnfrozenTy::from_input_value(&value.item);
            },
            "mutable" => {
              mutable = Some(value.item.as_scalar()?.as_boolean()?);
            },
            _ => None?
          }
        }

        Some(Parameter {
          name: name?,
          ty: ty?,
          mutable: mutable.unwrap_or(false),
        })
      },
      _ => None,
    }
  }
}

impl Parameter {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    self.ty.dependencies(set);
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Parameter {
  type Frozen = frozen::Parameter;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    Ok(Self::Frozen {
      name: self.name.clone(),
      ty: self.ty.freeze(freezer).await?,
      mutable: self.mutable,
    })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
  pub parameters: HashMap<Uuid, Parameter>,
  pub parameter_ordering: Vec<Uuid>,
  #[serde(rename = "returnType")]
  pub return_ty: UnfrozenTy,
}



impl Function {
  pub fn parameter_id(&self, name: &str) -> Option<&Uuid> {
    self
      .parameters
      .iter()
      .find(|(_, parameter)| parameter.name == *name)
      .map(|(id, _)| id)
  }

  pub fn has_parameter(&self, id: &Uuid) -> bool {
    self.parameters.contains_key(id)
  }

  pub fn has_parameter_named(&self, name: &str) -> bool {
    self.parameter_id(name).is_some()
  }

  pub fn parameter(&self, id: &Uuid) -> Option<&Parameter> {
    self.parameters.get(id)
  }
  
  pub fn parameter_named(&self, name: &str) -> Option<&Parameter> {
    self.parameter(self.parameter_id(name)?)
  }

  pub fn parameter_mut(&mut self, id: &Uuid) -> Option<&mut Parameter> {
    self.parameters.get_mut(id)
  }

  pub fn parameter_named_mut(&mut self, name: &str) -> Option<&mut Parameter> {
    self.parameter_mut(&self.parameter_id(name)?.clone())
  }

  pub fn remove_parameter(&mut self, id: &Uuid) -> Option<Parameter> {
    self.parameter_ordering.retain(|&parameter_id| parameter_id != *id);
    self.parameters.remove(id)
  }

  pub fn remove_parameter_named(&mut self, name: &str) -> Option<Parameter> {
    self.remove_parameter(&self.parameter_id(name)?.clone())
  }

  pub fn append_parameter(&mut self, id: Uuid, parameter: Parameter) {
    self.parameters.insert(id.clone(), parameter);
    self.parameter_ordering.push(id);
  }

  pub fn insert_parameter(&mut self, index: usize, id: Uuid, parameter: Parameter) {
    self.parameters.insert(id.clone(), parameter);
    self.parameter_ordering.insert(index, id);
  }

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    for (_, parameter) in &self.parameters {
      parameter.dependencies(set);
    }
    self.return_ty.dependencies(set);
  }
}

#[derive(GraphQLObject)]
pub struct IdParameter {
  pub id: Uuid,
  pub parameter: Parameter,
}

impl From<(Uuid, Parameter)> for IdParameter {
  fn from(pair: (Uuid, Parameter)) -> Self {
    Self {
      id: pair.0,
      parameter: pair.1,
    }
  }
}

impl<S: ScalarValue> IsInputType<S> for IdParameter {}


impl<S: ScalarValue> FromInputValue<S> for IdParameter {
  fn from_input_value(v: &InputValue<S>) -> Option<Self> {
    match v {
      InputValue::Object(object) => {
        let mut id = None;
        let mut parameter = None;
        for (key, value) in object {
          match key.item.as_str() {
            "id" => {
              id = Some(Uuid::parse_str(value.item.as_string_value()?).ok()?);
            },
            "parameter" => {
              parameter = Parameter::from_input_value(&value.item);
            },
            _ => {
              return None;
            }
          }
        }

        let id = if let Some(id) = id {
          id
        } else {
          return None;
        };

        let parameter = if let Some(parameter) = parameter {
          parameter
        } else {
          return None;
        };

        Some(Self {
          id,
          parameter,
        })
      },
      _ => None,
    }
  }
}

#[graphql_object]
impl Function {
  #[graphql(name = "parameters")]
  pub fn gql_parameters(&self) -> Vec<IdParameter> {
    self.parameters.iter().map(|(k, v)| (k.clone(), v.clone()).into()).collect()
  }

  #[graphql(name = "parameterId")]
  pub fn gql_parameter_id(&self, id: String) -> Option<Uuid> {
    self.parameter_id(&id).map(Clone::clone)
  }

  #[graphql(name = "hasParameter")]
  pub fn gql_has_parameter(&self, id: Uuid) -> bool {
    self.has_parameter(&id)
  }

  #[graphql(name = "hasParameterNamed")]
  pub fn gql_has_parameter_named(&self, name: String) -> bool {
    self.has_parameter_named(&name)
  }

  #[graphql(name = "parameter")]
  pub fn gql_parameter(&self, id: Uuid) -> Option<&Parameter> {
    self.parameter(&id)
  }

  #[graphql(name = "parameterNamed")]
  pub fn gql_parameter_named(&self, name: String) -> Option<&Parameter> {
    self.parameter_named(&name)
  }

  #[graphql(name = "returnType")]
  pub fn gql_return_ty(&self) -> UnfrozenTy {
    self.return_ty.clone()
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Function {
  type Frozen = frozen::Function;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    let mut parameters = HashMap::with_capacity(self.parameters.len());
    for (id, parameter) in &self.parameters {
      parameters.insert(id.clone(), parameter.freeze(freezer).await?);
    }

    Ok(Self::Frozen {
      parameters,
      parameter_ordering: self.parameter_ordering.clone(),
      return_ty: self.return_ty.freeze(freezer).await?,
    })
  }
}

impl<S: ScalarValue> IsInputType<S> for Function {}

impl<S: ScalarValue> FromInputValue<S> for Function {
  fn from_input_value(value: &InputValue<S>) -> Option<Self> {
    match value {
      InputValue::Object(object) => {
        let mut parameters = HashMap::new();
        let mut parameter_ordering = Vec::new();
        let mut return_ty = None;

        for (key, value) in object {
          match key.item.as_str() {
            "parameters" => {
              if let InputValue::List(list) = &value.item {
                for value in list {
                  let id_parameter = IdParameter::from_input_value(&value.item)?;
                  parameters.insert(id_parameter.id.clone(), id_parameter.parameter);
                  parameter_ordering.push(id_parameter.id);
                }
              } else {
                None?;
              }
            }
            "returnTypeRef" => {
              return_ty = Some(UnfrozenTy::from_input_value(&value.item)?);
            }
            _ => None?
          }
        }

        Some(Self {
          parameters,
          parameter_ordering,
          return_ty: return_ty?,
        })
      },
      _ => None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLUnion, From)]
#[serde(tag = "type", rename_all = "lowercase", content = "value")]
pub enum ExportKind {
  Function(Function),
}

impl ExportKind {
  pub fn as_function(&self) -> Option<&Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  pub fn as_function_mut(&mut self) -> Option<&mut Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  pub fn to_function(self) -> Option<Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    match self {
      Self::Function(function) => function.dependencies(set),
    }
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for ExportKind {
  type Frozen = frozen::ExportKind;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    match self {
      Self::Function(function) => Ok(Self::Frozen::Function(function.freeze(freezer).await?)),
    }
  } 
}

impl<S: ScalarValue> FromInputValue<S> for ExportKind {
  fn from_input_value(value: &InputValue<S>) -> Option<Self> {
    Function::from_input_value(value).map(Self::Function)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Export {
  pub name: String,
  pub kind: ExportKind,
}

impl<S: ScalarValue> IsInputType<S> for Export {}

impl<S: ScalarValue> FromInputValue<S> for Export {
  fn from_input_value(value: &InputValue<S>) -> Option<Self> {
    match value {
      InputValue::Object(object) => {
        let mut name = None;
        let mut kind = None;
        for (key, value) in object {
          match key.item.as_str() {
            "name" => {
              if let Some(gname) = value.item.as_string_value() {
                name = Some(gname.to_string());
              } else {
                return None;
              }
            },
            "kind" => {
              kind = Some(ExportKind::from_input_value(&value.item)?);
            },
            _ => {}
          }
        }

        if name.is_none() || kind.is_none() {
          return None;
        }

        Some(Self {
          name: name.unwrap(),
          kind: kind.unwrap(),
        })
      }
      _ => None,
    }
  }
}

impl Export {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    self.kind.dependencies(set)
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Export {
  type Frozen = frozen::Export;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    let kind = self.kind.freeze(freezer).await?;
    Ok(Self::Frozen {
      name: self.name.clone(),
      kind,
    })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Module {
  pub acl: Acl,
  pub parent: Uuid,
  pub name: String,
  pub exports: HashMap<Uuid, Export>,
  pub executable: Option<Uuid>,
  pub dependencies: Vec<UnfrozenReference>,
}

impl Module {
  pub fn export_id(&self, name: &str) -> Option<&Uuid> {
    self
      .exports
      .iter()
      .find(|(_, export)| export.name == *name)
      .map(|(id, _)| id)
  }
  
  pub fn has_export(&self, id: &Uuid) -> bool {
    self.exports.contains_key(id)
  }

  pub fn has_export_named(&self, name: &str) -> bool {
    self.export_id(name).is_some()
  }

  pub fn export(&self, id: &Uuid) -> Option<&Export> {
    self.exports.get(id)
  }

  pub fn export_named(&self, name: &str) -> Option<&Export> {
    self.exports.get(&self.export_id(name)?.clone())
  }

  pub fn export_mut(&mut self, id: &Uuid) -> Option<&mut Export> {
    self.exports.get_mut(id)
  }

  pub fn export_named_mut(&mut self, name: &str) -> Option<&mut Export> {
    self.exports.get_mut(&self.export_id(name)?.clone())
  }

  pub fn remove_export(&mut self, id: &Uuid) -> Option<Export> {
    self.exports.remove(id)
  }

  pub fn remove_export_named(&mut self, name: &str) -> Option<Export> {
    self.remove_export(&self.export_id(name)?.clone())
  }

  pub fn add_export(&mut self, id: Uuid, export: Export) {
    self.exports.insert(id, export);
  }
}

#[derive(GraphQLObject)]
pub struct IdExport {
  pub id: Uuid,
  pub export: Export,
}

impl From<(Uuid, Export)> for IdExport {
  fn from(pair: (Uuid, Export)) -> Self {
    Self {
      id: pair.0,
      export: pair.1,
    }
  }
}


#[graphql_object]
impl Module {
  pub fn acl(&self) -> &Acl {
    &self.acl
  }

  #[graphql(name = "parent")]
  pub fn gql_parent(&self) -> Uuid {
    self.parent.clone()
  }

  #[graphql(name = "name")]
  pub fn gql_name(&self) -> String {
    self.name.clone()
  }

  #[graphql(name = "exports")]
  pub fn gql_exports(&self) -> Vec<IdExport> {
    self.exports.iter().map(|(k, v)| (k.clone(), v.clone()).into()).collect()
  }

  #[graphql(name = "executable")]
  pub fn gql_executable(&self) -> Option<Uuid> {
    self.executable.clone()
  }

  #[graphql(name = "exportId")]
  pub fn gql_export_id(&self, name: String) -> Option<Uuid> {
    self.export_id(&name).map(Clone::clone)
  }

  #[graphql(name = "hasExport")]
  pub fn gql_has_export(&self, id: Uuid) -> bool {
    self.has_export(&id)
  }

  #[graphql(name = "hasExportNamed")]
  pub fn gql_has_export_named(&self, name: String) -> bool {
    self.has_export_named(&name)
  }

  #[graphql(name = "export")]
  pub fn gql_export(&self, id: Uuid) -> Option<&Export> {
    self.export(&id)
  }

  #[graphql(name = "exportNamed")]
  pub fn gql_export_named(&self, name: String) -> Option<&Export> {
    self.export_named(&name)
  }

  #[graphql(name = "dependencies")]
  pub fn gql_dependencies(&self) -> &Vec<UnfrozenReference> {
    &self.dependencies
  }
}

impl_unfrozen!(Module, Action);

impl Default for Module {
  fn default() -> Self {
    Self {
      acl: Default::default(),
      parent: Uuid::nil(),
      name: String::new(),
      exports: HashMap::new(),
      executable: None,
      dependencies: Vec::new(),
    }
  }
}

impl Unfrozen<Action> for Module {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    for (_, export) in &self.exports {
      export.dependencies(set);
    }

    for dependency in &self.dependencies {
      set.insert(dependency);
    }
  }
}

with_acl!(Module);

impl View for Module {
  fn name<'a>(&'a self) -> Option<&'a str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a Uuid> {
    Some(&self.parent)
  }

  fn acl<'a>(&'a self) -> Option<&'a Acl> {
    Some(&self.acl)
  }
}

impl BlobDependencies for Module {
  fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    if let Some(executable) = &self.executable {
      set.insert(&executable);
    }
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Module {
  type Frozen = frozen::Module;
  
  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    let mut exports = HashMap::with_capacity(self.exports.len());
    for (id, export) in &self.exports {
      exports.insert(id.clone(), export.freeze(freezer).await?);
    }

    let mut dependencies = Vec::with_capacity(self.dependencies.len());
    for dependency in &self.dependencies {
      dependencies.push(freezer.freeze(dependency).await?);
    }

    Ok(Self::Frozen {
      parent: self.parent,
      name: self.name.clone(),
      exports,
      executable: self.executable.clone(),
      dependencies,
    })
  }
}

name!(Module);
parent!(Module);