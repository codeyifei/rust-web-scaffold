#[cfg(feature = "http")]
use crate::pagination::{response::PaginationResponse, types::PaginationQueryMeta};
use crate::result::error::AppError;
#[cfg(feature = "http")]
use axum::body::Body;
#[cfg(feature = "http")]
use axum::http::StatusCode;
#[cfg(feature = "http")]
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize, Debug, Clone)]
pub struct AppResponse<T> {
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}

#[cfg(feature = "http")]
impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}

impl<T: Serialize> AppResponse<T> {
    pub fn data(data: T) -> Self {
        Self {
            code: 0,
            message: "Success!".to_string(),
            data: Some(data),
        }
    }
}

impl AppResponse<()> {
    pub fn error(err: AppError) -> Self {
        err.into()
    }
}

impl<T: Serialize> From<T> for AppResponse<T> {
    fn from(value: T) -> Self {
        AppResponse::data(value)
    }
}

#[cfg(feature = "http")]
impl<T> From<(Vec<T>, u64, PaginationQueryMeta)> for AppResponse<PaginationResponse<T>>
where
    T: Serialize,
{
    fn from(value: (Vec<T>, u64, PaginationQueryMeta)) -> Self {
        AppResponse::data(value.into())
    }
}
