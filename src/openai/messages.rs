
use std::io::Write;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::env;
use std::fs::create_dir;
use serde::{Serialize, Deserialize};
use async_openai::{ types::{ChatCompletionRequestMessage}};
use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs};
use crate::openai::model::OpenAiModel;

#[derive(Deserialize, Serialize)]
pub struct Messages {
	pub messages: Vec<ChatCompletionRequestMessage>
}

impl Messages {

 pub fn save(&self) {
        match get_list_file_path() {
            Ok(path) => {
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                {
                    Ok(mut file) => {
                        let serialized_messages_list =
                            serde_json::to_string(&self.messages).expect("invalid json");
                        file.write_all(serialized_messages_list.as_bytes())
                            .expect("Failed to save open ai chat to file");
                    }
                    Err(e) => {
                        println!("Error opening open ai chat file for save operation: {}", e);
                    }
                };
            }

            Err(e) => {
                println!(
                    "Error could not access open ai chat file path, or create data directory: {}",
                    e
                )
            }
        };
    }
    
	pub fn load(&mut self) {
		match get_list_file_path() {
            Ok(path) => {
                if path.exists() {
                    match OpenOptions::new().read(true).open(path) {
                        Ok(file) => {
                            let metadata = file.metadata().expect("could not get file metadata");
                            if metadata.len() != 0 {
                                let reader = BufReader::new(file);
                                self.messages = serde_json::from_reader::<BufReader<File>, Vec<ChatCompletionRequestMessage>>(reader).expect("Badly formated json");
                            }
                        }
                        Err(e) => {
                            println!("Error loading open ai chat file, did you command list/remove before ever adding an item? {}", e);
                        }
                    };
                }
            }

            Err(e) => {
                println!(
                    "Error could not access open ai chat file path, or create data directory {}",
                    e
                )
            }
        }; 
	}

    pub fn push(&mut self, prompt: String) -> Result<(), OpenAIError> {
        let new_message = ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?;
        self.messages.push(ChatCompletionRequestMessage::from(new_message));
        Ok(())
    }
    pub fn new () -> Self {
        Messages {messages: Vec::new()}
    }

    pub fn push_then_save(&mut self, prompt: String) -> Result<(), OpenAIError> {
        self.push(prompt)?;
        self.save();
        Ok(())
    }

    pub fn load_then_push(&mut self, prompt: String) -> Result<(), OpenAIError> {
        self.load();
        self.push(prompt)
    }

    pub fn init_load_push(prompt: String) -> Result<Self, OpenAIError> {
        let mut messages = Messages::new();
        messages.load_then_push(prompt)?;
        Ok(messages)
    }

    pub fn init_request(&mut self, model: OpenAiModel, max_tokens: u32) -> Result<CreateChatCompletionRequest, OpenAIError> {
        CreateChatCompletionRequestArgs::default()
            .max_completion_tokens(max_tokens)
            .model(model.as_str())
            .messages(&*self.messages).build()
    }
}

fn get_list_file_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("data");

    if !dir.as_path().exists() {
        create_dir(dir.as_path())?;
    }

    dir.push("chat-history.json");
    Ok(dir)
}