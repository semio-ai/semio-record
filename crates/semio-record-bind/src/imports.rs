use std::collections::{BTreeMap, btree_map};

use camino::{Utf8Path, Utf8PathBuf, Utf8Component};

use bimap::BiMap;

use crate::ast::{ToTypescript, Decl, ImportDecl};

pub struct Imports {
  name: String,
  path: Utf8PathBuf,
  imports: BiMap<String, Utf8PathBuf>,
}

impl Imports {
  pub fn new<S: AsRef<str>, P: AsRef<Utf8Path>>(name: S, path: P) -> Self {
    Self {
      name: name.as_ref().to_string(),
      path: path.as_ref().to_path_buf(),
      imports: BiMap::new(),
    }
  }


  pub fn add<S: AsRef<str>, P: AsRef<Utf8Path>>(&mut self, name: S, path: P) -> String {
    let name = name.as_ref();
    let path = path.as_ref();

    // Is the path already imported under the same or different name?
    if let Some(name) = self.imports.get_by_right(path) {
      return name.to_string();
    }

    if self.imports.contains_left(name) || name == self.name {
      let mut i = 1;
      loop {
        let new_name = path
          .components()
          .collect::<Vec<_>>()
          .iter()
          .rev()
          .take(i)
          .rev()
          .map(Utf8Component::as_str)
          .collect::<Vec<_>>()
          .join("");
        if self.imports.get_by_left(&new_name).is_none() && new_name != self.name {
          self.imports.insert(new_name.clone(), path.to_path_buf());
          return new_name;
        }
        i += 1;
      }
    }
    self.imports.insert(name.to_string(), path.to_path_buf());
    name.to_string()
  }

  pub fn get<S: AsRef<str>>(&self, name: S) -> Option<&Utf8PathBuf> {
    self.imports.get_by_left(name.as_ref())
  }

  pub fn iter(&self) -> bimap::hash::Iter<String, Utf8PathBuf> {
    self.imports.iter()
  }

  pub fn import_decls(&self) -> Vec<ImportDecl> {
    let mut ret = Vec::new();
    for (name, path) in self.imports.iter() {
      ret.push(ImportDecl {
        name: name.to_string(),
        path: format!("./{}", pathdiff::diff_utf8_paths(path, &self.path).unwrap())
      });
    }
    ret
  }

  pub fn decls(&self) -> Vec<Decl> {
    self.import_decls().into_iter().map(Decl::Import).collect()
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn add_same_path_different_names() {
    let mut imports = super::Imports::new("MyFile", "/MyFile");
    assert_eq!(imports.add("Foo", "/Bar"), "Foo");
    assert_eq!(imports.add("Bar", "/Bar"), "Foo");
    assert_eq!(imports.iter().count(), 1);
  }

  #[test]
  fn add_same_name_different_paths() {
    let mut imports = super::Imports::new("MyFile", "/MyFile");
    assert_eq!(imports.add("Foo", "/Foo"), "Foo");
    assert_eq!(imports.add("Foo", "/Bar"), "Bar");
    assert_eq!(imports.iter().count(), 2);
  }

  #[test]
  fn add_same_name_as_file() {
    let mut imports = super::Imports::new("MyFile", "/MyFile");
    assert_eq!(imports.add("MyFile", "/Foo/MyFile"), "FooMyFile");
    assert_eq!(imports.iter().count(), 1);
  }
}