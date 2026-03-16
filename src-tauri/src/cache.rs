use std::{collections::HashSet, fs, path::PathBuf};

use reqwest::{
    header::{CONTENT_TYPE, COOKIE, USER_AGENT},
    Client,
};
use tokio::time::{sleep, Duration};

use crate::{
    models::{AvatarCachePayload, AvatarSummary},
    vrchat::app_user_agent,
};

const APP_DIR_NAME: &str = "AvatarChanger";
const CACHE_DIR_NAME: &str = "cache";
const THUMBNAIL_DIR_NAME: &str = "thumbnails";
const AVATAR_CACHE_FILE: &str = "avatars.json";
const THUMBNAIL_REQUEST_DELAY_MS: u64 = 250;

pub struct AvatarCache {
    base_dir: PathBuf,
    thumbnail_dir: PathBuf,
    cache_file: PathBuf,
}

impl AvatarCache {
    pub fn new() -> Result<Self, String> {
        let base_dir = dirs::data_local_dir()
            .ok_or_else(|| "Could not resolve LocalAppData directory".to_string())?
            .join(APP_DIR_NAME)
            .join(CACHE_DIR_NAME);
        let thumbnail_dir = base_dir.join(THUMBNAIL_DIR_NAME);
        let cache_file = base_dir.join(AVATAR_CACHE_FILE);

        fs::create_dir_all(&thumbnail_dir).map_err(|error| error.to_string())?;

        Ok(Self {
            base_dir,
            thumbnail_dir,
            cache_file,
        })
    }

    pub fn load(&self) -> Result<AvatarCachePayload, String> {
        if !self.cache_file.exists() {
            return Ok(AvatarCachePayload {
                owner_username: None,
                avatars: Vec::new(),
                last_synced_at: None,
            });
        }

        let json = fs::read_to_string(&self.cache_file).map_err(|error| error.to_string())?;
        serde_json::from_str::<AvatarCachePayload>(&json).map_err(|error| error.to_string())
    }

    pub fn thumbnail_dir_path(&self) -> PathBuf {
        self.thumbnail_dir.clone()
    }

    pub fn store(
        &self,
        owner_username: &str,
        avatars: Vec<AvatarSummary>,
        last_synced_at: String,
    ) -> Result<AvatarCachePayload, String> {
        let existing_payload = self.load()?;
        let payload = AvatarCachePayload {
            owner_username: Some(owner_username.to_string()),
            avatars: merge_cached_fields(avatars, Some(&existing_payload.avatars)),
            last_synced_at: Some(last_synced_at),
        };

        self.write_payload(&payload)?;
        Ok(payload)
    }

    pub async fn store_partial(
        &self,
        _client: &Client,
        _auth_token: &str,
        owner_username: &str,
        avatars: Vec<AvatarSummary>,
        last_synced_at: String,
    ) -> Result<AvatarCachePayload, String> {
        let existing_payload = self.load()?;
        let refreshed = merge_cached_fields(avatars, Some(&existing_payload.avatars));
        let refreshed_ids: HashSet<String> = refreshed.iter().map(|avatar| avatar.id.clone()).collect();
        let mut merged = refreshed;

        merged.extend(
            existing_payload
                .avatars
                .into_iter()
                .filter(|avatar| !refreshed_ids.contains(&avatar.id)),
        );

        let payload = AvatarCachePayload {
            owner_username: Some(owner_username.to_string()),
            avatars: merged,
            last_synced_at: Some(last_synced_at),
        };

        self.write_payload(&payload)?;
        Ok(payload)
    }

    pub async fn cache_thumbnails(
        &self,
        client: &Client,
        auth_token: &str,
        avatar_ids: &[String],
        mut on_progress: impl FnMut(usize, usize),
    ) -> Result<AvatarCachePayload, String> {
        self.cache_thumbnails_inner(client, auth_token, avatar_ids, false, |fetched, total| {
            on_progress(fetched, total);
        })
        .await
    }

    pub async fn refresh_thumbnails(
        &self,
        client: &Client,
        auth_token: &str,
        avatar_ids: &[String],
        mut on_progress: impl FnMut(usize, usize),
    ) -> Result<AvatarCachePayload, String> {
        self.cache_thumbnails_inner(client, auth_token, avatar_ids, true, |fetched, total| {
            on_progress(fetched, total);
        })
        .await
    }

