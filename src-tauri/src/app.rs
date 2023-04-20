use crate::config::app::AppConfig;
use crate::core::handle;
use crate::error::Result;
use crate::server::helper::chatgpt::ChatGPTModelConfiguration;
use crate::storage::KVStorage;

use std::collections::HashMap;

pub async fn update_global_chatgpt_config(config: &ChatGPTModelConfiguration) -> Result<()> {
    let json_str = serde_json::to_string(&config).unwrap();
    let conf: HashMap<String, serde_json::Value> = serde_json::from_str(&json_str).unwrap();
    let chatgpt_config = AppConfig::chatgpt();
    chatgpt_config.lock().batch_update(conf)?;

    handle::Handle::refresh_app()?;
    Ok(())
}

pub async fn update_chatgpt_config_by_id(
    id: &str,
    config: &ChatGPTModelConfiguration,
) -> Result<ChatGPTModelConfiguration> {
    let json_str = serde_json::to_string(&config).unwrap();

    let updated_config = AppConfig::update_chatgpt_config(id, Some(json_str))?;

    let data = updated_config.data;

    let res_str = serde_json::to_string(&data).unwrap();
    let chatgpt_config: ChatGPTModelConfiguration = serde_json::from_str(&res_str).unwrap();

    handle::Handle::refresh_app()?;
    Ok(chatgpt_config)
}
