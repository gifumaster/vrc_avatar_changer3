use std::{fs, path::PathBuf};

use crate::models::{AvatarSwitchMethod, AvatarSwitchSettings, OscSettings};

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

    pub fn load_switch_settings(&self) -> Result<AvatarSwitchSettings, String> {
        if !self.path.exists() {
            return Ok(AvatarSwitchSettings::default());
        }

        let json = fs::read_to_string(&self.path).map_err(|error| error.to_string())?;
        let settings = serde_json::from_str::<serde_json::Value>(&json).map_err(|error| error.to_string())?;
        Ok(normalize_switch_settings(parse_switch_settings(settings)?))
    }

    pub fn save_switch_settings(
        &self,
        settings: &AvatarSwitchSettings,
    ) -> Result<AvatarSwitchSettings, String> {
        let normalized = normalize_switch_settings(settings.clone());
        let json = serde_json::to_string_pretty(&normalized).map_err(|error| error.to_string())?;
        fs::write(&self.path, json).map_err(|error| error.to_string())?;
        Ok(normalized)
    }
}

fn parse_switch_settings(value: serde_json::Value) -> Result<AvatarSwitchSettings, String> {
    if value.get("osc").is_some() || value.get("method").is_some() {
        return serde_json::from_value::<AvatarSwitchSettings>(value).map_err(|error| error.to_string());
    }

    let osc = serde_json::from_value::<OscSettings>(value).map_err(|error| error.to_string())?;
    Ok(AvatarSwitchSettings {
        method: AvatarSwitchMethod::Osc,
        osc,
    })
}

fn normalize_switch_settings(settings: AvatarSwitchSettings) -> AvatarSwitchSettings {
    AvatarSwitchSettings {
        method: settings.method,
        osc: normalize_osc_settings(settings.osc),
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
