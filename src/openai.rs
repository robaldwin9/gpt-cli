use async_openai::{
    types::{CreateImageRequestArgs,
            ImageResponseFormat, ImageSize}, Client};
use std::error::Error;
use std::io::{stdout, Write};
use tokio_stream::StreamExt;

pub mod model;
use model::OpenAiModel;

mod messages;
use messages::Messages;

pub async fn send_image_request(image_count: u8, prompt:&str) -> Result<(), Box<dyn Error>> {
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

pub async fn send_chat_stream_request(model: OpenAiModel, prompt: String, max_tokens: u32) -> Result<(), Box<dyn Error>> {
    let mut chat_history = Messages::init_load_push(prompt)?;
    let request = chat_history.init_request(model, max_tokens)?;

    let client = Client::new();
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

    chat_history.push_then_save(response_string)?;
    Ok(())
}

pub async fn send_chat_request(model: OpenAiModel, prompt: String, max_tokens: u32) -> Result<(), Box<dyn Error>> {
    let mut chat_history = Messages::init_load_push(prompt)?;
    let request = chat_history.init_request(model, max_tokens)?;

    let client = Client::new();
    let response = client.chat().create(request).await?;
    let mut response_string = String::from("");
    for choice in response.choices {
        response_string += &("\n".to_owned() +  &choice.message.content.unwrap_or("".to_string()));
    }

    println!("{}", response_string);
    chat_history.push_then_save(response_string)?;
    Ok(())
}