use crate::result::response::AppResponse;
#[cfg(feature = "http")]
use axum::http::StatusCode;
#[cfg(feature = "http")]
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound,
    Unauthorized,
    BusinessError(i64, String),
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "http")]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let app_resp: AppResponse<()> = self.into();

        (status_code, app_resp).into_response()
    }
}

impl From<AppError> for AppResponse<()> {
    fn from(err: AppError) -> Self {
        let (code, message) = match err {
            AppError::BadRequest(msg) => (-1, msg),
            AppError::NotFound => (1, "未找到".into()),
            AppError::Unauthorized => (2, "未授权".into()),
            AppError::BusinessError(code, msg) => (code, msg),
            AppError::InternalServerError(err) => (-1, err.to_string()),
        };
        AppResponse {
            code,
            message,
            data: None,
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::InternalServerError(err.into())
    }
}