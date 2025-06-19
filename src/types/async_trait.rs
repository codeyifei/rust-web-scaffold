use async_trait::async_trait;

#[async_trait]
pub trait TryFromAsync<T>: Sized {
    type Error;
    async fn try_from_async(value: T) -> Result<Self, Self::Error>;
}

#[async_trait]
pub trait TryIntoAsync<T>: Sized {
    type Error;
    async fn try_into_async(self) -> Result<T, Self::Error>;
}

#[async_trait]
impl<T, U> TryIntoAsync<U> for T
where
    U: TryFromAsync<T>,
    T: Send,
{
    type Error = U::Error;

    async fn try_into_async(self) -> Result<U, Self::Error> {
        U::try_from_async(self).await
    }
}