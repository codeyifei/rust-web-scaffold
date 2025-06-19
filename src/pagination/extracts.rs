use crate::pagination::types::PaginationQueryMeta;
use axum::extract::rejection::QueryRejection;
use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::ops::Deref;

#[derive(Deserialize, Debug, Clone)]
pub struct PaginationQuery<T>(pub T, pub PaginationQueryMeta);

impl<T> Deref for PaginationQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequestParts<S> for PaginationQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = QueryRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(query): Query<T> = Query::try_from_uri(&parts.uri)?;
        let Query(meta): Query<PaginationQueryMeta> = Query::try_from_uri(&parts.uri)?;

        Ok(PaginationQuery(query, meta))
    }
}