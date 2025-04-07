pub mod v0;

pub use v0 as latest;

use crate::record::impl_record;

impl_record!(
  0 => v0
);

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use uuid::Uuid;

  use crate::{
    acl::Acl,
    ty::{Primitive, PrimitiveKind, UnfrozenTy},
  };

  use super::v0::unfrozen::{Enumeration, EnumerationVariant};

  #[test]
  fn test_serde_yaml() {
    let enumeration_meta = Enumeration {
      name: "Status".to_string(),
      parent: Uuid::new_v4(),
      variants: HashMap::from([
        (
          Uuid::new_v4(),
          EnumerationVariant {
            name: "Success".to_string(),
            ty: UnfrozenTy::Primitive(Primitive {
              kind: PrimitiveKind::Unit,
            }),
          },
        ),
        (
          Uuid::new_v4(),
          EnumerationVariant {
            name: "Failure".to_string(),
            ty: UnfrozenTy::Primitive(Primitive {
              kind: PrimitiveKind::Unit,
            }),
          },
        ),
        (
          Uuid::new_v4(),
          EnumerationVariant {
            name: "Running".to_string(),
            ty: UnfrozenTy::Primitive(Primitive {
              kind: PrimitiveKind::Unit,
            }),
          },
        ),
      ]),
      acl: <Acl as Default>::default(),
    };

    let yaml = serde_yaml::to_string(&enumeration_meta).unwrap();
    println!("YAML: {}", yaml);
    let deserialized: Enumeration = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(enumeration_meta, deserialized);
  }
}
