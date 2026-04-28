mod auth;
mod cache;
mod commands;
mod models;
mod osc;
mod settings;
mod vrchat;

use std::process::Command;
use tauri::{
    menu::{MenuBuilder, SubmenuBuilder},
};

const MENU_ID_OPEN_THUMBNAIL_CACHE_FOLDER: &str = "open_thumbnail_cache_folder";

fn build_app_menu(app: &tauri::AppHandle) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    let file_menu = SubmenuBuilder::new(app, "File")
        .text(
            MENU_ID_OPEN_THUMBNAIL_CACHE_FOLDER,
            "Open Thumbnail Cache Folder",
        )
        .build()?;

    MenuBuilder::new(app).item(&file_menu).build()
}

fn open_thumbnail_cache_folder() -> Result<(), String> {
    let thumbnail_dir = cache::AvatarCache::new()?.thumbnail_dir_path();

    Command::new("explorer")
        .arg(thumbnail_dir)
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _credential_store = auth::CredentialStore::new().ok();
    let _vrchat_client = vrchat::VrchatClient::new();
    let _osc_client = osc::OscClient::new();
    let _avatar_cache = cache::AvatarCache::new();

    tauri::Builder::default()
        .setup(|app| {
            let menu = build_app_menu(&app.handle())?;
            app.set_menu(menu)?;
            Ok(())
        })
        .on_menu_event(|_app, event| {
            if event.id().0 == MENU_ID_OPEN_THUMBNAIL_CACHE_FOLDER {
                if let Err(error) = open_thumbnail_cache_folder() {
                    eprintln!("failed to open thumbnail cache folder: {error}");
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_saved_session,
            commands::save_session,
            commands::clear_saved_session,
            commands::load_switch_settings,
            commands::save_switch_settings,
            commands::login_vrchat,
            commands::submit_two_factor,
            commands::load_cached_avatar_list,
            commands::save_avatar_tags,
            commands::verify_session,
            commands::refresh_avatar_list,
            commands::refresh_latest_avatar_page,
            commands::cache_avatar_thumbnails,
            commands::refresh_avatar_detail,
            commands::upload_avatar_image,
            commands::switch_avatar,
            commands::set_avatar_eye_height,
            commands::open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