    async fn cache_thumbnails_inner(
        &self,
        client: &Client,
        auth_token: &str,
        avatar_ids: &[String],
        force: bool,
        mut on_progress: impl FnMut(usize, usize),
    ) -> Result<AvatarCachePayload, String> {
        let mut payload = self.load()?;
        let requested_ids: HashSet<&str> = avatar_ids.iter().map(String::as_str).collect();
        let total = payload
            .avatars
            .iter()
            .filter(|avatar| requested_ids.contains(avatar.id.as_str()) && (force || avatar.thumbnail_path.is_none()))
            .count();

        if total == 0 {
            return Ok(payload);
        }

        let mut processed = 0usize;
        on_progress(0, total);

        for avatar in &mut payload.avatars {
            if !requested_ids.contains(avatar.id.as_str()) || (!force && avatar.thumbnail_path.is_some()) {
                continue;
            }

            if let Some(url) = avatar.thumbnail_url.clone().filter(|url| !url.is_empty()) {
                if let Ok((path, version)) = self
                    .download_thumbnail(client, auth_token, &avatar.id, &url)
                    .await
                {
                    avatar.thumbnail_path = Some(path);
                    avatar.thumbnail_version = Some(version);
                }
            }

            processed += 1;
            on_progress(processed, total);
            if processed < total {
                sleep(Duration::from_millis(THUMBNAIL_REQUEST_DELAY_MS)).await;
            }
        }

        self.write_payload(&payload)?;
        Ok(payload)
    }

    pub fn update_avatar_tags(
        &self,
        avatar_id: &str,
        tags: Vec<String>,
    ) -> Result<AvatarCachePayload, String> {
        let mut payload = self.load()?;
        let avatar = payload
            .avatars
            .iter_mut()
            .find(|avatar| avatar.id == avatar_id)
            .ok_or_else(|| "Avatar was not found in cache".to_string())?;

        avatar.tags = tags;

        self.write_payload(&payload)?;
        Ok(payload)
    }

    pub fn clear_avatar_list(&self) -> Result<AvatarCachePayload, String> {
        let payload = AvatarCachePayload {
            owner_username: None,
            avatars: Vec::new(),
            last_synced_at: None,
        };

        self.write_payload(&payload)?;
        Ok(payload)
    }

    pub fn upsert_avatar(&self, avatar: AvatarSummary) -> Result<AvatarCachePayload, String> {
        let mut payload = self.load()?;
        let next_avatar = merge_cached_avatar(avatar, Some(&payload.avatars));

        if let Some(existing_avatar) = payload
            .avatars
            .iter_mut()
            .find(|existing_avatar| existing_avatar.id == next_avatar.id)
        {
            *existing_avatar = next_avatar;
        } else {
            payload.avatars.insert(0, next_avatar);
        }

        self.write_payload(&payload)?;
        Ok(payload)
    }

    fn write_payload(&self, payload: &AvatarCachePayload) -> Result<(), String> {
        fs::create_dir_all(&self.base_dir).map_err(|error| error.to_string())?;
        let json = serde_json::to_string_pretty(payload).map_err(|error| error.to_string())?;
        fs::write(&self.cache_file, json).map_err(|error| error.to_string())
    }

    async fn download_thumbnail(
        &self,
        client: &Client,
        auth_token: &str,
        avatar_id: &str,
        url: &str,
    ) -> Result<(String, i64), String> {
        let response = client
            .get(url)
            .header(USER_AGENT, app_user_agent())
            .header(COOKIE, format!("auth={auth_token}"))
            .send()
            .await
            .map_err(|error| error.to_string())?;

        if !response.status().is_success() {
            return Err(format!("Thumbnail download failed with status {}", response.status()));
        }

        let extension = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(content_type_to_extension)
            .unwrap_or("jpg");
        let bytes = response.bytes().await.map_err(|error| error.to_string())?;
        let path = self.thumbnail_dir.join(format!("{avatar_id}.{extension}"));

        fs::write(&path, bytes).map_err(|error| error.to_string())?;
        let version = chrono::Utc::now().timestamp_millis();

        Ok((path.to_string_lossy().to_string(), version))
    }
}

fn merge_cached_fields(
    avatars: Vec<AvatarSummary>,
    existing: Option<&[AvatarSummary]>,
) -> Vec<AvatarSummary> {
    avatars
        .into_iter()
        .map(|avatar| merge_cached_avatar(avatar, existing))
        .collect()
}

fn merge_cached_avatar(
    avatar: AvatarSummary,
    existing: Option<&[AvatarSummary]>,
) -> AvatarSummary {
    let existing_avatar = existing.and_then(|items| items.iter().find(|item| item.id == avatar.id));
    let tags = if avatar.tags.is_empty() {
        existing_avatar
            .map(|item| item.tags.clone())
            .unwrap_or_default()
    } else {
        avatar.tags.clone()
    };

    AvatarSummary {
        thumbnail_path: avatar
            .thumbnail_path
            .or_else(|| existing_avatar.and_then(|item| item.thumbnail_path.clone())),
        thumbnail_version: avatar
            .thumbnail_version
            .or_else(|| existing_avatar.and_then(|item| item.thumbnail_version)),
        tags,
        ..avatar
    }
}

fn content_type_to_extension(content_type: &str) -> &'static str {
    if content_type.contains("png") {
        "png"
    } else if content_type.contains("webp") {
        "webp"
    } else {
        "jpg"
    }
}
