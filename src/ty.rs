use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

use crate::entity::Reference;
use crate::id::Id;

#[derive(Debug, Serialize, Deserialize)]
pub enum Ty {
  Unit,
  Boolean,
  U8,
  U16,
  U32,
  U64,
  S8,
  S16,
  S32,
  S64,
  R32,
  R64,
  String,
  ArrayBoolean,
  ArrayU8,
  ArrayU16,
  ArrayU32,
  ArrayU64,
  ArrayS8,
  ArrayS16,
  ArrayS32,
  ArrayS64,
  ArrayR32,
  ArrayR64,
  ArrayString,
  Scalar(Reference),
  Array(Reference),
}

impl Ty {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
    match self {
      Ty::Scalar(ty) => {
        set.insert(ty);
      },
      Ty::Array(ty) => {
        set.insert(ty);
      },
      _ => {}
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnresolvedTy {
  Unit,
  Boolean,
  U8,
  U16,
  U32,
  U64,
  S8,
  S16,
  S32,
  S64,
  R32,
  R64,
  String,
  ArrayBoolean,
  ArrayU8,
  ArrayU16,
  ArrayU32,
  ArrayU64,
  ArrayS8,
  ArrayS16,
  ArrayS32,
  ArrayS64,
  ArrayR32,
  ArrayR64,
  ArrayString,
  Scalar(Id),
  Array(Id),
}
