use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{ty::FrozenTy, record::{View, Frozen, FrozenReference}, blob::BlobDependencies, acl::Acl};

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "FrozenStructureField")]
pub struct StructureField {
  pub name: String,
  #[serde(rename = "type")]
  #[graphql(name = "type")]
  pub ty: FrozenTy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
  pub parent: Uuid,
  pub name: String,
  pub fields: HashMap<Uuid, StructureField>,
}

impl Structure {
  pub fn field_named(&self, name: &str) -> Option<&StructureField> {
    for (_, field) in &self.fields {
      if field.name == name {
        return Some(field);
      }
    }
    None
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "FrozenIdStructureField")]
pub struct IdStructureField {
  pub id: Uuid,
  pub field: StructureField,
}

#[graphql_object(name = "FrozenStructure")]
impl Structure {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn parent(&self) -> &Uuid {
    &self.parent
  }

  pub fn fields(&self) -> Vec<IdStructureField> {
    self.fields.iter().map(|(id, field)| IdStructureField {
      id: id.clone(),
      field: field.clone(),
    }).collect()
  }

  #[graphql(name = "fieldNamed")]
  pub fn gql_field_named(&self, name: String) -> Option<&StructureField> {
    self.field_named(&name)
  }
}

impl View for Structure {
  fn name<'a>(&'a self) -> Option<&'a str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a Uuid> {
    Some(&self.parent)
  }
}

impl Frozen for Structure {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    for (_, field) in &self.fields {
      field.ty.dependencies(set);
    }
  }
}

impl BlobDependencies for Structure {
  fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    
  }
}