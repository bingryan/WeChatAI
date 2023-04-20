use anyhow::anyhow;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::utils::platform::current_exe;

use crate::{config::app::AppConfig, storage::KVStorage};

pub struct Sysopt {
    auto_launch: Arc<Mutex<Option<AutoLaunch>>>,
}

impl Sysopt {
    pub fn global() -> &'static Sysopt {
        static SYSOPT: OnceCell<Sysopt> = OnceCell::new();

        SYSOPT.get_or_init(|| Sysopt {
            auto_launch: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init_launch(&self) -> anyhow::Result<()> {
        let mut enable = false;
        if let Some(auto_launch_conf) = AppConfig::settings().lock().query("auto_launch")? {
            enable = auto_launch_conf
                .as_str()
                .map(|s| s == "true")
                .unwrap_or(false)
        }

        let app_exe = current_exe()?;
        let app_exe = dunce::canonicalize(app_exe)?;
        let app_name = app_exe
            .file_stem()
            .and_then(|f| f.to_str())
            .ok_or(anyhow!("failed to get file stem"))?;
        let app_path = app_exe
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("failed to get app_path"))?
            .to_string();
        #[cfg(target_os = "windows")]
        let app_path = format!("\"{app_path}\"");

        #[cfg(target_os = "macos")]
        let app_path = (|| -> Option<String> {
            let path = std::path::PathBuf::from(&app_path);
            let path = path.parent()?.parent()?.parent()?;
            let extension = path.extension()?.to_str()?;
            match extension == "app" {
                true => Some(path.as_os_str().to_str()?.to_string()),
                false => None,
            }
        })()
        .unwrap_or(app_path);

        let auto = AutoLaunchBuilder::new()
            .set_app_name(app_name)
            .set_app_path(&app_path)
            .build()?;

        if let Ok(true) = auto.is_enabled() {
            if enable {
                return Ok(());
            }
        }

        #[cfg(target_os = "macos")]
        let _ = auto.disable();

        if enable {
            auto.enable()?;
        }
        *self.auto_launch.lock() = Some(auto);

        Ok(())
    }
}
