use std::collections::HashSet;
use std::str::FromStr;

use async_trait::async_trait;
use derive_more::From;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::record::Freeze;
use crate::record::Freezer;
use crate::record::FrozenReference;
use crate::record::UnfrozenReference;
use crate::record::VersionReq;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Primitive_Kind", rename_all = "camelCase")]
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
      PrimitiveKind::ArrayBoolean
      | PrimitiveKind::ArrayU8
      | PrimitiveKind::ArrayU16
      | PrimitiveKind::ArrayU32
      | PrimitiveKind::ArrayU64
      | PrimitiveKind::ArrayI8
      | PrimitiveKind::ArrayI16
      | PrimitiveKind::ArrayI32
      | PrimitiveKind::ArrayI64
      | PrimitiveKind::ArrayF32
      | PrimitiveKind::ArrayF64
      | PrimitiveKind::ArrayString => true,
      _ => false,
    }
  }

  pub fn is_scalar(&self) -> bool {
    match self {
      PrimitiveKind::Unit
      | PrimitiveKind::Boolean
      | PrimitiveKind::U8
      | PrimitiveKind::U16
      | PrimitiveKind::U32
      | PrimitiveKind::U64
      | PrimitiveKind::I8
      | PrimitiveKind::I16
      | PrimitiveKind::I32
      | PrimitiveKind::I64
      | PrimitiveKind::F32
      | PrimitiveKind::F64
      | PrimitiveKind::String => true,
      _ => false,
    }
  }
}

impl ToString for PrimitiveKind {
  fn to_string(&self) -> String {
    match self {
      PrimitiveKind::Unit => "unit".to_string(),
      PrimitiveKind::Boolean => "bool".to_string(),
      PrimitiveKind::U8 => "u8".to_string(),
      PrimitiveKind::U16 => "u16".to_string(),
      PrimitiveKind::U32 => "u32".to_string(),
      PrimitiveKind::U64 => "u64".to_string(),
      PrimitiveKind::I8 => "i8".to_string(),
      PrimitiveKind::I16 => "i16".to_string(),
      PrimitiveKind::I32 => "i32".to_string(),
      PrimitiveKind::I64 => "i64".to_string(),
      PrimitiveKind::F32 => "f32".to_string(),
      PrimitiveKind::F64 => "f64".to_string(),
      PrimitiveKind::String => "str".to_string(),
      PrimitiveKind::ArrayBoolean => "bool[]".to_string(),
      PrimitiveKind::ArrayU8 => "u8[]".to_string(),
      PrimitiveKind::ArrayU16 => "u16[]".to_string(),
      PrimitiveKind::ArrayU32 => "u32[]".to_string(),
      PrimitiveKind::ArrayU64 => "u64[]".to_string(),
      PrimitiveKind::ArrayI8 => "i8[]".to_string(),
      PrimitiveKind::ArrayI16 => "i16[]".to_string(),
      PrimitiveKind::ArrayI32 => "i32[]".to_string(),
      PrimitiveKind::ArrayI64 => "i64[]".to_string(),
      PrimitiveKind::ArrayF32 => "f32[]".to_string(),
      PrimitiveKind::ArrayF64 => "f64[]".to_string(),
      PrimitiveKind::ArrayString => "str[]".to_string(),
    }
  }
}

