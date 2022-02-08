// use std::collections::{HashMap, HashSet};

// use serde::{Serialize, Deserialize};
// use uuid::Uuid;

// use crate::{ty::Ty, record::{Reference, Record, TYPE_ENUMERATION}};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct EnumerationVariant {
//   pub name: String,
//   #[serde(rename = "type")]
//   pub ty: Ty,
// }

// impl EnumerationVariant {
//   fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
//     self.ty.dependencies(set)
//   }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Enumeration {
//   pub variants: HashMap<Uuid, EnumerationVariant>,
// }

// impl Record for Enumeration {
//   const NAME: &'static str = "enumeration";
//   const TYPE: i16 = TYPE_ENUMERATION;
//   const SCHEMA_VERSION: i16 = 0;

//   type Action = ();

//   fn parent<'a>(&'a self) -> Option<&'a Uuid> {
//     None
//   }

//   fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
//     for (_, variant) in &self.variants {
//       variant.dependencies(set);
//     }
//   }
// }