use std::collections::HashSet;
use std::str::FromStr;

use juniper::FromInputValue;
use juniper::InputValue;
use juniper::ScalarValue;
use juniper::marker::IsInputType;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::record::Freeze;
use crate::record::Freezer;
use crate::record::FrozenReference;
use crate::record::UnfrozenReference;
use crate::record::VersionReq;

use juniper::{GraphQLObject, GraphQLEnum, GraphQLUnion};

use derive_more::From;

#[derive(Debug, GraphQLEnum, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PrimitiveKind {
  Unit,
  Boolean,
  U8,
  U16,
  U32,
  U64,
  I8,
  I16,
  I32,
  I64,
  F32,
  F64,
  String,
  ArrayBoolean,
  ArrayU8,
  ArrayU16,
  ArrayU32,
  ArrayU64,
  ArrayI8,
  ArrayI16,
  ArrayI32,
  ArrayI64,
  ArrayF32,
  ArrayF64,
  ArrayString,
}

impl PrimitiveKind {
  pub fn is_array(&self) -> bool {
    match self {
      PrimitiveKind::ArrayBoolean |
      PrimitiveKind::ArrayU8 |
      PrimitiveKind::ArrayU16 |
      PrimitiveKind::ArrayU32 |
      PrimitiveKind::ArrayU64 |
      PrimitiveKind::ArrayI8 |
      PrimitiveKind::ArrayI16 |
      PrimitiveKind::ArrayI32 |
      PrimitiveKind::ArrayI64 |
      PrimitiveKind::ArrayF32 |
      PrimitiveKind::ArrayF64 |
      PrimitiveKind::ArrayString => true,
      _ => false,
    }
  }

  pub fn is_scalar(&self) -> bool {
    match self {
      PrimitiveKind::Unit |
      PrimitiveKind::Boolean |
      PrimitiveKind::U8 |
      PrimitiveKind::U16 |
      PrimitiveKind::U32 |
      PrimitiveKind::U64 |
      PrimitiveKind::I8 |
      PrimitiveKind::I16 |
      PrimitiveKind::I32 |
      PrimitiveKind::I64 |
      PrimitiveKind::F32 |
      PrimitiveKind::F64 |
      PrimitiveKind::String => true,
      _ => false,
    }
  }
}

#[derive(Debug, GraphQLObject, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Primitive {
  pub kind: PrimitiveKind,
}

impl From<PrimitiveKind> for Primitive {
  fn from(kind: PrimitiveKind) -> Self {
    Self { kind }
  }
}

impl Primitive {
  pub const UNIT: Self = Self { kind: PrimitiveKind::Unit };
  pub const BOOLEAN: Self = Self { kind: PrimitiveKind::Boolean };
  pub const U8: Self = Self { kind: PrimitiveKind::U8 };
  pub const U16: Self = Self { kind: PrimitiveKind::U16 };
  pub const U32: Self = Self { kind: PrimitiveKind::U32 };
  pub const U64: Self = Self { kind: PrimitiveKind::U64 };
  pub const I8: Self = Self { kind: PrimitiveKind::I8 };
  pub const I16: Self = Self { kind: PrimitiveKind::I16 };
  pub const I32: Self = Self { kind: PrimitiveKind::I32 };
  pub const I64: Self = Self { kind: PrimitiveKind::I64 };
  pub const F32: Self = Self { kind: PrimitiveKind::F32 };
  pub const F64: Self = Self { kind: PrimitiveKind::F64 };
  pub const STRING: Self = Self { kind: PrimitiveKind::String };
  pub const ARRAY_BOOLEAN: Self = Self { kind: PrimitiveKind::ArrayBoolean };
  pub const ARRAY_U8: Self = Self { kind: PrimitiveKind::ArrayU8 };
  pub const ARRAY_U16: Self = Self { kind: PrimitiveKind::ArrayU16 };
  pub const ARRAY_U32: Self = Self { kind: PrimitiveKind::ArrayU32 };
  pub const ARRAY_U64: Self = Self { kind: PrimitiveKind::ArrayU64 };
  pub const ARRAY_I8: Self = Self { kind: PrimitiveKind::ArrayI8 };
  pub const ARRAY_I16: Self = Self { kind: PrimitiveKind::ArrayI16 };
  pub const ARRAY_I32: Self = Self { kind: PrimitiveKind::ArrayI32 };
  pub const ARRAY_I64: Self = Self { kind: PrimitiveKind::ArrayI64 };
  pub const ARRAY_F32: Self = Self { kind: PrimitiveKind::ArrayF32 };
  pub const ARRAY_F64: Self = Self { kind: PrimitiveKind::ArrayF64 };
  pub const ARRAY_STRING: Self = Self { kind: PrimitiveKind::ArrayString };

