use enigo::{Enigo, Key, Keyboard, Settings};
use std::thread;
use std::time::Duration;

pub fn type_text(text: &str) {
    // Small delay before typing
    thread::sleep(Duration::from_millis(100));

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Type the text
    if let Err(e) = enigo.text(text) {
        eprintln!("Failed to type: {}", e);
    }
}

pub fn delete_char() {
    // No delay - the 'c' is already typed by the time we get the event
    if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
        let _ = enigo.key(Key::Backspace, enigo::Direction::Click);
    }
}
