use base64::Engine;
use reqwest::{
    header::HeaderMap,
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE, SET_COOKIE, USER_AGENT},
    Client,
};
use serde::Deserialize;
use serde_json::json;
use tokio::time::{sleep, Duration};

use crate::models::{AvatarSummary, LoginResult};

const API_BASE: &str = "https://api.vrchat.cloud/api/1";
const APP_USER_AGENT: &str = "AvatarChanger/0.1.0 (Windows)";
const AVATAR_PAGE_REQUEST_DELAY_MS: u64 = 1250;

pub fn app_user_agent() -> &'static str {
    APP_USER_AGENT
}

pub struct VrchatClient {
    client: Client,
}

#[derive(Debug, Deserialize)]
struct ConfigResponse {
    #[serde(rename = "clientApiKey")]
    client_api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentUserResponse {
    pub username: Option<String>,
    #[serde(rename = "requiresTwoFactorAuth")]
    pub requires_two_factor_auth: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct VerifyTwoFactorResponse {
    verified: bool,
}

#[derive(Debug, Deserialize)]
struct AvatarListItem {
    id: String,
    name: String,
    description: Option<String>,
    #[serde(rename = "thumbnailImageUrl")]
    thumbnail_image_url: Option<String>,
    #[serde(rename = "updated_at")]
    updated_at: Option<String>,
}

struct AvatarPage {
    avatars: Vec<AvatarSummary>,
    total: Option<usize>,
}

impl VrchatClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    pub fn http_client(&self) -> &Client {
        &self.client
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResult, String> {
        let api_key = self.fetch_api_key().await?;
        let basic_input = format!(
            "{}:{}",
            urlencoding::encode(username),
            urlencoding::encode(password)
        );
        let basic_auth = base64::engine::general_purpose::STANDARD.encode(basic_input);
        let url = format!("{API_BASE}/auth/user?apiKey={api_key}");

        let response = self
            .client
            .get(url)
            .header(USER_AGENT, APP_USER_AGENT)
            .header(AUTHORIZATION, format!("Basic {basic_auth}"))
            .send()
            .await
            .map_err(|error| error.to_string())?;

        if !response.status().is_success() {
            return Err(format!("VRChat login failed with status {}", response.status()));
        }

        let auth_token = extract_auth_token(&response)?;
        let body = response
            .json::<CurrentUserResponse>()
            .await
            .map_err(|error| error.to_string())?;

        let two_factor_mode = pick_two_factor_mode(body.requires_two_factor_auth.as_ref());

        Ok(LoginResult {
            status: if two_factor_mode.is_some() {
                "pending_2fa".to_string()
            } else {
                "authenticated".to_string()
            },
            username: body.username.unwrap_or_else(|| username.to_string()),
            auth_token: Some(auth_token),
            two_factor_mode,
        })
    }

    pub async fn verify_two_factor(
        &self,
        auth_token: &str,
        code: &str,
        mode: &str,
    ) -> Result<bool, String> {
        let endpoint = match mode {
            "emailotp" => "emailotp",
            _ => "totp",
        };
        let url = format!("{API_BASE}/auth/twofactorauth/{endpoint}/verify");

        let response = self
            .client
            .post(url)
            .header(USER_AGENT, APP_USER_AGENT)
            .header(COOKIE, format!("auth={auth_token}"))
            .header(CONTENT_TYPE, "application/json")
            .json(&json!({ "code": code }))
            .send()
            .await
            .map_err(|error| error.to_string())?;

        if !response.status().is_success() {
            return Err(format!(
                "VRChat two-factor verification failed with status {}",
                response.status()
            ));
        }

        let body = response
            .json::<VerifyTwoFactorResponse>()
            .await
            .map_err(|error| error.to_string())?;

        Ok(body.verified)
    }

    pub async fn get_own_avatars<F>(
        &self,
        auth_token: &str,
        mut on_progress: F,
    ) -> Result<Vec<AvatarSummary>, String>
    where
        F: FnMut(usize, Option<usize>),
    {
        let mut avatars = Vec::new();
        let mut total = None;

        for offset in (0..2000).step_by(100) {
            let page = self.get_own_avatars_page(auth_token, 100, offset).await?;
            let chunk = page.avatars;
            if total.is_none() {
                total = page.total;
            }

            let chunk_len = chunk.len();
            avatars.extend(chunk);
            on_progress(avatars.len(), total);

            if chunk_len < 100 {
                break;
            }

            sleep(Duration::from_millis(AVATAR_PAGE_REQUEST_DELAY_MS)).await;
        }

        Ok(avatars)
    }

    pub async fn get_recent_avatars(
        &self,
        auth_token: &str,
        limit: usize,
    ) -> Result<Vec<AvatarSummary>, String> {
        Ok(self.get_own_avatars_page(auth_token, limit, 0).await?.avatars)
    }

    async fn fetch_api_key(&self) -> Result<String, String> {
        let response = self
            .client
            .get(format!("{API_BASE}/config"))
            .header(USER_AGENT, APP_USER_AGENT)
            .send()
            .await
            .map_err(|error| error.to_string())?;

        if !response.status().is_success() {
            return Err(format!(
                "VRChat config fetch failed with status {}",
                response.status()
            ));
        }

        let config = response
            .json::<ConfigResponse>()
            .await
            .map_err(|error| error.to_string())?;

        Ok(config.client_api_key)
    }

    async fn get_own_avatars_page(
        &self,
        auth_token: &str,
        limit: usize,
        offset: usize,
    ) -> Result<AvatarPage, String> {
        let response = self
            .client
            .get(format!(
                "{API_BASE}/avatars?releaseStatus=all&organization=vrchat&sort=updated&order=descending&user=me&n={limit}&offset={offset}"
            ))
            .header(USER_AGENT, APP_USER_AGENT)
            .header(COOKIE, format!("auth={auth_token}"))
            .send()
            .await
            .map_err(|error| error.to_string())?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err("unauthorized".to_string());
        }

        if !response.status().is_success() {
            return Err(format!(
                "VRChat avatar list fetch failed with status {}",
                response.status()
            ));
        }

        let total = parse_total_count_header(response.headers());

        let chunk = response
            .json::<Vec<AvatarListItem>>()
            .await
            .map_err(|error| error.to_string())?;

        Ok(AvatarPage {
            avatars: chunk
                .into_iter()
                .map(|avatar| AvatarSummary {
                    id: avatar.id,
                    name: avatar.name,
                    description: avatar.description.unwrap_or_default(),
                    thumbnail_url: avatar.thumbnail_image_url,
                    thumbnail_path: None,
                    tags: Vec::new(),
                    updated_at: avatar.updated_at,
                })
                .collect(),
            total,
        })
    }
}

fn extract_auth_token(response: &reqwest::Response) -> Result<String, String> {
    for value in response.headers().get_all(SET_COOKIE) {
        let cookie = value.to_str().map_err(|error| error.to_string())?;
        if let Some(token) = cookie
            .split(';')
            .find_map(|part| part.trim().strip_prefix("auth="))
        {
            return Ok(token.to_string());
        }
    }

    Err("VRChat auth cookie was not returned".to_string())
}

fn pick_two_factor_mode(modes: Option<&Vec<String>>) -> Option<String> {
    let modes = modes?;

    if modes.iter().any(|mode| mode.eq_ignore_ascii_case("totp")) {
        Some("totp".to_string())
    } else if modes
        .iter()
        .any(|mode| mode.eq_ignore_ascii_case("emailotp") || mode.eq_ignore_ascii_case("emailOtp"))
    {
        Some("emailotp".to_string())
    } else {
        None
    }
}

fn parse_total_count_header(headers: &HeaderMap) -> Option<usize> {
    ["x-vrc-total-count", "x-total-count", "x-total"]
        .into_iter()
        .find_map(|name| headers.get(name))
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
}
