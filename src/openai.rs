
use std::error::Error;
use std::io::{stdout, Write};
use tokio_stream::StreamExt;
use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ImageResponseFormat, ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};

pub mod model;
use model::OpenAiModel;

mod messages;
use messages::Messages;



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

pub async fn send_chat_stream_request(model: OpenAiModel, prompt: String) -> Result<(), Box<dyn Error>> {
    let mut chat_history = Messages {messages: Vec::new()};
    chat_history.load();
    let client = Client::new();

    let new_message = ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into();
    chat_history.push(new_message);

    let request = CreateChatCompletionRequestArgs::default()
        .max_completion_tokens(1000u32)
        .model(model.as_str())
        .messages(chat_history.messages.clone()).build()?;


    let mut response_string = String::from("");
    let mut stream = client.chat().create_stream(request).await?;
    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        write!(lock, "{}", content).unwrap();
                        response_string += content;
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }


    let ai_response = ChatCompletionRequestAssistantMessageArgs::default().content(response_string).build()?.into();

    chat_history.push(ai_response);
    chat_history.save();
    
    Ok(())
}

pub async fn send_chat_request(model: OpenAiModel, prompt: String) -> Result<(), Box<dyn Error>> {
    let mut chat_history = Messages {messages: Vec::new()};
    chat_history.load();
    let client = Client::new();

    let new_message = ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into();
    chat_history.push(new_message);

    let request = CreateChatCompletionRequestArgs::default()
        .max_completion_tokens(1000u32)
        .model(model.as_str())
        .messages(chat_history.messages.clone()).build()?;
    

    let response = client.chat().create(request).await?;
    let mut response_string = String::from("");
    for choice in response.choices {
        response_string += &("\n".to_owned() +  &choice.message.content.unwrap_or("".to_string()));
    }

    println!("{}", response_string);
    let ai_response = ChatCompletionRequestAssistantMessageArgs::default().content(response_string).build()?.into();

    chat_history.push(ai_response);
    chat_history.save();
    
    Ok(())
}