  pub fn is_array(&self) -> bool {
    self.kind.is_array()
  }

  pub fn is_scalar(&self) -> bool {
    self.kind.is_scalar()
  }
}

#[derive(Debug, GraphQLObject, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FrozenArray {
  pub reference: FrozenReference
}

#[derive(Debug, GraphQLObject, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FrozenScalar {
  pub reference: FrozenReference
}

#[derive(Debug, Serialize, Deserialize, GraphQLUnion, From, Clone, PartialEq, Eq)]
pub enum FrozenTy {
  Primitive(Primitive),
  Scalar(FrozenScalar),
  Array(FrozenArray),
}

impl From<PrimitiveKind> for FrozenTy {
  fn from(kind: PrimitiveKind) -> Self {
    Self::Primitive(Primitive::from(kind))
  }
}

impl FrozenTy {
  pub fn as_primitive(&self) -> Option<&Primitive> {
    match self {
      Self::Primitive(primitive) => Some(primitive),
      _ => None,
    }
  }

  pub fn as_scalar(&self) -> Option<&FrozenScalar> {
    match self {
      Self::Scalar(scalar) => Some(scalar),
      _ => None,
    }
  }

  pub fn as_array(&self) -> Option<&FrozenArray> {
    match self {
      Self::Array(array) => Some(array),
      _ => None,
    }
  }

  pub fn is_primitive(&self) -> bool {
    match self {
      Self::Primitive(_) => true,
      _ => false,
    }
  }

  pub fn is_scalar(&self) -> bool {
    match self {
      Self::Scalar(_) => true,
      _ => false,
    }
  }

  pub fn is_array(&self) -> bool {
    match self {
      Self::Array(_) => true,
      _ => false,
    }
  }

  pub fn to_primitive(self) -> Option<Primitive> {
    match self {
      Self::Primitive(primitive) => Some(primitive),
      _ => None,
    }
  }

  pub fn to_scalar(self) -> Option<FrozenScalar> {
    match self {
      Self::Scalar(scalar) => Some(scalar),
      _ => None,
    }
  }

  pub fn to_array(self) -> Option<FrozenArray> {
    match self {
      Self::Array(array) => Some(array),
      _ => None,
    }
  }

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    match self {
      Self::Scalar(ty) => {
        set.insert(&ty.reference);
      },
      Self::Array(ty) => {
        set.insert(&ty.reference);
      },
      _ => {}
    }
  }
}



#[derive(Debug, GraphQLObject, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UnfrozenScalar {
  pub reference: UnfrozenReference,
}

#[async_trait]
impl<F: Freezer> Freeze<F> for UnfrozenScalar {
  type Frozen = FrozenScalar;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    Ok(FrozenScalar { reference: freezer.freeze(&self.reference).await? })
  }
}

#[derive(Debug, GraphQLObject, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UnfrozenArray {
  pub reference: UnfrozenReference,
}

#[async_trait]
impl<F: Freezer> Freeze<F> for UnfrozenArray {
  type Frozen = FrozenArray;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    Ok(FrozenArray { reference: freezer.freeze(&self.reference).await? })
  }
}


#[derive(Debug, Serialize, Deserialize, GraphQLUnion, From, Clone, PartialEq, Eq)]
pub enum UnfrozenTy {
  Primitive(Primitive),
  Scalar(UnfrozenScalar),
  Array(UnfrozenArray),
}

impl From<PrimitiveKind> for UnfrozenTy {
  fn from(kind: PrimitiveKind) -> Self {
    Self::Primitive(Primitive::from(kind))
  }
}

impl UnfrozenTy {
  pub fn as_primitive(&self) -> Option<&Primitive> {
    match self {
      Self::Primitive(primitive) => Some(primitive),
      _ => None,
    }
  }

  pub fn as_scalar(&self) -> Option<&UnfrozenScalar> {
    match self {
      Self::Scalar(scalar) => Some(scalar),
      _ => None,
    }
  }

  pub fn as_array(&self) -> Option<&UnfrozenArray> {
    match self {
      Self::Array(array) => Some(array),
      _ => None,
    }
  }

  pub fn is_primitive(&self) -> bool {
    match self {
      Self::Primitive(_) => true,
      _ => false,
    }
  }

  pub fn is_scalar(&self) -> bool {
    match self {
      Self::Scalar(_) => true,
      _ => false,
    }
  }

  pub fn is_array(&self) -> bool {
    match self {
      Self::Array(_) => true,
      _ => false,
    }
  }

