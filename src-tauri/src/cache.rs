use std::{collections::HashSet, fs, path::PathBuf};

use reqwest::{
    header::{CONTENT_TYPE, COOKIE, USER_AGENT},
    Client,
};

use crate::{
    models::{AvatarCachePayload, AvatarSummary},
    vrchat::app_user_agent,
};

const APP_DIR_NAME: &str = "AvatarChanger";
const CACHE_DIR_NAME: &str = "cache";
const THUMBNAIL_DIR_NAME: &str = "thumbnails";
const AVATAR_CACHE_FILE: &str = "avatars.json";

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
                avatars: Vec::new(),
                last_synced_at: None,
            });
        }

        let json = fs::read_to_string(&self.cache_file).map_err(|error| error.to_string())?;
        serde_json::from_str::<AvatarCachePayload>(&json).map_err(|error| error.to_string())
    }

    pub async fn store(
        &self,
        client: &Client,
        auth_token: &str,
        avatars: Vec<AvatarSummary>,
        last_synced_at: String,
    ) -> Result<AvatarCachePayload, String> {
        let payload = AvatarCachePayload {
            avatars: self.attach_thumbnails(client, auth_token, avatars, None).await,
            last_synced_at: Some(last_synced_at),
        };

        self.write_payload(&payload)?;
        Ok(payload)
    }

    pub async fn store_partial(
        &self,
        client: &Client,
        auth_token: &str,
        avatars: Vec<AvatarSummary>,
        last_synced_at: String,
    ) -> Result<AvatarCachePayload, String> {
        let existing_payload = self.load()?;
        let refreshed = self
            .attach_thumbnails(client, auth_token, avatars, Some(&existing_payload.avatars))
            .await;
        let refreshed_ids: HashSet<String> = refreshed.iter().map(|avatar| avatar.id.clone()).collect();
        let mut merged = refreshed;

        merged.extend(
            existing_payload
                .avatars
                .into_iter()
                .filter(|avatar| !refreshed_ids.contains(&avatar.id)),
        );

        let payload = AvatarCachePayload {
            avatars: merged,
            last_synced_at: Some(last_synced_at),
        };

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

    async fn attach_thumbnails(
        &self,
        client: &Client,
        auth_token: &str,
        avatars: Vec<AvatarSummary>,
        existing: Option<&[AvatarSummary]>,
    ) -> Vec<AvatarSummary> {
        let mut cached = Vec::with_capacity(avatars.len());

        for avatar in avatars {
            let existing_avatar = existing.and_then(|items| items.iter().find(|item| item.id == avatar.id));
            let thumbnail_path = match &avatar.thumbnail_url {
                Some(url) if !url.is_empty() => self
                    .download_thumbnail(client, auth_token, &avatar.id, url)
                    .await
                    .ok()
                    .or_else(|| existing_avatar.and_then(|item| item.thumbnail_path.clone())),
                _ => existing_avatar.and_then(|item| item.thumbnail_path.clone()),
            };
            let tags = if avatar.tags.is_empty() {
                existing_avatar
                    .map(|item| item.tags.clone())
                    .unwrap_or_default()
            } else {
                avatar.tags.clone()
            };

            cached.push(AvatarSummary {
                thumbnail_path,
                tags,
                ..avatar
            });
        }

        cached
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
    ) -> Result<String, String> {
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

        Ok(path.to_string_lossy().to_string())
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
