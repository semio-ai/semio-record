use anyhow::Result;

pub trait Migrate {
  fn migrate(from_version: i16, from: &[u8]) -> Result<Self>
    where Self: Sized;
}