macro_rules! impl_unfrozen {
  ($t:ty, $action:path) => {
    impl $t {
      pub fn apply_raw(&mut self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        use crate::record::Apply;
        let action: $action = rmp_serde::from_slice(bytes)?;
        self.apply(&action)?;
        Ok(())
      }
    
      pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(&mut self, actions: I) -> Result<(), Box<dyn std::error::Error>> {
        use crate::record::Apply;
        for action in actions {
          let action: $action = rmp_serde::from_slice(action.as_ref())?;
          let _ = self.apply(&action);
        }    
        Ok(())
      }
    }
  };
}

pub(crate) use impl_unfrozen;