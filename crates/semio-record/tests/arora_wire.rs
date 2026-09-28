//! semio-record and arora-types share one wire format. A remote Arora registry
//! reads the Semio store by serializing semio-record's records and
//! deserializing them as arora-types' (`arora-registry-remote`), so every
//! record kind it converts must survive that round trip unchanged.

use std::collections::HashMap;

use indexmap::IndexMap;
use semio_record::{
  enumeration::v0::frozen as enumeration,
  folder::v0::public as folder,
  module::v0::{frozen as module, unfrozen as unfrozen_module},
  record::{FrozenReference, UnfrozenReference, Version, VersionReq},
  structure::v0::frozen as structure,
  ty::{
    FrozenOption, FrozenScalar, FrozenTy, PrimitiveKind, UnfrozenArray, UnfrozenOption, UnfrozenTy,
  },
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Map, Value};
use uuid::Uuid;

/// `value`, serialized, reads as arora-types' `A` and writes back unchanged.
fn assert_same_wire<A: Serialize + DeserializeOwned>(value: &impl Serialize) {
  let wire = serde_json::to_value(value).unwrap();
  let arora: A = serde_json::from_value(wire.clone())
    .unwrap_or_else(|e| panic!("arora-types cannot read {wire}: {e}"));
  assert_eq!(serde_json::to_value(arora).unwrap(), wire);
}

fn reference() -> FrozenReference {
  FrozenReference {
    id: Uuid::new_v4(),
    version: Version(semver::Version::new(1, 2, 0)),
  }
}

/// An optional over a reference, the deepest form a type takes.
fn optional_reference() -> FrozenTy {
  FrozenTy::FrozenOption(FrozenOption {
    element: Box::new(FrozenTy::FrozenScalar(FrozenScalar {
      reference: reference(),
    })),
  })
}

fn frozen_function() -> module::Function {
  let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
  module::Function {
    parameters: HashMap::from([
      (
        a,
        module::Parameter {
          name: "target".into(),
          ty: optional_reference(),
          mutable: false,
        },
      ),
      (
        b,
        module::Parameter {
          name: "speed".into(),
          ty: PrimitiveKind::F32.into(),
          mutable: true,
        },
      ),
    ]),
    parameter_ordering: vec![a, b],
    return_ty: PrimitiveKind::ArrayString.into(),
  }
}

#[test]
fn a_frozen_module() {
  let module = module::Module {
    parent: Uuid::new_v4(),
    name: "Motion".into(),
    exports: HashMap::from([(
      Uuid::new_v4(),
      module::Export {
        name: "seek".into(),
        kind: module::ExportKind::Function(frozen_function()),
      },
    )]),
    executable: Some(Uuid::new_v4()),
    dependencies: vec![reference()],
  };
  assert_same_wire::<arora_types::record::module::frozen::Module>(&module);
}

#[test]
fn an_unfrozen_function() {
  let id = Uuid::new_v4();
  let function = unfrozen_module::Function {
    parameters: HashMap::from([(
      id,
      unfrozen_module::Parameter {
        name: "names".into(),
        ty: UnfrozenTy::UnfrozenOption(UnfrozenOption {
          element: Box::new(UnfrozenTy::UnfrozenArray(UnfrozenArray {
            reference: UnfrozenReference {
              id: Uuid::new_v4(),
              version_req: VersionReq(Some(semver::VersionReq::parse("^1").unwrap())),
            },
          })),
        }),
        mutable: false,
      },
    )]),
    parameter_ordering: vec![id],
    return_ty: PrimitiveKind::Unit.into(),
  };
  assert_same_wire::<arora_types::record::module::unfrozen::Function>(&function);
}

#[test]
fn a_frozen_structure() {
  let structure = structure::Structure {
    parent: Uuid::new_v4(),
    name: "Widget".into(),
    fields: IndexMap::from([
      (
        Uuid::new_v4(),
        structure::StructureField {
          name: "size".into(),
          ty: PrimitiveKind::U32.into(),
        },
      ),
      (
        Uuid::new_v4(),
        structure::StructureField {
          name: "part".into(),
          ty: optional_reference(),
        },
      ),
    ]),
  };
  assert_same_wire::<arora_types::record::structure::frozen::Structure>(&structure);
}

#[test]
fn a_frozen_enumeration() {
  let enumeration = enumeration::Enumeration {
    parent: Uuid::new_v4(),
    name: "Color".into(),
    variants: IndexMap::from([
      (
        Uuid::new_v4(),
        enumeration::EnumerationVariant {
          name: "none".into(),
          ty: PrimitiveKind::Unit.into(),
        },
      ),
      (
        Uuid::new_v4(),
        enumeration::EnumerationVariant {
          name: "custom".into(),
          ty: optional_reference(),
        },
      ),
    ]),
  };
  assert_same_wire::<arora_types::record::enumeration::frozen::Enumeration>(&enumeration);
}

#[test]
fn a_public_folder() {
  let folder = folder::Public {
    name: "robots".into(),
    parent: Uuid::new_v4(),
  };
  assert_same_wire::<arora_types::record::folder::public::Public>(&folder);
}

/// Frozen functions stored before the return type was spelled `returnType`
/// hold it under `returnTy`.
#[test]
fn a_stored_frozen_function_reads_under_either_spelling() {
  for key in ["returnType", "returnTy"] {
    let mut wire = Map::new();
    wire.insert("parameters".into(), json!({}));
    wire.insert("parameterOrdering".into(), json!([]));
    wire.insert(
      key.into(),
      json!({ "type": "primitive", "value": { "kind": "u8" } }),
    );
    let function: module::Function =
      serde_json::from_value(Value::Object(wire)).unwrap_or_else(|e| panic!("{key}: {e}"));
    assert!(
      matches!(function.return_ty, FrozenTy::Primitive(ref p) if p.kind == PrimitiveKind::U8),
      "{key}"
    );
  }
}
