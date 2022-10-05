use std::{fmt::{Debug, Formatter}, collections::{HashSet, HashMap}};

use crate::{
  user::v0::User,
  record::RecordDefn,
  organization::v0::Organization,
  module::v0::Module,
  structure::v0::Structure,
  enumeration::v0::Enumeration,
  folder::v0::Folder,
  animation::v1::Animation,
  scene::v0::Scene,
  workspace::v0::Workspace,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use derive_more::From;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Token", rename_all = "camelCase")]
pub struct Token {
  /// The raw token data
  pub token: String,

  /// The time (as a RFC 3339/ISO 8601 timestamp) at which the token expires.
  pub expires_at: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "api_View", tag = "type", content = "value", rename_all = "camelCase")]
pub enum View<R: RecordDefn> {
  #[serde(rename = "pub")]
  Public(R::Public),
  #[serde(rename = "pri")]
  Private(R::Private),
}

impl<R: RecordDefn> View<R> {
  pub fn public(unfrozen: R::Unfrozen) -> Self {
    Self::Public(unfrozen.into())
  }

  pub fn private(unfrozen: R::Unfrozen) -> Self {
    Self::Private(unfrozen.into())
  }
}

#[cfg(feature = "schemars")]
impl<R: RecordDefn + schemars::JsonSchema> schemars::JsonSchema for View<R> {
  fn schema_name() -> String {
    format!("api_View_{}", R::schema_name())
  }

  fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = schemars::schema::SchemaObject::default();
    schema.metadata = Some(Box::new(schemars::schema::Metadata {
      title: Some(format!("api_View_{}", R::schema_name())),
      ..Default::default()
    }));

    let public = {
      let mut obj = schemars::schema::SchemaObject::default();
      let obj_validation = obj.object();
      obj_validation.properties.insert("type".to_string(), schemars::schema::SchemaObject {
        enum_values: Some(vec!["pub".into()]),
        ..Default::default()
      }.into());
      obj_validation.properties.insert("value".to_string(), schemars::schema::SchemaObject {
        reference: Some(format!("#/definitions/{}", R::Public::schema_name())),
        ..Default::default()
      }.into());

      obj_validation.required.insert("type".to_string());
      obj_validation.required.insert("value".to_string());

      schemars::schema::Schema::Object(obj)
    };

    let private = {
      let mut obj = schemars::schema::SchemaObject::default();
      let obj_validation = obj.object();
      obj_validation.properties.insert("type".to_string(), schemars::schema::SchemaObject {
        enum_values: Some(vec!["pri".into()]),
        ..Default::default()
      }.into());
      obj_validation.properties.insert("value".to_string(), schemars::schema::SchemaObject {
        reference: Some(format!("#/definitions/{}", R::Private::schema_name())),
        ..Default::default()
      }.into());

      obj_validation.required.insert("type".to_string());
      obj_validation.required.insert("value".to_string());

      schemars::schema::Schema::Object(obj)
    };


    schema
      .subschemas()
      .one_of = Some(vec![public, private]);
    schema.into()
  }
}

impl<R: RecordDefn> Clone for View<R>
where
  R::Public: Clone,
  R::Private: Clone,
{
  fn clone(&self) -> Self {
    match self {
      View::Public(r) => View::Public(r.clone()),
      View::Private(r) => View::Private(r.clone()),
    }
  }
}

impl<R: RecordDefn> Debug for View<R>
where
  R::Public: Debug,
  R::Private: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      View::Public(r) => r.fmt(f),
      View::Private(r) => r.fmt(f),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Record_Kind", tag = "type", content = "view", rename_all = "camelCase")]
