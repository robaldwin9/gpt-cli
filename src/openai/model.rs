pub enum OpenAiModel {
    Chatgpt4oLatest,
    Chatgpt4oMini,
    O3Mini
}



impl OpenAiModel {
    pub fn as_str(&self) -> &str {
        match self {
            OpenAiModel::Chatgpt4oLatest => "chatgpt-4o-latest",
            OpenAiModel::Chatgpt4oMini => "gpt-4o-mini",
            OpenAiModel::O3Mini => "o3-mini"
        }
    }
}

impl PartialEq for OpenAiModel {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}