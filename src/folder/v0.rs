// use std::collections::{HashSet, HashMap};

// use uuid::Uuid;

// use crate::record::{Record, Reference, TYPE_FOLDER};

// pub mod action;

// pub struct Folder {
//   pub parent: Uuid,
//   pub links: HashMap<String, Reference>,
// }

// impl Record for Folder {
//   const NAME: &'static str = "folder";
//   const TYPE: i16 = TYPE_FOLDER;
//   const SCHEMA_VERSION: i16 = 0;


//   type Action = ();

//   fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
//     for (_, link) in &self.links {
//       set.insert(link);
//     }
//   }

//   fn parent<'a>(&'a self) -> Option<&'a Uuid> {
//     Some(&self.parent)
//   }
// }