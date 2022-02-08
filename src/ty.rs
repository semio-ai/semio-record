use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

use crate::record::FrozenReference;
use crate::record::Reference;
use crate::record::UnfrozenReference;

#[derive(Debug, Serialize, Deserialize)]
pub enum FrozenTy {
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
  Scalar(FrozenReference),
  Array(FrozenReference),
}

impl FrozenTy {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    match self {
      Self::Scalar(ty) => {
        set.insert(ty);
      },
      Self::Array(ty) => {
        set.insert(ty);
      },
      _ => {}
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnfrozenTy {
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
  Scalar(UnfrozenReference),
  Array(UnfrozenReference),
}

impl UnfrozenTy {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    match self {
      Self::Scalar(ty) => {
        set.insert(ty);
      },
      Self::Array(ty) => {
        set.insert(ty);
      },
      _ => {}
    }
  }
}