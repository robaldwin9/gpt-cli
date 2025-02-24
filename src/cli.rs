use std::time::Duration;
use tokio::io::AsyncReadExt;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	#[clap(long, short, action)]
	pub image: bool,
    pub prompt: String,
}

pub async fn read_from_stdin_timeout(timeout: Duration) -> String {
    let mut buf = String::new();
    match tokio::time::timeout(timeout, tokio::io::stdin().read_to_string(&mut buf)).await {
    	_ => buf
    }
}