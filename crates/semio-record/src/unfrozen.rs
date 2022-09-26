macro_rules! impl_unfrozen {
  ($t:ty, $action:path) => {
    impl $t {
      pub fn apply_raw(&mut self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        use crate::record::Apply;
        let action: $action = crate::deserialize(bytes)?;
        self.apply(&action)?;
        Ok(())
      }
    
      pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(&mut self, actions: I) -> Result<(), Box<dyn std::error::Error>> {
        use crate::record::Apply;
        for action in actions {
          let action: $action = crate::deserialize(action.as_ref())?;
          let _ = self.apply(&action);
        }    
        Ok(())
      }

      pub async fn apply_raw_stream<S: futures::Stream<Item = Vec<u8>> + Unpin>(&mut self, mut actions: S) -> Result<(), Box<dyn std::error::Error>> {
        use crate::record::Apply;
        use futures::StreamExt;
        while let Some(action) = actions.next().await {
          let action: $action = crate::deserialize(action.as_ref())?;
          let _ = self.apply(&action);
        }    
        Ok(())
      }
    }
  };
}

pub(crate) use impl_unfrozen;