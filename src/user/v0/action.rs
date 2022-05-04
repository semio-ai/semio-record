use crate::{record::Apply, action::{SetName, SetNameError}};

use super::unfrozen::{User};
use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Display, GraphQLObject, Clone)]
pub struct SetFirstName {
  pub first_name: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
pub enum SetFirstNameError {
  #[display(fmt = "First name can't be empty")]
  Empty,
}

impl Apply<SetFirstName> for User {
  type Error = SetFirstNameError;

  fn apply(&mut self, action: &SetFirstName) -> Result<(), Self::Error> {
    if action.first_name.is_empty() {
      return Err(SetFirstNameError::Empty);
    }

    self.first_name = action.first_name.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Display, GraphQLObject, Clone)]
pub struct SetLastName {
  pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
pub enum SetLastNameError {
  #[display(fmt = "Last name can't be empty")]
  Empty,
}

impl Apply<SetLastName> for User {
  type Error = SetLastNameError;

  fn apply(&mut self, action: &SetLastName) -> Result<(), Self::Error> {
    if action.last_name.is_empty() {
      return Err(SetLastNameError::Empty);
    }

    self.last_name = action.last_name.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Display, GraphQLObject, Clone)]
pub struct SetEmail {
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
pub enum SetEmailError {
  #[display(fmt = "String isn't an email address")]
  Invalid,
  #[display(fmt = "Email can't be empty")]
  Empty,
}

impl Apply<SetEmail> for User {
  type Error = SetEmailError;

  fn apply(&mut self, action: &SetEmail) -> Result<(), Self::Error> {
    if action.email.is_empty() {
      return Err(SetEmailError::Empty);
    }

    // FIXME: Better logic here needed
    if !action.email.contains('@') {
      return Err(SetEmailError::Invalid);
    }

    if self.email == action.email {
      return Ok(());
    }

    self.email = action.email.clone();
    self.email_verified = false;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Display, GraphQLObject, Clone)]
pub struct SetEmailVerified {
  pub email_verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
pub enum SetEmailVerifiedError {
  _Dummy,
}

impl Apply<SetEmailVerified> for User {
  type Error = SetEmailVerifiedError;

  fn apply(&mut self, action: &SetEmailVerified) -> Result<(), Self::Error> {
    self.email_verified = action.email_verified;
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Display, GraphQLObject, Clone)]
pub struct SetPasswordHash {
  pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
pub enum SetPasswordHashError {
  #[display(fmt = "Password hash can't be empty")]
  Empty,
}

impl Apply<SetPasswordHash> for User {
  type Error = SetPasswordHashError;

  fn apply(&mut self, action: &SetPasswordHash) -> Result<(), Self::Error> {
    if action.password_hash.is_empty() {
      return Err(SetPasswordHashError::Empty);
    }

    self.password_hash = action.password_hash.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Display, GraphQLObject, Clone)]
pub struct SetTokenSecret  {
  pub token_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, GraphQLEnum, Clone)]
pub enum SetTokenSecretError {
  #[display(fmt = "Token secret is too short")]
  TooShort,
}

impl Apply<SetTokenSecret> for User {
  type Error = SetTokenSecretError;

  fn apply(&mut self, action: &SetTokenSecret) -> Result<(), Self::Error> {
    if action.token_secret.len() < 64 {
      return Err(SetTokenSecretError::TooShort);
    }

    self.token_secret = action.token_secret.clone();

    Ok(())
  }
}



#[derive(Debug, Serialize, Deserialize, Display, From, GraphQLUnion, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
  SetUserName(SetName),
  SetFirstName(SetFirstName),
  SetLastName(SetLastName),
  SetEmail(SetEmail),
  SetEmailVerified(SetEmailVerified),
  SetPasswordHash(SetPasswordHash),
  SetTokenSecret(SetTokenSecret),
}

#[derive(Display, Debug, Error, From, Clone)]
pub enum ActionError {
  SetUserName(SetNameError),
  SetFirstName(SetFirstNameError),
  SetLastName(SetLastNameError),
  SetEmail(SetEmailError),
  SetEmailVerified(SetEmailVerifiedError),
  SetPasswordHash(SetPasswordHashError),
  SetTokenSecret(SetTokenSecretError),
}

impl Apply<Action> for User {
  type Error = ActionError;

  fn mutates_name(action: &Action) -> bool {
    match action {
      Action::SetUserName(_) => true,
      _ => false,
    }
  }
  
  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetUserName(action) => self.apply(action)?,
      Action::SetFirstName(action) => self.apply(action)?,
      Action::SetLastName(action) => self.apply(action)?,
      Action::SetEmail(action) => self.apply(action)?,
      Action::SetEmailVerified(action) => self.apply(action)?,
      Action::SetPasswordHash(action) => self.apply(action)?,
      Action::SetTokenSecret(action) => self.apply(action)?,
    }

    Ok(())
  }
}