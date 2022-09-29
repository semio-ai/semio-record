use std::collections::LinkedList;

use derive_more::From;

#[derive(Debug, From)]
pub enum BlockComponent {
  Line(String),
  Block(Block),
}

impl BlockComponent {
  pub fn into_line(self) -> Option<String> {
    match self {
      BlockComponent::Line(line) => Some(line),
      BlockComponent::Block(block) => None,
    }
  }

  pub fn into_block(self) -> Option<Block> {
    match self {
      BlockComponent::Line(line) => None,
      BlockComponent::Block(block) => Some(block),
    }
  }
}

#[derive(Debug)]
pub struct Block {
  pub components: Vec<BlockComponent>,
}

impl Block {
  pub fn new() -> Self {
    Self {
      components: Vec::new(),
    }
  }

  pub fn add_line<S: ToString>(&mut self, line: S) {
    self.components.push(line.to_string().into());
  }

  pub fn add_block(&mut self, block: Block) {
    self.components.push(block.into());
  }

  pub fn concat_block(&mut self, block: Block) {
    for component in block.components {
      self.components.push(component);
    }
  }

  pub fn to_string(&self, indent: u32) -> String {
    let indent_string = "  ".repeat(indent as usize);
    self
      .components
      .iter()
      .map(|component| match component {
        BlockComponent::Line(line) => format!("{}{}", indent_string, line),
        BlockComponent::Block(block) => block.to_string(indent + 1),
      })
      .collect::<Vec<_>>()
      .join("\n")
  }

  pub fn into_single_line(self) -> Option<String> {
    if self.components.len() == 1 {
      self.components.into_iter().next().unwrap().into_line()
    } else {
      None
    }
  }
}

pub trait ToTypescript {
  fn to_typescript(&self) -> Block;
}

pub struct InterfaceField {
  pub name: String,
  pub description: Option<String>,
  pub optional: Option<bool>,
  pub ty: TypeExpr
}

impl ToTypescript for InterfaceField {
  fn to_typescript(&self) -> Block {
    let mut ret = Block::new();
    if let Some(description) = &self.description {
      ret.add_line(format!("/** {} */\n", description));
    }

    let mut line = String::new();
    line.push_str(&format!("{}", self.name));
    if self.optional.unwrap_or(false) {
      line.push_str("?");
    }
    line.push_str(": ");
    let ty = self.ty.to_typescript().into_single_line().unwrap();
    line.push_str(&ty);
    
    line.push_str(";");

    ret.add_line(line);

    ret
  }
}

pub struct InterfaceDecl {
  pub export: Option<bool>,
  pub name: String,
  pub fields: Vec<InterfaceField>,
}

impl InterfaceDecl {
  pub fn new<N: ToString>(name: N) -> Self {
    Self {
      export: None,
      name: name.to_string(),
      fields: Vec::new(),
    }
  }

  pub fn export<N: ToString>(name: N) -> Self {
    Self {
      export: Some(true),
      name: name.to_string(),
      fields: Vec::new(),
    }
  }

  pub fn add_field(&mut self, field: InterfaceField) {
    self.fields.push(field);
  }
}

impl ToTypescript for InterfaceDecl {
  fn to_typescript(&self) -> Block {
    let mut ret = Block::new();
    let mut line = String::new();
    if self.export.unwrap_or(false) {
      line.push_str("export ");
    }
    line.push_str(&format!("interface {} {{", self.name));
    ret.add_line(line);

    let mut inner = Block::new();
    for field in &self.fields {
      inner.concat_block(field.to_typescript());
    }
    ret.add_block(inner);
    ret.add_line("}");
    ret
  }
}

pub enum TypeExpr {
  Identifier(String),
  Union(Vec<TypeExpr>),
  Dictionary(Box<TypeExpr>),
  StringLiteral(String),
  NumberLiteral(i64),
  Array(Box<TypeExpr>),
}

impl TypeExpr {
  pub fn identifier<S: ToString>(string: S) -> Self {
    Self::Identifier(string.to_string())
  }

  pub fn union(types: Vec<TypeExpr>) -> Self {
    Self::Union(types)
  }

  pub fn string_literal<S: ToString>(string: S) -> Self {
    Self::StringLiteral(string.to_string())
  }

  pub fn number_literal(number: i64) -> Self {
    Self::NumberLiteral(number)
  }

  pub fn dictionary<T: Into<TypeExpr>>(ty: T) -> Self {
    Self::Dictionary(Box::new(ty.into()))
  }

  pub fn array<T: Into<TypeExpr>>(ty: T) -> Self {
    Self::Array(Box::new(ty.into()))
  }
}

