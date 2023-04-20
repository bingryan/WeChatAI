// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{App, Manager, SystemTray};

use crate::config::app::AppConfig;
use crate::core::sysopt::Sysopt;
use crate::server::http;
use crate::ui::AppSystemTray;

mod app;
mod cmd;
mod config;
mod core;
mod error;
mod server;
mod storage;
mod ui;

fn main() {
    let builder = tauri::Builder::default()
        .setup(init)
        .system_tray(SystemTray::new())
        .on_system_tray_event(AppSystemTray::event)
        .invoke_handler(tauri::generate_handler![
            cmd::update_global_chatgpt_config,
            cmd::update_chatgpt_config_by_id,
        ]);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, e| match e {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Exit => {
            app_handle.exit(0);
        }
        _ => {}
    });
}

fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)] // only include this code on debug builds
    {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
        window.close_devtools();
    }
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Regular);

    AppConfig::init()?;
    core::handle::Handle::global().init(app.app_handle());
    Sysopt::global().init_launch()?;
    AppSystemTray::update(&app.app_handle())?;
    let _ = core::handle::Handle::refresh_global_shortcut();

    tauri::async_runtime::spawn(http::serve());
    Ok(())
}
