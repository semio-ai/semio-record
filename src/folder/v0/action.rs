// use async_trait::async_trait;
// use serde::{Serialize, Deserialize};
// use uuid::Uuid;

// use derive_more::{Display, Error};

// use crate::patch::Apply;

// use super::Folder;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AddEntry {
//   pub name: String,
//   pub id: Uuid,
// }

// #[derive(Debug, Display, Error)]
// pub enum AddEntryError {
//   #[display(fmt = "Entry or link with that name already exists")]
//   NameAlreadyExists,
//   #[display(fmt = "Entry with that id already exists")]
//   IdAlreadyExists,
// }

// #[async_trait]
// impl<C: Send> Apply<AddEntry, C> for Folder
// {
//   type Result = Result<(), AddEntryError>;
  
//   async fn apply(&mut self, _: &mut C, action: &AddEntry) -> Self::Result {
//     if self.entries.contains_key(&action.name) {
//       return Err(AddEntryError::NameAlreadyExists);
//     }

//     if self.links.contains_key(&action.name) {
//       return Err(AddEntryError::NameAlreadyExists);
//     }

//     if self.entries.values().find(|id| *id == &action.id).is_some() {
//       return Err(AddEntryError::IdAlreadyExists);
//     }

//     self.entries.insert(action.name.clone(), action.id);
    
//     Ok(())
//   }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AddEntry {
//   pub name: String,
//   pub id: Uuid,
// }

// #[derive(Debug, Display, Error)]
// pub enum AddEntryError {
//   #[display(fmt = "Entry or link with that name already exists")]
//   NameAlreadyExists,
//   #[display(fmt = "Entry with that id already exists")]
//   IdAlreadyExists,
// }

// #[async_trait]
// impl<C: Send> Apply<AddEntry, C> for Folder
// {
//   type Result = Result<(), AddEntryError>;
  
//   async fn apply(&mut self, _: &mut C, action: &AddEntry) -> Self::Result {
//     if self.entries.contains_key(&action.name) {
//       return Err(AddEntryError::NameAlreadyExists);
//     }

//     if self.links.contains_key(&action.name) {
//       return Err(AddEntryError::NameAlreadyExists);
//     }

//     if self.entries.values().find(|id| *id == &action.id).is_some() {
//       return Err(AddEntryError::IdAlreadyExists);
//     }

//     self.entries.insert(action.name.clone(), action.id);
    
//     Ok(())
//   }
// }