impl ToTypescript for TypeExpr {
  fn to_typescript(&self) -> Block {
    let mut ret = Block::new();
    match self {
      TypeExpr::Identifier(name) => ret.add_line(name),
      TypeExpr::Union(types) => {
        let mut line = String::new();
        line.push_str("(");
        line.push_str(&types.iter().map(|ty| ty.to_typescript().into_single_line().unwrap()).collect::<Vec<_>>().join(" | "));
        line.push_str(")");
        ret.add_line(line)
      },
      TypeExpr::StringLiteral(value) => ret.add_line(format!("\"{}\"", value)),
      TypeExpr::NumberLiteral(value) => ret.add_line(format!("{}", value)),
      TypeExpr::Dictionary(ty) => {
        let mut line = String::new();
        line.push_str("{ [key: string]: ");
        line.push_str(&ty.to_typescript().into_single_line().unwrap());
        line.push_str(" }");
        ret.add_line(line)
      },
      TypeExpr::Array(ty) => {
        let mut line = String::new();
        line.push_str(&ty.to_typescript().into_single_line().unwrap());
        line.push_str("[]");
        ret.add_line(line)
      }
    }
    ret
  }
}

pub struct TypeDecl {
  pub name: String,
  pub ty: TypeExpr,
}

impl TypeDecl {
  pub fn new<N: ToString>(name: N, ty: TypeExpr) -> Self {
    Self {
      name: name.to_string(),
      ty,
    }
  }
}

impl ToTypescript for TypeDecl {
  fn to_typescript(&self) -> Block {
    let mut ret = Block::new();
    let mut line = String::new();
    line.push_str(&format!("type {} = ", self.name));
    line.push_str(&self.ty.to_typescript().into_single_line().unwrap());
    line.push_str(";");
    ret.add_line(line);
    ret
  }
}

pub struct NamespaceDecl {
  pub name: String,
  pub decls: Vec<Decl>,
}

impl NamespaceDecl {
  pub fn new<N: ToString>(name: N) -> Self {
    Self {
      name: name.to_string(),
      decls: Vec::new(),
    }
  }

  pub fn append<D: Into<Decl>>(&mut self, decl: D) {
    self.decls.push(decl.into());
  }
}

impl ToTypescript for NamespaceDecl {
  fn to_typescript(&self) -> Block {
    let mut ret = Block::new();
    
    let mut line = String::new();
    line.push_str(&format!("namespace {} {{", self.name));
    ret.add_line(line);

    let mut inner = Block::new();
    for decl in &self.decls {
      inner.concat_block(decl.to_typescript());
    }

    ret.add_block(inner);
    ret.add_line("}".to_string());
    ret
  }
}

pub struct ImportDecl {
  pub name: String,
  pub path: String,
}

impl ToTypescript for ImportDecl {
  fn to_typescript(&self) -> Block {
    let mut ret = Block::new();
    ret.add_line(format!("import {} from \"{}\";", self.name, self.path));
    ret
  }
}

pub struct ExportDefaultDecl {
  pub identifier: String,
}

impl ToTypescript for ExportDefaultDecl {
  fn to_typescript(&self) -> Block {
    let mut ret = Block::new();
    ret.add_line(format!("export default {};", self.identifier));
    ret
  }
}

#[derive(From)]
pub enum Decl {
  Type(TypeDecl),
  Interface(InterfaceDecl),
  Namespace(NamespaceDecl),
  Import(ImportDecl),
  ExportDefault(ExportDefaultDecl),
}

impl ToTypescript for Decl {
  fn to_typescript(&self) -> Block {
    match self {
      Decl::Type(decl) => decl.to_typescript(),
      Decl::Interface(decl) => decl.to_typescript(),
      Decl::Namespace(decl) => decl.to_typescript(),
      Decl::Import(decl) => decl.to_typescript(),
      Decl::ExportDefault(decl) => decl.to_typescript(),
    }
  }
}

pub struct File {
  pub decls: LinkedList<Decl>,
}

impl File {
  pub fn new() -> Self {
    Self {
      decls: LinkedList::new(),
    }
  }

  pub fn append<D: Into<Decl>>(&mut self, decl: D) {
    self.decls.push_back(decl.into());
  }

  pub fn prepend_all<D: Into<Decl>, I: Iterator<Item = D>>(&mut self, decls: I) {
    for decl in decls {
      self.decls.push_front(decl.into());
    }
  }
}

impl ToTypescript for File {
  fn to_typescript(&self) -> Block {
    let mut ret = Block::new();
    for decl in &self.decls {
      ret.concat_block(decl.to_typescript());
    }
    ret
  }
}