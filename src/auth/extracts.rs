use crate::auth::cache::find_state_by_access_token;
use crate::auth::types::AuthState;
use crate::result::error::AppError;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use redis::aio::ConnectionManager as RedisConnectionManager;
use std::ops::Deref;
use std::sync::Arc;
use crate::redis::RdsGetter;

pub struct Authed(pub AuthState);

impl Deref for Authed {
    type Target = AuthState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> FromRequestParts<S> for Authed
where
    RedisConnectionManagerWrapper: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(AppError::Unauthorized)?
            .to_str()?;
        let bearer_token = auth_header.trim_start_matches("Bearer ");

        let mut rds = RedisConnectionManagerWrapper::from_ref(state).clone();
        let state = find_state_by_access_token(&mut rds, bearer_token)
            .await?
            .ok_or(AppError::Unauthorized)?;

        Ok(Authed(state))
    }
}

struct RedisConnectionManagerWrapper(RedisConnectionManager);

impl Deref for RedisConnectionManagerWrapper {
    type Target = RedisConnectionManager;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> FromRef<Arc<S>> for RedisConnectionManagerWrapper where S: RdsGetter {
    fn from_ref(state: &Arc<S>) -> Self {
        // 从 AppState 中获取 DatabaseConnection
        let conn = state.rds().clone();
        RedisConnectionManagerWrapper(conn)
    }
}