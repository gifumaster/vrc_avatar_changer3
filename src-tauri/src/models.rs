use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionState {
    pub status: String,
    pub username: Option<String>,
    #[serde(rename = "twoFactorRequired")]
    pub two_factor_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvatarSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "thumbnailPath")]
    pub thumbnail_path: Option<String>,
    pub tags: Vec<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredSession {
    pub username: String,
    pub auth_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorRequest {
    pub auth_token: String,
    pub username: String,
    pub code: String,
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub status: String,
    pub username: String,
    pub auth_token: Option<String>,
    pub two_factor_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvatarCachePayload {
    pub avatars: Vec<AvatarSummary>,
    #[serde(rename = "lastSyncedAt")]
    pub last_synced_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OscSettings {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}

impl Default for OscSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            host: "127.0.0.1".to_string(),
            port: 9000,
        }
    }
}
