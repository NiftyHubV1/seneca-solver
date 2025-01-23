use copypasta::{ClipboardContext, ClipboardProvider};

pub fn read_clipboard() -> String {
    let mut clipboard_context = ClipboardContext::new().unwrap();
    clipboard_context.get_contents().unwrap_or("".to_string())
}