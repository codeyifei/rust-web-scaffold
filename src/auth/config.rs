use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub enable_refresh_token: bool,
    #[serde(default)]
    pub enable_sso: bool,
}
