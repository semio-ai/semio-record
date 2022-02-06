use std::collections::{HashSet, HashMap};

use uuid::Uuid;

use crate::entity::{Entity, Reference};

pub mod action;

pub struct Folder {
  pub entries: HashMap<String, Uuid>,
  pub links: HashMap<String, Reference>,
}

impl Entity for Folder {
  const NAME: &'static str = "folder";

  type Action = ();

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
    for (_, link) in &self.links {
      set.insert(link);
    }
  }

  fn children<'a>(&'a self, map: &mut HashMap<&'a str, &'a Uuid>) {
    for (name, id) in &self.entries {
      map.insert(name, id);
    }
  }
}