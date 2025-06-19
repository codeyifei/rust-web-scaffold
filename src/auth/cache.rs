use crate::auth::error::Error;
use crate::auth::types::AuthState;
use crate::redis::types::RedisValue;
use crate::result::error::AppError;
use futures::future;
use redis::aio::ConnectionManager as RedisConnectionManager;
use redis::{AsyncTypedCommands, RedisFuture};
use serde::Serialize;
use std::fmt::Display;

static CACHE_USER_SESSIONS_PREFIX: &'static str = "auth:user_sessions:";
static CACHE_ACCESS_TOKEN_KEY_PREFIX: &'static str = "auth:access:";
static CACHE_REFRESH_TOKEN_KEY_PREFIX: &'static str = "auth:refresh:";

// 保存认证信息
pub async fn save_state(
    rds: &mut RedisConnectionManager,
    state: &AuthState,
    sso: bool,
) -> Result<(), AppError> {
    let state = RedisValue::new(state);

    if sso {
        remove_state_by_user_id(rds, state.user_id).await?;
    }

    let mut rds1 = rds.clone();
    let mut rds2 = rds.clone();
    let mut rds3 = rds.clone();

    let f1: RedisFuture<_> = rds1.zadd(
        format!("{}{}", CACHE_USER_SESSIONS_PREFIX, state.user_id),
        &state.access_token,
        state.expires_at.timestamp(),
    );
    let f2: RedisFuture<()> = rds2.set_ex(
        format!("{}{}", CACHE_ACCESS_TOKEN_KEY_PREFIX, state.access_token),
        &state,
        (state.expires_at - state.login_at).num_seconds() as u64,
    );

    if let Some(refresh_token) = &state.refresh_token {
        let f3: RedisFuture<()> = rds3.set_ex(
            format!("{}{}", CACHE_REFRESH_TOKEN_KEY_PREFIX, refresh_token),
            &state,
            (state
                .refresh_expires_at
                .ok_or(Error::InvalidRefreshExpiresAt)?
                - state.login_at)
                .num_seconds() as u64,
        );
        future::try_join3(f1, f2, f3).await?;
    } else {
        future::try_join(f1, f2).await?;
    }

    Ok(())
}

// 通过认证令牌查询认证信息
pub async fn find_state_by_access_token(
    rds: &mut RedisConnectionManager,
    access_token: &str,
) -> Result<Option<AuthState>, AppError> {
    let state = rds
        .get(format!("{}{}", CACHE_ACCESS_TOKEN_KEY_PREFIX, access_token))
        .await?;
    if let None = state {
        return Ok(None);
    }

    Ok(Some(AuthState::try_from(state.unwrap())?))
}

// 通过刷新令牌查询认证信息
pub async fn find_state_by_refresh_token(
    rds: &mut RedisConnectionManager,
    refresh_token: &str,
) -> Result<Option<AuthState>, AppError> {
    let state = rds
        .get(format!("{}{}", CACHE_REFRESH_TOKEN_KEY_PREFIX, refresh_token))
        .await?;
    if let None = state {
        return Ok(None);
    }

    Ok(Some(AuthState::try_from(state.unwrap())?))
}

// 通过认证令牌删除认证信息
pub async fn remove_state_by_access_token(
    rds: &mut RedisConnectionManager,
    access_token: &str,
) -> Result<(), AppError> {
    let state = find_state_by_access_token(rds, access_token).await?;
    if let None = state {
        return Ok(());
    }
    let state = state.unwrap();

    let mut rds1 = rds.clone();
    let mut rds2 = rds.clone();
    let mut rds3 = rds.clone();

    let f1 = rds1.zrem(
        format!("{}{}", CACHE_USER_SESSIONS_PREFIX, state.user_id),
        access_token,
    );
    let f2 = rds2.del(format!("{}{}", CACHE_ACCESS_TOKEN_KEY_PREFIX, access_token));
    if let Some(refresh_token) = &state.refresh_token {
        let f3 = rds3.del(format!(
            "{}{}",
            CACHE_REFRESH_TOKEN_KEY_PREFIX, refresh_token
        ));
        future::try_join3(f1, f2, f3).await?;
    } else {
        future::try_join(f1, f2).await?;
    }

    Ok(())
}

