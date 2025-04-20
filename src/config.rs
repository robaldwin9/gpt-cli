use std::{env, io};
use std::fs::{create_dir, OpenOptions};
use std::io::{BufReader, Write};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::openai::model::OpenAiModel;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub open_ai_model: String,
    pub system_message: String,
    pub open_ai_max_tokens: u32,
    pub open_ai_stream_chat: bool,
    pub stdin_read_time: u64,
}

impl Config {
    pub fn save(&self) {
        match get_config_file_path() {
            Ok(path) => {
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                {
                    Ok(mut file) => {
                        let serialized_messages_list =
                            serde_json::to_string(&self).expect("invalid json");
                        file.write_all(serialized_messages_list.as_bytes())
                            .expect("Failed to save config file");
                    }
                    Err(e) => {
                        println!("Error opening config file for save operation: {}", e);
                    }
                };
            }

            Err(e) => {
                println!(
                    "Error could not access config file path, or create data directory: {}",
                    e
                )
            }
        };
    }

    pub fn load(&mut self) -> Config {
        let mut config = Config::new();
        match get_config_file_path() {
            Ok(path) => {
                if path.exists() {
                    match OpenOptions::new().read(true).open(path) {
                        Ok(file) => {
                            let metadata = file.metadata().expect("could not get config file metadata");
                            if metadata.len() != 0 {
                                let reader = BufReader::new(file);
                                match serde_yaml::from_reader(reader) {
                                    Ok(loaded_config) => {
                                        config = loaded_config;
                                    },
                                    Err(e) => {
                                        println!("Error loading config file {}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error loading config file {}", e);
                        }
                    };
                } else {
                    config.save();
                }
            }

            Err(e) => {
                println!(
                    "Error could not access config file path, or create data directory {}",
                    e
                )
            }
        };
        
        config
    }
    
    pub fn new () -> Self {
        Config  {open_ai_model: String::from(OpenAiModel::Gpt4o.as_str()),
            open_ai_max_tokens: 2000, open_ai_stream_chat: true, stdin_read_time: 2000,
        system_message: String::from("you are a helpful CLI assistant, \
        all your answers will be output to the terminal. \
        Responses should be formatted so they are easy to read".to_string())}
    }
}

fn get_config_file_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("data");

    if !dir.as_path().exists() {
        create_dir(dir.as_path())?;
    }

    dir.push("config.yaml");
    Ok(dir)
}