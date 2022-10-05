use serde::{Serialize, Deserialize};

#[cfg(feature = "schemars")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
  pub private: schemars::schema::RootSchema,
  pub public: schemars::schema::RootSchema,
  pub frozen: schemars::schema::RootSchema,
  pub action: schemars::schema::RootSchema,
}

macro_rules! impl_schema_version {
  ($t: ty, $name: literal) => {
    #[cfg(feature = "schemars")]
    impl schemars::JsonSchema for $t {
      fn schema_name() -> String {
        $name.to_string()
      }

      fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema_object = schemars::schema::SchemaObject::default();
        schema_object.metadata = Some(Box::new(schemars::schema::Metadata {
          title: Some($name.to_string()),
          description: Some("This is a schema for a Semio record schema version. It is not intended to be used directly, but rather to be used as a reference for other schemas.".to_string()),
          ..Default::default()
        }));
        schema_object.into()
      }
    }

    pub async fn freeze<F: 'static + crate::record::Freezer>(freezer: &F, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      use crate::record::Freeze;
      let unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize(data)?;
      let frozen = unfrozen.freeze(freezer).await?;
      Ok(crate::serialize(&frozen)?)
    }
    
    pub fn apply_raw(data: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      let mut unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize(data)?;
      unfrozen.apply_raw(action)?;
      Ok(crate::serialize(&unfrozen)?)
    }

    pub fn apply_public_raw(data: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      type Public = <$t as crate::record::RecordDefn>::Public;
      let mut unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize::<Public>(data)?.into();
      unfrozen.apply_raw(action)?;
      Ok(crate::serialize::<Public>(&unfrozen.into())?)
    }

    pub fn apply_private_raw(data: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      type Private = <$t as crate::record::RecordDefn>::Private;
      let mut unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize::<Private>(data)?.into();
      unfrozen.apply_raw(action)?;
      Ok(crate::serialize::<Private>(&unfrozen.into())?)
    }
    
    pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(data: &[u8], actions: I) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      let mut unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize(data)?;
      unfrozen.apply_raw_iter(actions)?;
      Ok(crate::serialize(&unfrozen)?)
    }

    pub async fn apply_raw_stream<S: futures::Stream<Item = Vec<u8>> + Unpin>(data: &[u8], actions: S) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      let mut unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize(data)?;
      unfrozen.apply_raw_stream(actions).await?;
      Ok(crate::serialize(&unfrozen)?)
    }
    
    pub fn name(data: &[u8]) -> Option<String> {
      use crate::record::View;
      let unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize(data).ok()?;
      unfrozen.name().map(|s| s.to_string())
    }
    
    pub fn parent(data: &[u8]) -> Option<uuid::Uuid> {
      use crate::record::View;
      let unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize(data).ok()?;
      unfrozen.parent().map(|u| u.clone())
    }

    pub fn acl(data: &[u8]) -> Option<crate::acl::Acl> {
      use crate::record::View;
      let unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = crate::deserialize(data).ok()?;
      unfrozen.acl().map(|v| v.clone())
    }

    #[cfg(feature = "schemars")]
    pub fn schema() -> crate::schema_version::Schema {
      crate::schema_version::Schema {
        private: schemars::schema_for!(<$t as crate::record::RecordDefn>::Private),
        public: schemars::schema_for!(<$t as crate::record::RecordDefn>::Public),
        frozen: schemars::schema_for!(<$t as crate::record::RecordDefn>::Frozen),
        action: schemars::schema_for!(<$t as crate::record::RecordDefn>::Action),
      }
    }
  };
}

pub(crate) use impl_schema_version;

