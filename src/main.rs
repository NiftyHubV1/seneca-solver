mod seneca_client;
mod utils;

use seneca_client::SenecaClient;
use utils::read_clipboard;

use inquire::{Password, PasswordDisplayMode};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Prompt the user for access key
    let entered_key = Password::new("Enter your access key (leave empty to use clipboard):")
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Hidden)
        .prompt()
        .unwrap_or("".to_string());

    // Read access key from clipboard if none was entered, then trim whitespace
    let access_key = if entered_key.is_empty() {
        println!("No access key provided, attempting to get from clipboard");
        read_clipboard()
    } else {
        entered_key
    }
    .trim()
    .to_string();

    let client = SenecaClient::new(access_key);
    println!("{}", client.get_user_id().await?);
    // println!("nice key buddy! {}", access_key);
    // reqtest().await?;
    Ok(())
}
