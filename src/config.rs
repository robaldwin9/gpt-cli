use serde::{Serialize, Deserialize};
use crate::openai::model::OpenAiModel;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub open_ai_model: String,
    pub open_ai_max_tokens: u32,
    pub open_ai_stream_chat: bool,
    pub stdin_read_time: u64,
}

impl Config {
    pub fn save(&self) {
        // TODO: implement
    }

    pub fn load(&mut self) {
        // TODO: implement
    }

    pub fn new () -> Self {
        Config  {open_ai_model: String::from(OpenAiModel::O3Mini.as_str()),
            open_ai_max_tokens: 1000, open_ai_stream_chat: true, stdin_read_time: 2000}
    }
}