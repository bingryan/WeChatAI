use chatgpt::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::Result;
#[derive(Deserialize, Clone, Debug)]
pub struct CreateNewChat {
    pub history: Option<Vec<ChatMessage>>,
    pub config: Option<ChatGPTModelConfiguration>,
}

impl CreateNewChat {
    pub fn new(
        history: Option<Vec<ChatMessage>>,
        config: Option<ChatGPTModelConfiguration>,
    ) -> Self {
        Self { history, config }
    }

    pub fn get_model_config(&self) -> Result<(String, ModelConfiguration)> {
        let config = match self.config.clone() {
            Some(config) => config,
            None => ChatGPTModelConfiguration::default(),
        };

        let model_config = ModelConfigurationBuilder::default()
            .api_url(url::Url::parse(&config.api_url)?)
            .engine(ChatGPTEngine::from_str(&config.engine)?)
            .temperature(config.to_owned().temperature.parse::<f32>()?)
            .top_p(config.to_owned().top_p.parse::<f32>()?)
            .presence_penalty(config.to_owned().presence_penalty.parse::<f32>()?)
            .frequency_penalty(config.to_owned().frequency_penalty.parse::<f32>()?)
            .reply_count(config.to_owned().reply_count.parse::<u32>()?)
            .build()
            .map_err(|_| anyhow::anyhow!("Invalid model configuration"))?;
        Ok((config.openai_key, model_config))
    }

    pub fn get_messages(&self) -> Vec<ChatMessage> {
        let msg = match self.history.clone() {
            Some(messages) => messages,
            None => Vec::new(),
        };
        msg
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChatGPTModelConfiguration {
    #[serde(default = "default_openai_key")]
    pub openai_key: String,
    /// The GPT version used.
    #[serde(default = "default_engine")]
    pub engine: String,
    /// Controls randomness of the output. Higher valeus means more random
    #[serde(default = "default_temperature")]
    pub temperature: String,
    /// Controls diversity via nucleus sampling, not recommended to use with temperature
    #[serde(default = "default_top_p")]
    pub top_p: String,
    /// Determines how much to penalize new tokens pased on their existing presence so far
    #[serde(default = "default_presence_penalty")]
    pub presence_penalty: String,
    /// Determines how much to penalize new tokens based on their existing frequency so far
    #[serde(default = "default_frequency_penalty")]
    pub frequency_penalty: String,
    /// The maximum amount of replies
    #[serde(default = "default_reply_count")]
    pub reply_count: String,
    /// URL of the /v1/chat/completions endpoint. Can be used to set a proxy
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

impl Default for ChatGPTModelConfiguration {
    fn default() -> Self {
        Self {
            openai_key: default_openai_key(),
            engine: default_engine(),
            temperature: default_temperature(),
            top_p: default_top_p(),
            presence_penalty: default_presence_penalty(),
            frequency_penalty: default_frequency_penalty(),
            reply_count: default_reply_count(),
            api_url: default_api_url(),
        }
    }
}

fn default_openai_key() -> String {
    "".to_string()
}

fn default_engine() -> String {
    "gpt-3.5-turbo".to_string()
}

fn default_temperature() -> String {
    "0.5".to_string()
}

fn default_top_p() -> String {
    "1.0".to_string()
}

fn default_presence_penalty() -> String {
    "0.0".to_string()
}

fn default_frequency_penalty() -> String {
    "0.0".to_string()
}

fn default_reply_count() -> String {
    "1".to_string()
}

fn default_api_url() -> String {
    "https://api.openai.com/v1/chat/completions".to_string()
}
