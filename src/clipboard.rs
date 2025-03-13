use arboard::Clipboard;

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    clipboard.set_text(text).expect("Failed to copy to clipboard");
    Ok(())
}
