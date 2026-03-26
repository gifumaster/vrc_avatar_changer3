use crate::{
    auth::CredentialStore,
    cache::AvatarCache,
    models::{
        AvatarCachePayload, AvatarFetchProgress, AvatarSwitchMethod, AvatarSwitchSettings,
        LoginRequest, LoginResult, SessionState, StoredSession, TwoFactorRequest,
    },
    osc::OscClient,
    settings::SettingsStore,
    vrchat::VrchatClient,
};
use std::process::Command;
use tauri::Emitter;

const AVATAR_FETCH_PROGRESS_EVENT: &str = "avatar-fetch-progress";

#[tauri::command]
pub fn load_saved_session() -> Result<Option<StoredSession>, String> {
    CredentialStore::new()?.load_session()
}

#[tauri::command]
pub fn save_session(session: StoredSession) -> Result<(), String> {
    CredentialStore::new()?.save_session(&session)
}

#[tauri::command]
pub fn clear_saved_session() -> Result<(), String> {
    CredentialStore::new()?.clear_session()
}

#[tauri::command]
pub fn load_switch_settings() -> Result<AvatarSwitchSettings, String> {
    SettingsStore::new()?.load_switch_settings()
}

#[tauri::command]
pub fn save_switch_settings(settings: AvatarSwitchSettings) -> Result<AvatarSwitchSettings, String> {
    SettingsStore::new()?.save_switch_settings(&settings)
}

#[tauri::command]
pub fn load_cached_avatar_list() -> Result<AvatarCachePayload, String> {
    AvatarCache::new()?.load()
}

#[tauri::command]
pub fn save_avatar_tags(avatar_id: String, tags: Vec<String>) -> Result<AvatarCachePayload, String> {
    AvatarCache::new()?.update_avatar_tags(&avatar_id, tags)
}

#[tauri::command]
pub async fn login_vrchat(request: LoginRequest) -> Result<LoginResult, String> {
    let result = VrchatClient::new()
        .login(&request.username, &request.password)
        .await?;
    let mut cache_reset = false;

    if result.status == "authenticated" {
        if let Some(auth_token) = &result.auth_token {
            let cache = AvatarCache::new()?;
            let cached_payload = cache.load()?;
            let should_reset_cache = cached_payload
                .owner_username
                .as_deref()
                .is_some_and(|owner_username| owner_username != result.username);
            if should_reset_cache {
                cache.clear_avatar_list()?;
                cache_reset = true;
            }
            CredentialStore::new()?.save_session(&StoredSession {
                username: result.username.clone(),
                auth_token: auth_token.clone(),
            })?;
        }
    }

    Ok(LoginResult { cache_reset, ..result })
}

#[tauri::command]
pub async fn submit_two_factor(request: TwoFactorRequest) -> Result<SessionState, String> {
    let client = VrchatClient::new();
    let verified = client
        .verify_two_factor(&request.auth_token, &request.code, &request.mode)
        .await?;

    if !verified {
        return Err("Two-factor verification was rejected".to_string());
    }

    let cache = AvatarCache::new()?;
    let cached_payload = cache.load()?;
    let should_reset_cache = cached_payload
        .owner_username
        .as_deref()
        .is_some_and(|owner_username| owner_username != request.username);
    if should_reset_cache {
        cache.clear_avatar_list()?;
    }

    CredentialStore::new()?.save_session(&StoredSession {
        username: request.username.clone(),
        auth_token: request.auth_token.clone(),
    })?;

    Ok(SessionState {
        status: "authenticated".to_string(),
        username: Some(request.username),
        two_factor_required: false,
    })
}

#[tauri::command]
pub async fn verify_session() -> Result<SessionState, String> {
    let session = CredentialStore::new()?.load_session()?;

    match session {
        Some(session) => Ok(SessionState {
            status: "authenticated".to_string(),
            username: Some(session.username),
            two_factor_required: false,
        }),
        None => Ok(SessionState {
            status: "signed_out".to_string(),
            username: None,
            two_factor_required: false,
        }),
    }
}

#[tauri::command]
pub async fn refresh_avatar_list(app: tauri::AppHandle) -> Result<AvatarCachePayload, String> {
    let session = CredentialStore::new()?.load_session()?;
    let Some(session) = session else {
        return Err("No saved session was found".to_string());
    };

    let client = VrchatClient::new();
    let _ = app.emit(
        AVATAR_FETCH_PROGRESS_EVENT,
        AvatarFetchProgress {
            phase: "avatars".to_string(),
            fetched: 0,
            total: None,
        },
    );
    let avatars = client
        .get_own_avatars(&session.auth_token, |fetched, total| {
            let _ = app.emit(
                AVATAR_FETCH_PROGRESS_EVENT,
                AvatarFetchProgress {
                    phase: "avatars".to_string(),
                    fetched,
                    total,
                },
            );
        })
        .await?;
    let cache = AvatarCache::new()?;

    cache.store(&session.username, avatars, chrono::Utc::now().to_rfc3339())
}

