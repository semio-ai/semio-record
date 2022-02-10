macro_rules! impl_schema_version {
  ($t: ty) => {
    pub async fn freeze<F: 'static + crate::record::Freezer>(freezer: &mut F, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      use crate::record::Freeze;
      let unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = rmp_serde::from_slice(data)?;
      let frozen = unfrozen.freeze(freezer).await?;
      Ok(rmp_serde::to_vec(&frozen)?)
    }
    
    pub fn apply_raw(data: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      let mut unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = rmp_serde::from_slice(data)?;
      unfrozen.apply_raw(action)?;
      Ok(rmp_serde::to_vec(&unfrozen)?)
    }
    
    pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(data: &[u8], actions: I) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      let mut unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = rmp_serde::from_slice(data)?;
      unfrozen.apply_raw_iter(actions)?;
      Ok(rmp_serde::to_vec(&unfrozen)?)
    }

    pub async fn apply_raw_stream<S: futures::Stream<Item = Vec<u8>> + Unpin>(data: &[u8], actions: S) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      let mut unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = rmp_serde::from_slice(data)?;
      unfrozen.apply_raw_stream(actions).await?;
      Ok(rmp_serde::to_vec(&unfrozen)?)
    }
    
    pub fn name(data: &[u8]) -> Option<String> {
      use crate::record::View;
      let unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = rmp_serde::from_slice(data).ok()?;
      unfrozen.name().map(|s| s.to_string())
    }
    
    pub fn parent(data: &[u8]) -> Option<uuid::Uuid> {
      use crate::record::View;
      let unfrozen: <$t as crate::record::RecordDefn>::Unfrozen = rmp_serde::from_slice(data).ok()?;
      unfrozen.parent().map(|u| u.clone())
    }
  };
}

pub(crate) use impl_schema_version;

