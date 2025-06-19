#[cfg(feature = "http")]
use crate::pagination::response::PaginationResponse;
#[cfg(feature = "http")]
use crate::pagination::types::PaginationQueryMeta;
use crate::result::error::AppError;
use serde::Serialize;
use crate::result::response::AppResponse;

pub mod error;
pub mod response;

pub type AppResult<T = ()> = Result<AppResponse<T>, AppError>;

pub fn empty() -> AppResult {
    Ok(().into())
}

pub fn ok<T>(data: T) -> AppResult<T>
where
    T: Serialize,
{
    Ok(data.into())
}

#[cfg(feature = "http")]
pub fn pagination<T>(list: Vec<T>, total: u64, meta: PaginationQueryMeta) -> AppResult<PaginationResponse<T>>
where
    T: Serialize,
{
    Ok((list, total, meta).into())
}

pub fn error(err: anyhow::Error) -> AppResult {
    Err(err.into())
}
