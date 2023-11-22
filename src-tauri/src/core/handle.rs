use anyhow::{bail, Result};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::{AppHandle, GlobalShortcutManager, Manager, Window};

use crate::core::window_manager::{WindowInfo, WindowType};
use crate::{config::app::AppConfig, storage::KVStorage};

#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<Mutex<Option<AppHandle>>>,
}

impl Handle {
    pub fn global() -> &'static Handle {
        static HANDLE: OnceCell<Handle> = OnceCell::new();

        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init(&self, app_handle: AppHandle) {
        *self.app_handle.lock() = Some(app_handle);
    }

    pub fn main_window(&self) -> Option<Window> {
        self.app_handle
            .lock()
            .as_ref()
            .map_or(None, |a| a.get_window("main"))
    }

    fn get_manager(&self) -> Result<impl GlobalShortcutManager> {
        let app_handle = self.app_handle.lock();
        if app_handle.is_none() {
            bail!("failed to get the hotkey manager");
        }
        Ok(app_handle.as_ref().unwrap().global_shortcut_manager())
    }

    pub fn refresh_app() -> Result<()> {
        if let Some(window) = Self::global().main_window() {
            window.emit("WeChatAI://refresh-app", "yes")?;
        }
        Ok(())
    }
    pub fn refresh_config() -> Result<()> {
        if let Some(window) = Self::global().main_window() {
            window.emit("WeChatAI://refresh-config", "yes")?;
        }
        Ok(())
    }
    pub fn refresh_shortcut() -> Result<()> {
        if let Some(window) = Self::global().main_window() {
            window.emit("WeChatAI://refresh-shortcut", "yes")?;
        }
        Ok(())
    }
    pub fn refresh_global_shortcut() -> Result<()> {
        let mut shortcut_manager = Self::global().get_manager()?;
        let _ = shortcut_manager.unregister_all();
        if let Some(shortcut_conf) = AppConfig::settings().lock().query("shortcut")? {
            let accelerator = shortcut_conf.as_str().unwrap_or("CommandOrControl+Shift+K");
            if !shortcut_manager.is_registered(accelerator)? {
                let _ =
                    shortcut_manager.register(accelerator, || Self::open_window(WindowType::Main));
            };
        }
        Ok(())
    }

    pub fn open_window(window_type: WindowType) {
        let binding = Self::global().app_handle.lock();
        let app_handle = binding.as_ref().unwrap();

        let window_info = match window_type {
            WindowType::Config => WindowInfo::config(),
            WindowType::Main => WindowInfo::main(),
        };

        let label = window_info.label.as_str();
        let title = window_info.title.as_str();
        let url = window_info.url.as_str();

        if let Some(window) = app_handle.get_window(label) {
            // let _ = window.set_always_on_top(true);
            let _ = window.show();
            let _ = window.set_focus();
            return;
        }

        let new_window = tauri::window::WindowBuilder::new(
            app_handle,
            label.to_string(),
            tauri::WindowUrl::App(url.into()),
        )
        .title(title)
        .center()
        .visible(true)
        .maximized(window_info.maximized)
        .resizable(window_info.resizable)
        .fullscreen(window_info.fullscreenable)
        .always_on_top(window_info.always_on_top)
        .inner_size(window_info.width, window_info.height)
        .transparent(window_info.transparent)
        .decorations(window_info.decorations)
        .skip_taskbar(window_info.skip_taskbar)
        // .hidden_title(window_info.hidden_title)
        .center()
        .build();
        match new_window {
            Ok(window) => {
                let _ = window.show();
            }
            Err(e) => {
                println!("create_window error: {}", e);
            }
        }
    }
}
