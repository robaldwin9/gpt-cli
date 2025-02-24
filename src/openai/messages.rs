
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

#[derive(Deserialize, Serialize)]
pub struct Messages {
	pub messages: Vec<ChatCompletionRequestMessage>
}

impl Messages {
	fn size(&self) -> u32 {
		self.messages.len() as u32
	}


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
                        let serizlized_todo_list =
                            serde_json::to_string(&self.messages).expect("invalid json");
                        file.write_all(serizlized_todo_list.as_bytes())
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

    pub fn push(&mut self, message: ChatCompletionRequestMessage) {
        self.messages.push(message);
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