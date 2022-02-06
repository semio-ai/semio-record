use serde::{Deserialize, Serialize};

use crate::ty::UnresolvedTy;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnresolvedStructureField {
  pub name: String,
  pub ty: UnresolvedTy,
}