pub enum RecordKind {
  User(View<User>),
  Organization(View<Organization>),
  Module(View<Module>),
  Structure(View<Structure>),
  Enumeration(View<Enumeration>),
  Folder(View<Folder>),
  Animation(View<Animation>),
  Scene(View<Scene>),
  Workspace(View<Workspace>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Record", rename_all = "camelCase")]
pub struct Record {
  pub last_modified: String,
  pub version: i64,
  pub kind: RecordKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Failure", rename_all = "camelCase")]
pub struct Failure {
  code: Option<u32>,
  message: Option<String>,
}

impl From<String> for Failure {
  fn from(s: String) -> Self {
    Self {
      code: None,
      message: Some(s),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Result<S> {
  Success(S),
  Failure(Failure),
}

#[cfg(feature = "schemars")]
impl<S: schemars::JsonSchema> schemars::JsonSchema for Result<S> {
  fn schema_name() -> String {
    format!("api_Result_{}", S::schema_name())
  }

  fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = schemars::schema::SchemaObject::default();
    schema.metadata = Some(Box::new(schemars::schema::Metadata {
      title: Some(Self::schema_name()),
      ..Default::default()
    }));

    let success = {
      let mut obj = schemars::schema::SchemaObject::default();
      let obj_validation = obj.object();
      obj_validation.properties.insert("type".to_string(), schemars::schema::SchemaObject {
        enum_values: Some(vec!["success".into()]),
        ..Default::default()
      }.into());
      
      let success_schema = S::json_schema(gen);
      gen.definitions_mut().insert(S::schema_name(), success_schema);
      obj_validation.properties.insert("value".to_string(), schemars::schema::SchemaObject {
        reference: Some(format!("#/definitions/{}", S::schema_name())),
        ..Default::default()
      }.into());

      obj_validation.required.insert("type".to_string());
      obj_validation.required.insert("value".to_string());

      schemars::schema::Schema::Object(obj)
    };

    let failure = {
      let mut obj = schemars::schema::SchemaObject::default();
      let obj_validation = obj.object();
      obj_validation.properties.insert("type".to_string(), schemars::schema::SchemaObject {
        enum_values: Some(vec!["failure".into()]),
        ..Default::default()
      }.into());
      let failure_schema = Failure::json_schema(gen);
      gen.definitions_mut().insert(Failure::schema_name(), failure_schema);
      obj_validation.properties.insert("value".to_string(), schemars::schema::SchemaObject {
        reference: Some(format!("#/definitions/{}", Failure::schema_name())),
        ..Default::default()
      }.into());


      obj_validation.required.insert("type".to_string());
      obj_validation.required.insert("value".to_string());

      schemars::schema::Schema::Object(obj)
    };


    schema
      .subschemas()
      .one_of = Some(vec![success, failure]);
    schema.into()
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateFolderReq", rename_all = "camelCase")]
pub struct CreateFolderReq {
  pub parent: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateEnumerationReq", rename_all = "camelCase")]
pub struct CreateEnumerationReq {
  pub parent: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateStructureReq", rename_all = "camelCase")]
pub struct CreateStructureReq {
  pub parent: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateModuleReq", rename_all = "camelCase")]
pub struct CreateModuleReq {
  pub parent: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateOrganizationReq", rename_all = "camelCase")]
pub struct CreateOrganizationReq {
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateAnimationReq", rename_all = "camelCase")]
pub struct CreateAnimationReq {
  pub parent: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateSceneReq", rename_all = "camelCase")]
pub struct CreateSceneReq {
  pub parent: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateWorkspaceReq", rename_all = "camelCase")]
pub struct CreateWorkspaceReq {
  pub parent: Uuid,
  pub name: String,
}

/// Retain a record. The client will receive updates as the record changes.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_RetainReq", rename_all = "camelCase")]
pub struct RetainReq {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_RetainSuccess", rename_all = "camelCase")]
pub struct RetainSuccess {
  pub id: Uuid,
  pub version: i64,
  pub record: Record,
}

/// Fetch a record. The client will *NOT* receive updates as the record changes.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_GetReq", rename_all = "camelCase")]
pub struct GetReq {
  pub id: Uuid,
}


#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_GetSuccess", rename_all = "camelCase")]
pub struct GetSuccess {
  pub id: Uuid,
  pub record: Record,
}

/// Fetch multiple records. The client will *NOT* receive updates as the record changes.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_GetManyReq", rename_all = "camelCase")]
pub struct GetManyReq {
  pub ids: HashSet<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_GetManySuccess", rename_all = "camelCase")]
pub struct GetManySuccess {
  pub records: HashMap<Uuid, Record>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_ChildrenReq", rename_all = "camelCase")]
pub struct ChildrenReq {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_ChildrenSuccess", rename_all = "camelCase")]
pub struct ChildrenSuccess {
  pub id: Uuid,
  pub children: Vec<Uuid>,
}

/// Release a retained record.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_ReleaseReq", rename_all = "camelCase")]
pub struct ReleaseReq {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_CreateSuccess", rename_all = "camelCase")]
pub struct CreateSuccess {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_LoginReq", rename_all = "camelCase")]
pub struct LoginReq {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_LoginWithTokenReq", rename_all = "camelCase")]
pub struct LoginWithTokenReq {
  pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_LoginWithTokenSuccess", rename_all = "camelCase")]
pub struct LoginWithTokenSuccess {
  pub user_id: Uuid,
  pub user: <User as RecordDefn>::Private,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_RefreshTokenReq", rename_all = "camelCase")]
pub struct RefreshTokenReq {
  pub user_id: Uuid,
  pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_RefreshTokenSuccess", rename_all = "camelCase")]
pub struct RefreshTokenSuccess {
  pub access: Token,
  pub refresh: Option<Token>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_LoginSuccess", rename_all = "camelCase")]
pub struct LoginSuccess {
  pub user_id: Uuid,
  pub access_token: Token,
  pub refresh_token: Option<Token>,
  pub user: <User as RecordDefn>::Private,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_SignupReq", rename_all = "camelCase")]
pub struct SignupReq {
  pub username: String,
  pub password: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_SignupSuccess", rename_all = "camelCase")]
pub struct SignupSuccess {
  pub user_id: Uuid,
  pub access_token: Token,
  pub refresh_token: Option<Token>,
  pub user: <User as RecordDefn>::Private,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Apply", rename_all = "camelCase")]
pub struct Apply<A> {
  pub id: Uuid,
  pub action: A
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_ApplySuccess", rename_all = "camelCase")]
pub struct ApplySuccess {
  pub id: Uuid,
  pub version: i64,
  pub last_modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Req", tag = "type", content = "value", rename_all = "camelCase")]

pub enum Req {
  CreateFolder(CreateFolderReq),
  CreateOrganization(CreateOrganizationReq),
  CreateEnumeration(CreateEnumerationReq),
  CreateStructure(CreateStructureReq),
  CreateModule(CreateModuleReq),
  CreateAnimation(CreateAnimationReq),
  CreateWorkspace(CreateWorkspaceReq),
  CreateScene(CreateSceneReq),
  Retain(RetainReq),
  Release(ReleaseReq),
  Get(GetReq),
  GetMany(GetManyReq),
  Children(ChildrenReq),
  Login(LoginReq),
  LoginWithToken(LoginWithTokenReq),
  RefreshToken(RefreshTokenReq),
  Signup(SignupReq),
  ApplyUser(Apply<<User as RecordDefn>::Action>),
  ApplyOrganization(Apply<<Organization as RecordDefn>::Action>),
  ApplyModule(Apply<<Module as RecordDefn>::Action>),
  ApplyStructure(Apply<<Structure as RecordDefn>::Action>),
  ApplyEnumeration(Apply<<Enumeration as RecordDefn>::Action>),
  ApplyFolder(Apply<<Folder as RecordDefn>::Action>),
  ApplyAnimation(Apply<<Animation as RecordDefn>::Action>),
  ApplyWorkspace(Apply<<Workspace as RecordDefn>::Action>),
  ApplyScene(Apply<<Scene as RecordDefn>::Action>),
}

#[derive(Debug, Serialize, Deserialize, From)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Res", tag = "type", content = "value", rename_all = "camelCase")]
pub enum Res {
  Create(Result<CreateSuccess>),
  Retain(Result<RetainSuccess>),
  Release(Result<()>),
  Get(Result<GetSuccess>),
  GetMany(Result<GetManySuccess>),
  Children(Result<ChildrenSuccess>),
  Login(Result<LoginSuccess>),
  LoginWithToken(Result<LoginWithTokenSuccess>),
  Signup(Result<SignupSuccess>),
  Apply(Result<ApplySuccess>),
  RefreshToken(Result<RefreshTokenSuccess>),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Msg_Kind", tag = "type", content = "value", rename_all = "snake_case")]
pub enum MsgKind {
  Req(Req),
  Res(Res),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "api_Msg", rename_all = "camelCase")]
pub struct Msg {
  pub id: Uuid,
  pub kind: MsgKind,
}

impl Msg {
  fn reply<R: Into<Res>>(&self, res: R) -> Self {
    Self {
      id: self.id,
      kind: MsgKind::Res(res.into()),
    }
  }

  pub fn fail<F: Into<Failure>>(&self, failure: F) -> Self {
    if let MsgKind::Req(req) = &self.kind {
      match req {
        Req::CreateFolder(_) |
        Req::CreateOrganization(_) |
        Req::CreateStructure(_) |
        Req::CreateEnumeration(_) |
        Req::CreateAnimation(_) |
        Req::CreateScene(_) |
        Req::CreateWorkspace(_) |
        Req::CreateModule(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::Create(Result::Failure(failure.into()))),
          }
        },
        Req::ApplyUser(_) |
        Req::ApplyOrganization(_) |
        Req::ApplyModule(_) |
        Req::ApplyStructure(_) |
        Req::ApplyEnumeration(_) |
        Req::ApplyFolder(_) |
        Req::ApplyScene(_) |
        Req::ApplyWorkspace(_) |
        Req::ApplyAnimation(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::Apply(Result::Failure(failure.into())))
          }
        },
        Req::Login(_) |
        Req::LoginWithToken(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::LoginWithToken(Result::Failure(failure.into()))),
          }
        },
        Req::Signup(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::Signup(Result::Failure(failure.into()))),
          }
        },
        Req::Get(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::Get(Result::Failure(failure.into()))),
          }
        },
        Req::GetMany(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::GetMany(Result::Failure(failure.into()))),
          }
        },
        Req::Children(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::Children(Result::Failure(failure.into()))),
          }
        },
        Req::Retain(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::Retain(Result::Failure(failure.into()))),
          }
        },
        Req::Release(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::Release(Result::Failure(failure.into()))),
          }
        },
        Req::RefreshToken(_) => {
          Self {
            id: self.id,
            kind: MsgKind::Res(Res::RefreshToken(Result::Failure(failure.into()))),
          }
        },
      }
    } else {
      panic!("cannot fail a response");
    }
  }
}