  pub fn to_primitive(self) -> Option<Primitive> {
    match self {
      Self::Primitive(primitive) => Some(primitive),
      _ => None,
    }
  }

  pub fn to_scalar(self) -> Option<UnfrozenScalar> {
    match self {
      Self::Scalar(scalar) => Some(scalar),
      _ => None,
    }
  }

  pub fn to_array(self) -> Option<UnfrozenArray> {
    match self {
      Self::Array(array) => Some(array),
      _ => None,
    }
  }

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    match self {
      Self::Scalar(ty) => {
        set.insert(&ty.reference);
      },
      Self::Array(ty) => {
        set.insert(&ty.reference);
      },
      _ => {}
    }
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for UnfrozenTy {
  type Frozen = FrozenTy;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    match self {
      Self::Primitive(primitive) => Ok(FrozenTy::Primitive(primitive.clone())),
      Self::Scalar(scalar) => Ok(FrozenTy::Scalar(scalar.freeze(freezer).await?)),
      Self::Array(array) => Ok(FrozenTy::Array(array.freeze(freezer).await?)),
    }
  }
}

impl<S: ScalarValue> FromInputValue<S> for UnfrozenTy {
  fn from_input_value(value: &InputValue<S>) -> Option<Self> {
    let str = value.as_string_value()?;
    Self::from_str(&str).ok()
  }
}

impl<S: ScalarValue> IsInputType<S> for UnfrozenTy {}

impl FromStr for UnfrozenTy {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let array = s.ends_with("[]");
    let s = if array { &s[..s.len() - 2] } else { s };
    let mut iter = s.split('@');
    let s = iter.next().ok_or(())?;
    
    match s {
      "unit" => Ok(PrimitiveKind::Unit.into()),
      "bool" => Ok(if !array { PrimitiveKind::Boolean } else { PrimitiveKind::ArrayBoolean }.into()),
      "u8" => Ok(if !array { PrimitiveKind::U8 } else { PrimitiveKind::ArrayU8 }.into()),
      "u16" => Ok(if !array { PrimitiveKind::U16 } else { PrimitiveKind::ArrayU16 }.into()),
      "u32" => Ok(if !array { PrimitiveKind::U32 } else { PrimitiveKind::ArrayU32 }.into()),
      "u64" => Ok(if !array { PrimitiveKind::U64 } else { PrimitiveKind::ArrayU64 }.into()),
      "i8" => Ok(if !array { PrimitiveKind::I8 } else { PrimitiveKind::ArrayI8 }.into()),
      "i16" => Ok(if !array { PrimitiveKind::I16 } else { PrimitiveKind::ArrayI16 }.into()),
      "i32" => Ok(if !array { PrimitiveKind::I32 } else { PrimitiveKind::ArrayI32 }.into()),
      "i64" => Ok(if !array { PrimitiveKind::I64 } else { PrimitiveKind::ArrayI64 }.into()),
      "f32" => Ok(if !array { PrimitiveKind::F32 } else { PrimitiveKind::ArrayF32 }.into()),
      "f64" => Ok(if !array { PrimitiveKind::F64 } else { PrimitiveKind::ArrayF64 }.into()),
      "str" => Ok(if !array { PrimitiveKind::String } else { PrimitiveKind::ArrayString }.into()),
      _ => {
        let id = Uuid::parse_str(s).map_err(|_| ())?;
        let version_req = VersionReq(iter.next().map(|v| semver::VersionReq::parse(v).ok()).flatten());
        let reference = UnfrozenReference { id, version_req };
        Ok(if array {
          UnfrozenTy::Array(UnfrozenArray { reference })
        } else {
          UnfrozenTy::Scalar(UnfrozenScalar { reference })
        })
      }
    }
  }
}

#[cfg(test)]
mod test {
  use std::str::FromStr;

use crate::ty::UnfrozenArray;

  #[test]
  fn parse_u8_array() {
    assert_eq!(
      super::UnfrozenTy::from_str("u8[]").unwrap(),
      super::UnfrozenTy::Primitive(super::PrimitiveKind::ArrayU8.into())
    );
  }

  #[test]
  fn parse_ty_version_array() {
    assert_eq!(
      super::UnfrozenTy::from_str("006867d6-7898-4d11-8a8a-471135f66aed@>1.0.0[]").unwrap(),
      super::UnfrozenTy::Array(UnfrozenArray {
        reference: super::UnfrozenReference {
          id: super::Uuid::from_str("006867d6-7898-4d11-8a8a-471135f66aed").unwrap(),
          version_req: super::VersionReq(Some(semver::VersionReq::parse(">1.0.0").unwrap())),
        },
      })
    );
  }
}
