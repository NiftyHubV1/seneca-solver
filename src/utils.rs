use copypasta::{ClipboardContext, ClipboardProvider};
use rand::Rng;

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