#[tauri::command]
pub async fn refresh_latest_avatar_page(
    app: tauri::AppHandle,
    limit: usize,
) -> Result<AvatarCachePayload, String> {
    let session = CredentialStore::new()?.load_session()?;
    let Some(session) = session else {
        return Err("No saved session was found".to_string());
    };

    let client = VrchatClient::new();
    let _ = app.emit(
        AVATAR_FETCH_PROGRESS_EVENT,
        AvatarFetchProgress {
            phase: "avatars".to_string(),
            fetched: 0,
            total: Some(limit),
        },
    );
    let avatars = client.get_recent_avatars(&session.auth_token, limit).await?;
    let avatar_ids = avatars.iter().map(|avatar| avatar.id.clone()).collect::<Vec<_>>();
    let _ = app.emit(
        AVATAR_FETCH_PROGRESS_EVENT,
        AvatarFetchProgress {
            phase: "avatars".to_string(),
            fetched: avatars.len(),
            total: Some(limit),
        },
    );
    let cache = AvatarCache::new()?;

    let _ = cache
        .store_partial(
            &client.http_client(),
            &session.auth_token,
            &session.username,
            avatars,
            chrono::Utc::now().to_rfc3339(),
        )
        .await?;

    cache
        .refresh_thumbnails(&client.http_client(), &session.auth_token, &avatar_ids, |fetched, total| {
            let _ = app.emit(
                AVATAR_FETCH_PROGRESS_EVENT,
                AvatarFetchProgress {
                    phase: "thumbnails".to_string(),
                    fetched,
                    total: Some(total),
                },
            );
        })
        .await
}

#[tauri::command]
pub async fn cache_avatar_thumbnails(
    app: tauri::AppHandle,
    avatar_ids: Vec<String>,
) -> Result<AvatarCachePayload, String> {
    if avatar_ids.is_empty() {
        return AvatarCache::new()?.load();
    }

    let session = CredentialStore::new()?.load_session()?;
    let Some(session) = session else {
        return Err("No saved session was found".to_string());
    };

    let client = VrchatClient::new();
    let cache = AvatarCache::new()?;

    cache
        .cache_thumbnails(&client.http_client(), &session.auth_token, &avatar_ids, |fetched, total| {
            let _ = app.emit(
                AVATAR_FETCH_PROGRESS_EVENT,
                AvatarFetchProgress {
                    phase: "thumbnails".to_string(),
                    fetched,
                    total: Some(total),
                },
            );
        })
        .await
}

#[tauri::command]
pub async fn refresh_avatar_detail(
    app: tauri::AppHandle,
    avatar_id: String,
) -> Result<AvatarCachePayload, String> {
    let session = CredentialStore::new()?.load_session()?;
    let Some(session) = session else {
        return Err("No saved session was found".to_string());
    };

    let client = VrchatClient::new();
    let cache = AvatarCache::new()?;
    let avatar = client.get_avatar(&session.auth_token, &avatar_id).await?;
    let _ = cache.upsert_avatar(avatar)?;

    cache
        .refresh_thumbnails(&client.http_client(), &session.auth_token, &[avatar_id], |fetched, total| {
            let _ = app.emit(
                AVATAR_FETCH_PROGRESS_EVENT,
                AvatarFetchProgress {
                    phase: "thumbnails".to_string(),
                    fetched,
                    total: Some(total),
                },
            );
        })
        .await
}

#[tauri::command]
pub async fn switch_avatar(avatar_id: String) -> Result<(), String> {
    let settings = SettingsStore::new()?.load_switch_settings()?;

    match settings.method {
        AvatarSwitchMethod::Osc => {
            if !settings.osc.enabled {
                return Err("OSC is disabled in settings.".to_string());
            }

            OscClient::new().switch_avatar(&avatar_id, &settings.osc.host, settings.osc.port)
        }
        AvatarSwitchMethod::Api => {
            let session = CredentialStore::new()?.load_session()?;
            let Some(session) = session else {
                return Err("No saved session was found".to_string());
            };

            VrchatClient::new()
                .select_avatar(&session.auth_token, &avatar_id)
                .await
        }
    }
}

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), String> {
    Command::new("cmd")
        .args(["/C", "start", "", &url])
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}
