use std::{collections::{BTreeMap, LinkedList}, path::PathBuf};

use semio_record::{schema, schema_version::Schema as VersionSchema, record::*};
use schemars::schema::{Schema, RootSchema, SchemaObject};
use serde::{Serialize, Deserialize};

use tokio::fs::{create_dir, create_dir_all};
use std::pin::Pin;
use std::future::Future;

use clap::Parser;
use tokio::io::AsyncWriteExt;

fn merge_into<I: Iterator<Item = (String, Schema)>>(definitions: I, res: &mut BTreeMap<String, Schema>) {
  for (name, schema) in definitions {
    if res.contains_key(&name) {
      continue;
    }
    res.insert(name, schema);
  }
}

fn merge_root_schema(root_schema: RootSchema, res: &mut BTreeMap<String, Schema>) {
  if let Some(metadata) = &root_schema.schema.metadata {
    if let Some(title) = &metadata.title {
      res.insert(title.clone(), Schema::Object(root_schema.schema));
    }
  }
  
  merge_into(root_schema.definitions.into_iter(), res);
  
}

fn merge_version(version_schema: VersionSchema, res: &mut BTreeMap<String, Schema>) {
  merge_root_schema(version_schema.public, res);
  merge_root_schema(version_schema.private, res);
  merge_root_schema(version_schema.action, res);
  merge_root_schema(version_schema.frozen, res);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scope {
  definitions: BTreeMap<String, Schema>,
  children: BTreeMap<String, Scope>,
}

fn lookup_scope<'a, 'b>(root: &'a mut Scope, parts: &'b [&str]) -> &'a mut Scope {
  if parts.len() == 0 {
    return root;
  }
  
  let (first, rest) = parts.split_first().unwrap();
  
  let child = root.children.entry(first.to_string()).or_insert(Scope {
    definitions: BTreeMap::new(),
    children: BTreeMap::new(),
  });
  
  lookup_scope(child, rest)
}

fn rescope(definitions: BTreeMap<String, Schema>) -> Scope {
  let mut root = Scope {
    definitions: BTreeMap::new(),
    children: BTreeMap::new(),
  };

  for (name, definition) in definitions {
    let parts: Vec<&str> = name.split('_').collect();
    let scope = lookup_scope(&mut root, &parts[..parts.len() - 1]);
    scope.definitions.insert(parts[parts.len() - 1].to_string(), definition);
  }

  root
}



fn parse_schema_object(name: &str, object: &SchemaObject) {
  if name != "Action" {
    return;
  }

  println!("{:#?}", object);
}

fn parse_schema(name: &str, schema: &Schema) {
  match schema {
    Schema::Object(object) => parse_schema_object(name, object),
    _ => {}
  }
}

async fn write_file(path: PathBuf, name: &str, schema: &Schema) -> tokio::io::Result<()> {
  let mut file = tokio::fs::File::create(path).await?;
  parse_schema(name, schema);
  file.write_all(serde_json::to_string_pretty(&schema).unwrap().as_bytes()).await?;
  Ok(())
}

fn write_scope<'a>(path: PathBuf, scope: &'a Scope) -> Pin<Box<dyn Future<Output = tokio::io::Result<()>> + 'a>> {
  Box::pin(async move {
    create_dir(&path).await?;

    for (name, definition) in &scope.definitions {
      let mut definition_path = path.clone();
      definition_path.push(format!("{}.ts", name));
      write_file(definition_path, &name, &definition).await?;
    }

    for (name, scope) in scope.children.iter() {
      let mut child_path = path.clone();
      child_path.push(name);
      write_scope(child_path, &scope).await?;
    }

    Ok(())
  })
}

#[derive(Parser, Debug)]
struct Args {
  #[clap(short, long)]
  outdir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();
  
  let mut definitions = BTreeMap::new();
  
  merge_version(schema(TYPE_FOLDER, 0)?, &mut definitions);
  merge_version(schema(TYPE_USER, 0)?, &mut definitions);
  merge_version(schema(TYPE_ORGANIZATION, 0)?, &mut definitions);
  merge_version(schema(TYPE_STRUCTURE, 0)?, &mut definitions);
  merge_version(schema(TYPE_ENUMERATION, 0)?, &mut definitions);
  merge_version(schema(TYPE_MODULE, 0)?, &mut definitions);
  merge_version(schema(TYPE_ANIMATION, 0)?, &mut definitions);
  merge_version(schema(TYPE_ANIMATION, 1)?, &mut definitions);

  let root = rescope(definitions);
  let mut queue = LinkedList::new();
  // (Scope, indent)
  queue.push_front((("root", &root), 0));
  while let Some(((name, scope), indent)) = queue.pop_front() {
    println!("{}{}", "  ".repeat(indent), name);
    for (name, _) in &scope.definitions {
      println!("{}{}", "  ".repeat(indent + 1), name);
    }
    for (name, child) in &scope.children {
      queue.push_front(((name, child), indent + 1));
    }
  }

  write_scope(args.outdir, &root).await?;


  Ok(())
}