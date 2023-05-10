use crate::app;
use crate::error::Result;
use crate::server::helper::chatgpt::ChatGPTModelConfiguration;
use serde_json::Value;

use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[tauri::command]
pub(crate) async fn update_global_chatgpt_config(config: ChatGPTModelConfiguration) -> Result<()> {
    app::update_global_chatgpt_config(&config).await
}

#[tauri::command]
pub(crate) async fn update_chatgpt_config_by_id(
    id: &str, config: ChatGPTModelConfiguration,
) -> Result<ChatGPTModelConfiguration> {
    app::update_chatgpt_config_by_id(id, &config).await
}

#[tauri::command]
pub(crate) async fn write_to_file(filename: String, content: String) -> Result<(), String> {
    let download_dir = dirs::download_dir().unwrap();
    let content_str: Value = serde_json::from_str(&content).map_err(|err| err.to_string())?;
    let content_json = serde_json::to_string_pretty(&content_str).map_err(|err| err.to_string())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(download_dir.join(&filename))
        .await
        .map_err(|err| err.to_string())?;

    file.write_all(content_json.as_bytes())
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}
