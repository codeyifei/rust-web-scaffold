use crate::result::error::AppError;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthState {
    pub user_id: u64,
    pub access_token: String,
    pub expires_at: DateTime<Local>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_expires_at: Option<DateTime<Local>>,
    pub login_at: DateTime<Local>,
}

impl TryFrom<String> for AuthState {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&value)?)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use chrono::Duration;

    #[test]
    pub fn test_serde() {
        let now = Local::now();
        let state = AuthState {
            user_id: 123123,
            access_token: String::from("123123"),
            expires_at: now + Duration::hours(4),
            refresh_token: None,
            refresh_expires_at: None,
            login_at: now,
        };

        let json = serde_json::to_string(&state).unwrap();

        let auth_state: AuthState = serde_json::from_str(&json).unwrap();

        assert_eq!(state.user_id, auth_state.user_id);
        assert_eq!(state.access_token, auth_state.access_token);
    }
}
