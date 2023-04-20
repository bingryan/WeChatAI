use crate::app;
use crate::error::Result;
use crate::server::helper::chatgpt::ChatGPTModelConfiguration;

#[tauri::command]
pub(crate) async fn update_global_chatgpt_config(config: ChatGPTModelConfiguration) -> Result<()> {
    app::update_global_chatgpt_config(&config).await
}

#[tauri::command]
pub(crate) async fn update_chatgpt_config_by_id(
    id: &str,
    config: ChatGPTModelConfiguration,
) -> Result<ChatGPTModelConfiguration> {
    app::update_chatgpt_config_by_id(id, &config).await
}
