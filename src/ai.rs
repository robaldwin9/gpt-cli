use std::error::Error;
use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ImageResponseFormat, ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};

pub enum OpenAiModel {
    Chatgpt4oLatest,
    Chatgpt4oMini,
    O3Mini,
}

impl OpenAiModel {
    fn as_str(&self) -> &str {
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

pub async fn send_image_request( image_count: u8, prompt:
 &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request = CreateImageRequestArgs::default()
        .prompt(prompt)
        .n(image_count)
        .response_format(ImageResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("gpt-cli")
        .build()?;

    let response = client.images().create(request).await?;

    let paths = response.save("./data").await?;
   paths.iter().for_each(|path| println!("Image file path: {}", path.display()));
    Ok(())
}


pub async fn send_chat_request(model: OpenAiModel, prompt: String) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_completion_tokens(512u32)
        .model(model.as_str())
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Give me a command to ping google")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("ping www.google.com")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Give me a command to list the contents of a file")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("cat <filename>")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ]).build()?;
        let response = client.chat().create(request).await?;
        for choice in response.choices {
            println!("\n{}", choice.message.content.unwrap_or("".to_string()));
        }
    
    Ok(())
}