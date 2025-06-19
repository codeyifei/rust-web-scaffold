use axum::http::HeaderName;

pub static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");