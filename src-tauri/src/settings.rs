use std::{fs, path::PathBuf};

use crate::models::OscSettings;

const APP_DIR_NAME: &str = "AvatarChanger";
const SETTINGS_FILE: &str = "settings.json";

pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn new() -> Result<Self, String> {
        let path = dirs::data_local_dir()
            .ok_or_else(|| "Could not resolve LocalAppData directory".to_string())?
            .join(APP_DIR_NAME)
            .join(SETTINGS_FILE);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }

        Ok(Self { path })
    }

    pub fn load_osc_settings(&self) -> Result<OscSettings, String> {
        if !self.path.exists() {
            return Ok(OscSettings::default());
        }

        let json = fs::read_to_string(&self.path).map_err(|error| error.to_string())?;
        let settings = serde_json::from_str::<OscSettings>(&json).map_err(|error| error.to_string())?;
        Ok(normalize_osc_settings(settings))
    }

    pub fn save_osc_settings(&self, settings: &OscSettings) -> Result<OscSettings, String> {
        let normalized = normalize_osc_settings(settings.clone());
        let json = serde_json::to_string_pretty(&normalized).map_err(|error| error.to_string())?;
        fs::write(&self.path, json).map_err(|error| error.to_string())?;
        Ok(normalized)
    }
}

fn normalize_osc_settings(settings: OscSettings) -> OscSettings {
    let defaults = OscSettings::default();
    let host = if settings.host.trim().is_empty() {
        defaults.host
    } else {
        settings.host.trim().to_string()
    };
    let port = if settings.port == 0 {
        defaults.port
    } else {
        settings.port
    };

    OscSettings {
        enabled: settings.enabled,
        host,
        port,
    }
}
