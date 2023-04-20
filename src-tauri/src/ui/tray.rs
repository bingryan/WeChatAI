use tauri::{AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

use crate::core::{handle::Handle, window_manager::WindowType};

pub struct AppSystemTray;

impl AppSystemTray {
    pub fn menu(app_handle: &AppHandle) -> SystemTrayMenu {
        let version = app_handle.package_info().version.to_string();
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("open_window", "显示界面"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("app_version", format!("版本 {version}")).disabled())
            .add_item(CustomMenuItem::new("quit", "退出").accelerator("CmdOrControl+Q"))
    }
    pub fn update(app_handle: &AppHandle) -> anyhow::Result<()> {
        app_handle
            .tray_handle()
            .set_menu(AppSystemTray::menu(app_handle))?;
        Ok(())
    }
    pub fn event(app_handle: &AppHandle, event: SystemTrayEvent) {
        match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "open_window" => {
                    Handle::open_window(WindowType::Main);
                }
                "quit" => {
                    app_handle.exit(0);
                    std::process::exit(0);
                }
                _ => {}
            },
            // TODO: add windows
            _ => {}
        }
    }
}
