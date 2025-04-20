use std::time::Duration;
use tokio::io::AsyncReadExt;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	#[clap(long, short, action)]
	pub image: bool,

    #[clap(long, short, action)]
    pub reset: bool,
    
    pub prompt: String,
}

pub async fn read_from_stdin_timeout(timeout: Duration) -> String {
    let mut buf = String::new();
    match tokio::time::timeout(timeout, tokio::io::stdin().read_to_string(&mut buf)).await {
    	Ok(usize) => {
            match usize {
                Ok(bytes) => {
                    if bytes > 1000000 {
                        println!("{} bytes read from stdin os to large for api, they will be ignored", bytes);
                        String::new()
                    } else {
                        buf
                    }
                },
                Err(e) => {
                    println!("Error reading from stdin: {}", e);
                    buf
                }
            }
        },
        Err(_e) => {
            buf
        }
    }
}