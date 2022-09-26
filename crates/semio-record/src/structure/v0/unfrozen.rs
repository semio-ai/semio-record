use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use async_trait::async_trait;

use crate::{ty::UnfrozenTy, record::{View, Freezer, Freeze, Unfrozen, UnfrozenReference}, action::{name, parent}, acl::{Acl, action::with_acl}, blob::BlobDependencies, unfrozen::impl_unfrozen, migrate::Migrate};

use super::{frozen, action::Action};

use schemars::JsonSchema;

#[derive(Clone, Debug, Serialize, Deserialize, GraphQLObject, JsonSchema)]
pub struct StructureField {
  pub name: String,
  #[serde(rename = "type")]
  #[graphql(name = "type")]
  pub ty: UnfrozenTy,
}

#[async_trait]
impl<F: Freezer> Freeze<F> for StructureField {
  type Frozen = frozen::StructureField;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    Ok(frozen::StructureField {
      name: self.name.clone(),
      ty: self.ty.freeze(freezer).await?,
    })
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Structure {
  pub parent: Uuid,
  pub name: String,
  pub acl: Acl,
  pub fields: HashMap<Uuid, StructureField>,
}

impl Default for Structure {
  fn default() -> Self {
    Self {
      parent: Uuid::default(),
      name: "".to_string(),
      acl: Default::default(),
      fields: HashMap::new(),
    }
  }
}

impl_unfrozen!(Structure, Action);


#[derive(Debug, Serialize, Deserialize, GraphQLObject, JsonSchema)]
pub struct IdStructureField {
  pub id: Uuid,
  pub field: StructureField
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

#[graphql_object]
impl Structure {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn parent(&self) -> &Uuid {
    &self.parent
  }

  pub fn acl(&self) -> &Acl {
    &self.acl
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

  fn acl<'a>(&'a self) -> Option<&'a Acl> {
    Some(&self.acl)
  }
}


name!(Structure);
parent!(Structure);

with_acl!(Structure);

impl Unfrozen<Action> for Structure {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    for field in self.fields.values() {
      field.ty.dependencies(set);
    }
  }
}

impl BlobDependencies for Structure {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a uuid::Uuid>) {}
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Structure {
  type Frozen = frozen::Structure;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    let mut fields = HashMap::new();
    for (id, field) in &self.fields {
      fields.insert(id.clone(), field.freeze(freezer).await?);
    }
    Ok(frozen::Structure {
      parent: self.parent.clone(),
      name: self.name.clone(),
      fields,
    })
  }
}

impl Migrate for Structure {
  fn migrate(from_version: i16, _from: &[u8]) -> anyhow::Result<Self> {
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}