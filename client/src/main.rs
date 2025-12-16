mod api;
mod audio;
mod config;
mod typing;

use config::Config;
use rdev::{listen, Event, EventType, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

static SUPER_PRESSED: AtomicBool = AtomicBool::new(false);
static C_PRESSED: AtomicBool = AtomicBool::new(false);
static RECORDING: AtomicBool = AtomicBool::new(false);

fn main() {
    println!("Voice Type - Starting...");

    let config = Config::load();
    println!("Config loaded from: {:?}", Config::config_path());
    println!("API URL: {}", config.api_url);
    println!("Hotkey: {}", config.hotkey);
    println!("Language: {}", config.language);
    println!("\nHold Super+C to record, release to transcribe and type.");
    println!("Press Ctrl+C to exit.\n");

    let config = Arc::new(config);
    let recorder = Arc::new(Mutex::new(audio::Recorder::new()));

    let config_clone = Arc::clone(&config);
    let recorder_clone = Arc::clone(&recorder);

    // Listen for keyboard events
    if let Err(e) = listen(move |event| {
        handle_event(event, &config_clone, &recorder_clone);
    }) {
        eprintln!("Failed to listen for events: {:?}", e);
    }
}

fn handle_event(event: Event, config: &Config, recorder: &Arc<Mutex<audio::Recorder>>) {
    match event.event_type {
        EventType::KeyPress(key) => {
            // Track Super/Meta key
            if matches!(key, Key::MetaLeft | Key::MetaRight) {
                SUPER_PRESSED.store(true, Ordering::SeqCst);
            }

            // Track C key
            if matches!(key, Key::KeyC) {
                C_PRESSED.store(true, Ordering::SeqCst);
            }

            // Start recording if both keys pressed
            if SUPER_PRESSED.load(Ordering::SeqCst)
                && C_PRESSED.load(Ordering::SeqCst)
                && !RECORDING.load(Ordering::SeqCst)
            {
                // Delete the 'c' that was typed, then start recording
                typing::delete_char();
                start_recording(recorder);
            }
        }
        EventType::KeyRelease(key) => {
            // Check if we should stop recording
            let was_recording = RECORDING.load(Ordering::SeqCst);

            // Update key states
            if matches!(key, Key::MetaLeft | Key::MetaRight) {
                SUPER_PRESSED.store(false, Ordering::SeqCst);
            }
            if matches!(key, Key::KeyC) {
                C_PRESSED.store(false, Ordering::SeqCst);
            }

            // Stop recording if either key released
            if was_recording
                && (!SUPER_PRESSED.load(Ordering::SeqCst) || !C_PRESSED.load(Ordering::SeqCst))
            {
                stop_and_transcribe(config, recorder);
            }
        }
        _ => {}
    }
}

fn start_recording(recorder: &Arc<Mutex<audio::Recorder>>) {
    print!("\r🎤 Recording... ");
    use std::io::Write;
    std::io::stdout().flush().ok();

    audio::mute();

    let mut rec = recorder.lock().unwrap();
    if let Err(e) = rec.start() {
        eprintln!("Failed to start recording: {}", e);
        audio::unmute();
        return;
    }

    RECORDING.store(true, Ordering::SeqCst);
}

fn stop_and_transcribe(config: &Config, recorder: &Arc<Mutex<audio::Recorder>>) {
    // Atomic check - only proceed if we're actually transitioning from recording to not recording
    if !RECORDING.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        return; // Already stopped by another key release
    }

    print!("🛑 Processing... ");
    use std::io::Write;
    std::io::stdout().flush().ok();

    let audio_data = {
        let mut rec = recorder.lock().unwrap();
        rec.stop()
    };

    audio::unmute();

    // Skip if too short (less than 0.5 seconds at 16kHz = 16000 samples = 32000 bytes)
    if audio_data.len() < 32000 {
        println!("(too short, skipped)");
        return;
    }

    print!("🧠 Transcribing... ");
    std::io::stdout().flush().ok();

    match api::transcribe(&config.api_url, &config.api_token, audio_data, &config.language) {
        Ok(text) => {
            if text.is_empty() {
                println!("(no speech detected)");
            } else {
                println!("⌨️  \"{}\"", text);
                typing::type_text(&text);
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
}
