
use serde::{Serialize, Deserialize};
#[derive(Deserialize, Serialize)]
pub enum OpenAiModel {
    Chatgpt4oLatest, Chatgpt4oMini,
    Gpt4o, O3Mini
}

impl OpenAiModel {
    pub fn as_str(&self) -> &str {
        match self {
            OpenAiModel::Chatgpt4oLatest => "chatgpt-4o-latest",
            OpenAiModel::Chatgpt4oMini => "gpt-4o-mini",
            OpenAiModel::Gpt4o => "gpt-4o",
            OpenAiModel::O3Mini => "o3-mini"
        }
    }

    pub fn default() -> Self {
        OpenAiModel::O3Mini
    }

    pub fn from_str(model: &str) -> Self {
        match model {
            "chatgpt-4o-latest" => OpenAiModel::Chatgpt4oLatest,
            "gpt-4o-minigpt-4o-mini" => OpenAiModel::Chatgpt4oMini,
            "o3-mini" => OpenAiModel::O3Mini,
            "gpt-4o" => OpenAiModel::Gpt4o,
            _ => OpenAiModel::default()

        }
    }
}

impl PartialEq for OpenAiModel {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}