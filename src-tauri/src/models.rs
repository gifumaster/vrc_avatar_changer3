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
    #[serde(rename = "thumbnailVersion")]
    pub thumbnail_version: Option<i64>,
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
pub struct UploadAvatarImageRequest {
    pub avatar_id: String,
    pub image_base64: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub status: String,
    pub username: String,
    pub auth_token: Option<String>,
    pub two_factor_mode: Option<String>,
    #[serde(rename = "cacheReset")]
    pub cache_reset: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvatarCachePayload {
    #[serde(rename = "ownerUsername", default)]
    pub owner_username: Option<String>,
    pub avatars: Vec<AvatarSummary>,
    #[serde(rename = "lastSyncedAt")]
    pub last_synced_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvatarFetchProgress {
    pub phase: String,
    pub fetched: usize,
    pub total: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AvatarSwitchMethod {
    Osc,
    Api,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OscSettings {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvatarSwitchSettings {
    #[serde(default)]
    pub method: AvatarSwitchMethod,
    #[serde(default)]
    pub osc: OscSettings,
}

impl Default for AvatarSwitchMethod {
    fn default() -> Self {
        Self::Osc
    }
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

impl Default for AvatarSwitchSettings {
    fn default() -> Self {
        Self {
            method: AvatarSwitchMethod::default(),
            osc: OscSettings::default(),
        }
    }
}
