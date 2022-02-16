use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{ty::UnfrozenTy, record::View};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct StructureField {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: UnfrozenTy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Structure {
  pub parent: Uuid,
  pub name: String,
  pub fields: HashMap<Uuid, StructureField>,
}

impl View for Structure {
  fn name<'a>(&'a self) -> Option<&'a str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a Uuid> {
    Some(&self.parent)
  }
}


