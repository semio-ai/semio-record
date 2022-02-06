use std::collections::{HashSet, HashMap};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
  pub components: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Reference {
  pub id: Uuid,
  pub version: semver::Version,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum UnresolvedReferenceKind {
  Path(Path),
  Uuid(Uuid),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnresolvedReference {
  pub kind: UnresolvedReferenceKind,
  pub version: semver::VersionReq,
}

pub trait Entity {
  const NAME: &'static str;

  type Action;

  fn children<'a>(&'a self, _map: &mut HashMap<&'a str, &'a Uuid>) {}
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a Reference>) {}
}

#[async_trait]
pub trait EntityResolver {
  type Error: std::error::Error;

  async fn resolve_entity(
    &mut self,
    unresolved_reference: &UnresolvedReference,
  ) -> Result<Reference, Self::Error>;
}
