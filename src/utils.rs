use chrono::DateTime;
use copypasta::{ClipboardContext, ClipboardProvider};
use inquire::InquireError;
use rand::Rng;
use serde_json::Value;
use std::error::Error;

pub fn read_clipboard() -> String {
    let mut clipboard_context = ClipboardContext::new().unwrap();
    clipboard_context.get_contents().unwrap_or("".to_string())
}

// Generates a 12-character hexadecimal string
pub fn generate_hex_string(half_length: i32) -> String {
    let mut rng = rand::thread_rng();
    (0..half_length)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect::<String>()
}

pub fn generate_assignment_string(
    assignment: &Value,
    longest_assignment_length: usize,
    longest_status_length: usize,
) -> String {
    let name = assignment["name"].as_str().unwrap();
    let due = DateTime::parse_from_rfc3339(assignment["dueDate"].as_str().unwrap())
        .unwrap()
        .to_rfc2822();

    // Remove timezone
    let due = &due[..due.len() - 6];

    // Get status
    let status = assignment["status"].as_str().unwrap();

    // Create padding strings
    let padding_assignment = " ".repeat(longest_assignment_length - name.len());
    let padding_status = " ".repeat(longest_status_length - status.len());

    format!(
        "{}{}   {}{}  Due: {} (UTC)",
        name, padding_assignment, status, padding_status, due
    )
}

pub fn input_or_clipboard(
    user_input: Result<String, InquireError>,
) -> Result<String, Box<dyn Error>> {
    let input = if let Ok(input) = user_input {
        input
    } else {
        println!("");
        return Err("Failed to read input".into());
    };

    // Read input from clipboard if none was entered, then trim whitespace
    Ok(if input.is_empty() {
        read_clipboard()
    } else {
        input
    }
    .trim()
    .to_string())
}