impl FromStr for PrimitiveKind {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "unit" => Ok(PrimitiveKind::Unit),
      "bool" => Ok(PrimitiveKind::Boolean),
      "u8" => Ok(PrimitiveKind::U8),
      "u16" => Ok(PrimitiveKind::U16),
      "u32" => Ok(PrimitiveKind::U32),
      "u64" => Ok(PrimitiveKind::U64),
      "i8" => Ok(PrimitiveKind::I8),
      "i16" => Ok(PrimitiveKind::I16),
      "i32" => Ok(PrimitiveKind::I32),
      "i64" => Ok(PrimitiveKind::I64),
      "f32" => Ok(PrimitiveKind::F32),
      "f64" => Ok(PrimitiveKind::F64),
      "str" => Ok(PrimitiveKind::String),
      "bool[]" => Ok(PrimitiveKind::ArrayBoolean),
      "u8[]" => Ok(PrimitiveKind::ArrayU8),
      "u16[]" => Ok(PrimitiveKind::ArrayU16),
      "u32[]" => Ok(PrimitiveKind::ArrayU32),
      "u64[]" => Ok(PrimitiveKind::ArrayU64),
      "i8[]" => Ok(PrimitiveKind::ArrayI8),
      "i16[]" => Ok(PrimitiveKind::ArrayI16),
      "i32[]" => Ok(PrimitiveKind::ArrayI32),
      "i64[]" => Ok(PrimitiveKind::ArrayI64),
      "f32[]" => Ok(PrimitiveKind::ArrayF32),
      "f64[]" => Ok(PrimitiveKind::ArrayF64),
      "str[]" => Ok(PrimitiveKind::ArrayString),
      _ => Err(format!("unknown primitive kind: {}", s)),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Primitive {
  pub kind: PrimitiveKind,
}

impl ToString for Primitive {
  fn to_string(&self) -> String {
    self.kind.to_string()
  }
}

impl From<PrimitiveKind> for Primitive {
  fn from(kind: PrimitiveKind) -> Self {
    Self { kind }
  }
}

impl Primitive {
  pub const UNIT: Self = Self {
    kind: PrimitiveKind::Unit,
  };
  pub const BOOLEAN: Self = Self {
    kind: PrimitiveKind::Boolean,
  };
  pub const U8: Self = Self {
    kind: PrimitiveKind::U8,
  };
  pub const U16: Self = Self {
    kind: PrimitiveKind::U16,
  };
  pub const U32: Self = Self {
    kind: PrimitiveKind::U32,
  };
  pub const U64: Self = Self {
    kind: PrimitiveKind::U64,
  };
  pub const I8: Self = Self {
    kind: PrimitiveKind::I8,
  };
  pub const I16: Self = Self {
    kind: PrimitiveKind::I16,
  };
  pub const I32: Self = Self {
    kind: PrimitiveKind::I32,
  };
  pub const I64: Self = Self {
    kind: PrimitiveKind::I64,
  };
  pub const F32: Self = Self {
    kind: PrimitiveKind::F32,
  };
  pub const F64: Self = Self {
    kind: PrimitiveKind::F64,
  };
  pub const STRING: Self = Self {
    kind: PrimitiveKind::String,
  };
  pub const ARRAY_BOOLEAN: Self = Self {
    kind: PrimitiveKind::ArrayBoolean,
  };
  pub const ARRAY_U8: Self = Self {
    kind: PrimitiveKind::ArrayU8,
  };
  pub const ARRAY_U16: Self = Self {
    kind: PrimitiveKind::ArrayU16,
  };
  pub const ARRAY_U32: Self = Self {
    kind: PrimitiveKind::ArrayU32,
  };
  pub const ARRAY_U64: Self = Self {
    kind: PrimitiveKind::ArrayU64,
  };
  pub const ARRAY_I8: Self = Self {
    kind: PrimitiveKind::ArrayI8,
  };
  pub const ARRAY_I16: Self = Self {
    kind: PrimitiveKind::ArrayI16,
  };
  pub const ARRAY_I32: Self = Self {
    kind: PrimitiveKind::ArrayI32,
  };
  pub const ARRAY_I64: Self = Self {
    kind: PrimitiveKind::ArrayI64,
  };
  pub const ARRAY_F32: Self = Self {
    kind: PrimitiveKind::ArrayF32,
  };
  pub const ARRAY_F64: Self = Self {
    kind: PrimitiveKind::ArrayF64,
  };
  pub const ARRAY_STRING: Self = Self {
    kind: PrimitiveKind::ArrayString,
  };

  pub fn is_array(&self) -> bool {
    self.kind.is_array()
  }

  pub fn is_scalar(&self) -> bool {
    self.kind.is_scalar()
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "frozen_Array")]
pub struct FrozenArray {
  pub reference: FrozenReference,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "frozen_Scalar")]
pub struct FrozenScalar {
  pub reference: FrozenReference,
}

