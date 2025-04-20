
use clap::Parser;
use std::time::Duration;
mod cli;
use cli::Args;

mod openai;
use openai::send_image_request;
use openai::send_chat_stream_request;
use openai::send_chat_request;
use openai::model::OpenAiModel;

mod config;
use config::Config;
use crate::openai::clear_history;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config::new().load();
    let mut prompt = cli::read_from_stdin_timeout(Duration::from_millis(config.stdin_read_time)).await;
    
    // Clear chat history
    if args.reset {
        clear_history();
    }

    // image generation selected
    if args.image {
         match send_image_request(1, &args.prompt).await {
            Ok(()) => {},
            Err(e) => { println!("Error occurred when requesting image generation from API: {}", e )}
         }
    // chat request selected
    } else {
        prompt.push_str(&args.prompt);
        // stream means reply comes over in pieces result appears as if its being typed 
        if config.open_ai_stream_chat {
            match send_chat_stream_request(OpenAiModel::from_str(&config.open_ai_model), prompt, config.open_ai_max_tokens).await {
                Ok(()) => {
                },
                Err(e) => { println!("Error occurred when requesting chat response from the API: {}", e )}
            }
        } else {
              match send_chat_request(OpenAiModel::from_str(&config.open_ai_model), prompt, config.open_ai_max_tokens).await {
                Ok(()) => {
                },
                Err(e) => { println!("Error occurred when requesting chat response from the API: {}", e )}
            }   

        }
    }
}