// 通过userId删除认证信息
pub async fn remove_state_by_user_id<Pk>(
    rds: &mut RedisConnectionManager,
    user_id: Pk,
) -> Result<(), AppError>
where
    Pk: Serialize + PartialEq + Display,
{
    let access_tokens = rds
        .zrange(format!("{}{}", CACHE_USER_SESSIONS_PREFIX, user_id), 0, -1)
        .await?;
    if access_tokens.len() == 0 {
        return Ok(());
    }
    let access_token_keys = access_tokens
        .iter()
        .map(|access_token| format!("{}{}", CACHE_ACCESS_TOKEN_KEY_PREFIX, access_token))
        .collect::<Vec<String>>();
    let states = rds.mget(&access_token_keys).await?;

    let mut refresh_token_keys: Vec<String> = vec![];
    for state in states {
        if let Some(state) = state {
            let state: AuthState = serde_json::from_str(&state)?;
            if let Some(refresh_token) = state.refresh_token {
                refresh_token_keys.push(format!(
                    "{}{}",
                    CACHE_REFRESH_TOKEN_KEY_PREFIX, refresh_token
                ));
            }
        }
    }

    let mut keys = vec![format!("{}{}", CACHE_USER_SESSIONS_PREFIX, user_id)];
    keys.extend(access_token_keys);
    keys.extend(refresh_token_keys);
    rds.del(&keys).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::redis::config::Config;
    use crate::types::async_trait::TryIntoAsync;
    use chrono::{Duration, Local};
    use std::ops::Add;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_save_state_without_refresh_token() {
        let config = Config {
            addr: "redis://:Abc.123456@127.0.0.1/8".to_string(),
        };
        let mut rds: RedisConnectionManager = config.try_into_async().await.unwrap();

        let now = Local::now();
        let state = AuthState {
            user_id: 123,
            access_token: Uuid::new_v4().to_string(),
            expires_at: now.add(Duration::hours(4)),
            refresh_token: None,
            refresh_expires_at: None,
            login_at: now,
        };
        save_state(&mut rds, &state, true).await.unwrap();
    }

    #[tokio::test]
    async fn test_save_state_with_refresh_token() {
        let config = Config {
            addr: "redis://:Abc.123456@127.0.0.1/8".to_string(),
        };
        let mut rds: RedisConnectionManager = config.try_into_async().await.unwrap();

        let now = Local::now();
        let state = AuthState {
            user_id: 123,
            access_token: Uuid::new_v4().to_string(),
            expires_at: now.add(Duration::hours(4)),
            refresh_token: Some(Uuid::new_v4().to_string()),
            refresh_expires_at: Some(now.add(Duration::days(15))),
            login_at: now,
        };

        save_state(&mut rds, &state, false).await.unwrap();
    }

    #[tokio::test]
    async fn test_remove_state_by_user_id() {
        let config = Config {
            addr: "redis://:Abc.123456@127.0.0.1/8".to_string(),
        };
        let mut rds: RedisConnectionManager = config.try_into_async().await.unwrap();

        remove_state_by_user_id(&mut rds, "123").await.unwrap();
    }

    #[tokio::test]
    async fn test_remove_state_by_access_token() {
        let config = Config {
            addr: "redis://:Abc.123456@127.0.0.1/8".to_string(),
        };
        let mut rds: RedisConnectionManager = config.try_into_async().await.unwrap();

        let access_token = "50723eba-1a0b-475f-8a9d-952d82dc4978";

        remove_state_by_access_token(&mut rds, access_token)
            .await
            .unwrap();
    }
}
