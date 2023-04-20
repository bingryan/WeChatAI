use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::path::PathBuf;
use std::sync::Arc;

use crate::error::Result;
use crate::storage::{KVStorage, LocalJsonStorage};
pub struct AppConfig<T: KVStorage> {
    setting: Arc<Mutex<T>>,
    chatgpt: Arc<Mutex<T>>,
}

impl AppConfig<LocalJsonStorage> {
    /// init
    pub fn init() -> anyhow::Result<()> {
        /*
        ~/.WeChatAI
            ├── config
            │   ├── setting.json
            │   ├── chatgpt.json # chatgpt model settings for global
                ├── chatgpt # chatgpt model settings for single chat
                │   ├── [id].json
            ├── database
                ├── chatgpt
                │   ├── cache.json
            ├── upload
                ├── image
                │   ├── a.jpg
         */

        Self::generate_default_config(false)?;
        Ok(())
    }

    pub fn global() -> &'static AppConfig<LocalJsonStorage> {
        static CONFIG: OnceCell<AppConfig<LocalJsonStorage>> = OnceCell::new();

        CONFIG.get_or_init(|| AppConfig {
            // TODO: marco
            setting: Arc::new(Mutex::new(
                LocalJsonStorage::new(setting_config_path().unwrap()).unwrap(),
            )),
            chatgpt: Arc::new(Mutex::new(
                LocalJsonStorage::new(chatgpt_config_path().unwrap()).unwrap(),
            )),
        })
    }

    pub fn settings() -> Arc<Mutex<LocalJsonStorage>> {
        Self::global().setting.clone()
    }

    pub fn chatgpt() -> Arc<Mutex<LocalJsonStorage>> {
        Self::global().chatgpt.clone()
    }

    pub fn get_chatgpt_config(id: &str) -> Result<LocalJsonStorage> {
        let mut path = chatgpt_setting_path()?;
        path.push(id);
        path.set_extension("json");
        Ok(LocalJsonStorage::new(path)?)
    }

    pub fn update_chatgpt_config(id: &str, content: Option<String>) -> Result<LocalJsonStorage> {
        let mut path = chatgpt_setting_path()?;
        path.push(id);
        path.set_extension("json");
        Ok(LocalJsonStorage::create(path, content, true)?)
    }
    fn generate_default_config(force: bool) -> anyhow::Result<()> {
        let image_path = image_upload_path()?;
        let config_path = config_path()?;
        let setting_config_path = setting_config_path()?;
        let chatgpt_config_path = chatgpt_config_path()?;

        if !image_path.exists() {
            std::fs::create_dir_all(image_path)?;
        }
        if !config_path.exists() {
            std::fs::create_dir_all(config_path)?;
        }

        if force {
            std::fs::write(setting_config_path, SETTING)?;
            std::fs::write(chatgpt_config_path, CHATGPT)?;
            return Ok(());
        }

        if !setting_config_path.exists() {
            std::fs::write(setting_config_path, SETTING)?;
        }

        if !chatgpt_config_path.exists() {
            std::fs::write(chatgpt_config_path, CHATGPT)?;
        }

        Ok(())
    }
}

pub fn root_path() -> Result<PathBuf> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".WeChatAI");
    Ok(path)
}

pub fn config_path() -> Result<PathBuf> {
    let mut path = root_path()?;
    path.push("config");
    Ok(path)
}

pub fn upload_path() -> Result<PathBuf> {
    let mut path = root_path()?;
    path.push("upload");
    Ok(path)
}

pub fn image_upload_path() -> Result<PathBuf> {
    let mut path = upload_path()?;
    path.push("image");
    Ok(path)
}

pub fn setting_config_path() -> Result<PathBuf> {
    let mut path = config_path()?;
    path.push("setting.json");
    Ok(path)
}

pub fn chatgpt_config_path() -> Result<PathBuf> {
    let mut path = config_path()?;
    path.push("chatgpt.json");
    Ok(path)
}

pub fn chatgpt_setting_path() -> Result<PathBuf> {
    let mut path = config_path()?;
    path.push("chatgpt");
    Ok(path)
}

const SETTING: &str = r#"{
	"auto_launch": false,
	"shortcut": "CommandOrControl+Shift+K",
	"spellcheck": false
  }"#;
const CHATGPT: &str = r#"{
	"engine": "gpt-3.5-turbo",
	"temperature": 0.5,
	"top_p": 1.0,
	"presence_penalty": 0.0,
	"frequency_penalty": 0.0,
	"reply_count": 1,
	"api_url": "https://api.openai.com/v1/chat/completions"
  }"#;
