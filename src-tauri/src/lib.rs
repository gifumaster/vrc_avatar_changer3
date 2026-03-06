mod auth;
mod cache;
mod commands;
mod models;
mod osc;
mod settings;
mod vrchat;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _credential_store = auth::CredentialStore::new().ok();
    let _vrchat_client = vrchat::VrchatClient::new();
    let _osc_client = osc::OscClient::new();
    let _avatar_cache = cache::AvatarCache::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::load_saved_session,
            commands::save_session,
            commands::clear_saved_session,
            commands::load_osc_settings,
            commands::save_osc_settings,
            commands::login_vrchat,
            commands::submit_two_factor,
            commands::load_cached_avatar_list,
            commands::save_avatar_tags,
            commands::verify_session,
            commands::refresh_avatar_list,
            commands::refresh_latest_avatar_page,
            commands::switch_avatar_via_osc,
            commands::open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
