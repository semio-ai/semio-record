use async_trait::async_trait;

#[async_trait]
pub trait Apply<T, C>
where
  T: Send,
  C: Send,
{
  type Result;

  async fn apply(&mut self, context: &mut C, action: &T) -> Self::Result;
}
