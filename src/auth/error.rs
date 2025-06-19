use anyhow::anyhow;
use crate::result::error::AppError;

pub enum Error {
    InvalidRefreshExpiresAt,
}

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidRefreshExpiresAt => AppError::InternalServerError(anyhow!("非法的刷新token过期时间")),
        }
    }
}
