use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::io::Cursor;

const SAMPLE_RATE: u32 = 16000;

pub struct Recorder {
    samples: Arc<Mutex<Vec<i16>>>,
    stream: Option<cpal::Stream>,
}

impl Recorder {
    pub fn new() -> Self {
        Recorder {
            samples: Arc::new(Mutex::new(Vec::new())),
            stream: None,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device found")?;

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        // Clear previous samples
        self.samples.lock().unwrap().clear();

        let samples = Arc::clone(&self.samples);
        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut samples = samples.lock().unwrap();
                    for &sample in data {
                        // Convert f32 to i16
                        let s = (sample * 32767.0) as i16;
                        samples.push(s);
                    }
                },
                |err| eprintln!("Audio error: {}", err),
                None,
            )
            .map_err(|e| format!("Failed to build stream: {}", e))?;

        stream.play().map_err(|e| format!("Failed to start stream: {}", e))?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) -> Vec<u8> {
        // Drop the stream to stop recording
        self.stream = None;

        // Get samples
        let samples = self.samples.lock().unwrap().clone();

        // Convert to WAV bytes
        self.samples_to_wav(&samples)
    }

    fn samples_to_wav(&self, samples: &[i16]) -> Vec<u8> {
        let mut buffer = Cursor::new(Vec::new());

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::new(&mut buffer, spec).unwrap();
        for &sample in samples {
            writer.write_sample(sample).unwrap();
        }
        writer.finalize().unwrap();

        buffer.into_inner()
    }
}

// Platform-specific muting
#[cfg(target_os = "linux")]
pub fn mute() {
    let _ = std::process::Command::new("pactl")
        .args(["set-sink-mute", "@DEFAULT_SINK@", "1"])
        .output();
}

#[cfg(target_os = "linux")]
pub fn unmute() {
    let _ = std::process::Command::new("pactl")
        .args(["set-sink-mute", "@DEFAULT_SINK@", "0"])
        .output();
}

#[cfg(target_os = "macos")]
pub fn mute() {
    let _ = std::process::Command::new("osascript")
        .args(["-e", "set volume output muted true"])
        .output();
}

#[cfg(target_os = "macos")]
pub fn unmute() {
    let _ = std::process::Command::new("osascript")
        .args(["-e", "set volume output muted false"])
        .output();
}

#[cfg(target_os = "windows")]
pub fn mute() {
    // Windows mute via PowerShell (toggle)
    let _ = std::process::Command::new("powershell")
        .args(["-Command", "(New-Object -ComObject WScript.Shell).SendKeys([char]173)"])
        .output();
}

#[cfg(target_os = "windows")]
pub fn unmute() {
    // Same toggle key
    let _ = std::process::Command::new("powershell")
        .args(["-Command", "(New-Object -ComObject WScript.Shell).SendKeys([char]173)"])
        .output();
}
