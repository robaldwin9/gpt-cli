
use clap::Parser;
use std::time::Duration;
mod cli;
use cli::Args;
use std::process;

mod openai;
use openai::send_image_request;
use openai::send_chat_stream_request;
use openai::model::OpenAiModel;

#[tokio::main]
async fn main() {
    let mut prompt = cli::read_from_stdin_timeout(Duration::from_millis(1000)).await;
    let args = Args::parse();
    if args.image {
         match send_image_request(1, &args.prompt).await {
            Ok(()) => { process::exit(0) },
            Err(e) => { println!("Error occured when requesting image generation from API: {}", e )}
         }
    } else {
        prompt.push_str(&args.prompt);
        match send_chat_stream_request(OpenAiModel::O3Mini, prompt).await {
            Ok(()) => { process::exit(0) },
            Err(e) => { println!("Error occured when requesting chat response from the API: {}", e )}
        }
    }
}
