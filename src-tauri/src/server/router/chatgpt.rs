use axum::{response::IntoResponse, Json};
use chatgpt::prelude::*;

use crate::error::Result;
use crate::server::extractor::StreamBodyResponse;
use crate::server::helper::chatgpt::CreateNewChat;

// for developer test
pub async fn create_chat_json(Json(payload): Json<CreateNewChat>) -> Result<impl IntoResponse> {
    let (api_key, model_config) = payload.get_model_config()?;

    let client = ChatGPT::new_with_config(api_key, model_config)?;

    let stream = client
        .send_history_streaming(&payload.get_messages())
        .await?;
    Ok(StreamBodyResponse::json(stream))
}

pub async fn create_chat_raw(Json(payload): Json<CreateNewChat>) -> Result<impl IntoResponse> {
    let (api_key, model_config) = payload.get_model_config()?;

    let client = ChatGPT::new_with_config(api_key, model_config)?;
    let stream = client
        .send_history_streaming(&payload.get_messages())
        .await?;
    Ok(StreamBodyResponse::raw(stream))
}