#[derive(Debug, Serialize, Deserialize, From, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "frozen_Type", rename_all = "camelCase", tag = "type", content = "value")]
pub enum FrozenTy {
  Primitive(Primitive),
  FrozenScalar(FrozenScalar),
  FrozenArray(FrozenArray),
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
      Self::FrozenScalar(scalar) => Some(scalar),
      _ => None,
    }
  }

  pub fn as_array(&self) -> Option<&FrozenArray> {
    match self {
      Self::FrozenArray(array) => Some(array),
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
      Self::FrozenScalar(_) => true,
      _ => false,
    }
  }

  pub fn is_array(&self) -> bool {
    match self {
      Self::FrozenArray(_) => true,
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
      Self::FrozenScalar(scalar) => Some(scalar),
      _ => None,
    }
  }

  pub fn to_array(self) -> Option<FrozenArray> {
    match self {
      Self::FrozenArray(array) => Some(array),
      _ => None,
    }
  }

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    match self {
      Self::FrozenScalar(ty) => {
        set.insert(&ty.reference);
      }
      Self::FrozenArray(ty) => {
        set.insert(&ty.reference);
      }
      _ => {}
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unfrozen_Scalar")]
pub struct UnfrozenScalar {
  pub reference: UnfrozenReference,
}

impl ToString for UnfrozenScalar {
  fn to_string(&self) -> String {
    self.reference.to_string()
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for UnfrozenScalar {
  type Frozen = FrozenScalar;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    Ok(FrozenScalar {
      reference: freezer.freeze(&self.reference).await?,
    })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unfrozen_Array")]
pub struct UnfrozenArray {
  pub reference: UnfrozenReference,
}

impl ToString for UnfrozenArray {
  fn to_string(&self) -> String {
    format!("{}[]", self.reference.to_string())
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for UnfrozenArray {
  type Frozen = FrozenArray;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    Ok(FrozenArray {
      reference: freezer.freeze(&self.reference).await?,
    })
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Unfrozen_Type", rename_all = "camelCase", tag = "type", content = "value")]
pub enum UnfrozenTy {
  Primitive(Primitive),
  UnfrozenScalar(UnfrozenScalar),
  UnfrozenArray(UnfrozenArray),
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
      Self::UnfrozenScalar(scalar) => Some(scalar),
      _ => None,
    }
  }

  pub fn as_array(&self) -> Option<&UnfrozenArray> {
    match self {
      Self::UnfrozenArray(array) => Some(array),
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
      Self::UnfrozenScalar(_) => true,
      _ => false,
    }
  }

  pub fn is_array(&self) -> bool {
    match self {
      Self::UnfrozenArray(_) => true,
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
      Self::UnfrozenScalar(scalar) => Some(scalar),
      _ => None,
    }
  }

  pub fn to_array(self) -> Option<UnfrozenArray> {
    match self {
      Self::UnfrozenArray(array) => Some(array),
      _ => None,
    }
  }

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    match self {
      Self::UnfrozenScalar(ty) => {
        set.insert(&ty.reference);
      }
      Self::UnfrozenArray(ty) => {
        set.insert(&ty.reference);
      }
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
      Self::UnfrozenScalar(scalar) => Ok(FrozenTy::FrozenScalar(scalar.freeze(freezer).await?)),
      Self::UnfrozenArray(array) => Ok(FrozenTy::FrozenArray(array.freeze(freezer).await?)),
    }
  }
}

impl FromStr for UnfrozenTy {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let array = s.ends_with("[]");
    let s = if array { &s[..s.len() - 2] } else { s };
    let mut iter = s.split('@');
    let s = iter.next().ok_or(())?;

    match s {
      "unit" => Ok(PrimitiveKind::Unit.into()),
      "bool" => Ok(
        if !array {
          PrimitiveKind::Boolean
        } else {
          PrimitiveKind::ArrayBoolean
        }
        .into(),
      ),
      "u8" => Ok(
        if !array {
          PrimitiveKind::U8
        } else {
          PrimitiveKind::ArrayU8
        }
        .into(),
      ),
      "u16" => Ok(
        if !array {
          PrimitiveKind::U16
        } else {
          PrimitiveKind::ArrayU16
        }
        .into(),
      ),
      "u32" => Ok(
        if !array {
          PrimitiveKind::U32
        } else {
          PrimitiveKind::ArrayU32
        }
        .into(),
      ),
      "u64" => Ok(
        if !array {
          PrimitiveKind::U64
        } else {
          PrimitiveKind::ArrayU64
        }
        .into(),
      ),
      "i8" => Ok(
        if !array {
          PrimitiveKind::I8
        } else {
          PrimitiveKind::ArrayI8
        }
        .into(),
      ),
      "i16" => Ok(
        if !array {
          PrimitiveKind::I16
        } else {
          PrimitiveKind::ArrayI16
        }
        .into(),
      ),
      "i32" => Ok(
        if !array {
          PrimitiveKind::I32
        } else {
          PrimitiveKind::ArrayI32
        }
        .into(),
      ),
      "i64" => Ok(
        if !array {
          PrimitiveKind::I64
        } else {
          PrimitiveKind::ArrayI64
        }
        .into(),
      ),
      "f32" => Ok(
        if !array {
          PrimitiveKind::F32
        } else {
          PrimitiveKind::ArrayF32
        }
        .into(),
      ),
      "f64" => Ok(
        if !array {
          PrimitiveKind::F64
        } else {
          PrimitiveKind::ArrayF64
        }
        .into(),
      ),
      "str" => Ok(
        if !array {
          PrimitiveKind::String
        } else {
          PrimitiveKind::ArrayString
        }
        .into(),
      ),
      _ => {
        let id = Uuid::parse_str(s).map_err(|_| ())?;
        let version_req = VersionReq(
          iter
            .next()
            .map(|v| semver::VersionReq::parse(v).ok())
            .flatten(),
        );
        let reference = UnfrozenReference { id, version_req };
        Ok(if array {
          UnfrozenTy::UnfrozenArray(UnfrozenArray { reference })
        } else {
          UnfrozenTy::UnfrozenScalar(UnfrozenScalar { reference })
        })
      }
    }
  }
}

impl ToString for UnfrozenTy {
  fn to_string(&self) -> String {
    match self {
      Self::Primitive(primitive) => primitive.to_string(),
      Self::UnfrozenScalar(scalar) => scalar.to_string(),
      Self::UnfrozenArray(array) => array.to_string(),
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
      super::UnfrozenTy::UnfrozenArray(UnfrozenArray {
        reference: super::UnfrozenReference {
          id: super::Uuid::from_str("006867d6-7898-4d11-8a8a-471135f66aed").unwrap(),
          version_req: super::VersionReq(Some(semver::VersionReq::parse(">1.0.0").unwrap())),
        },
      })
    );
  }
}
