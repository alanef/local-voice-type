use reqwest::blocking::multipart;

pub fn transcribe(api_url: &str, api_token: &str, audio_data: Vec<u8>, language: &str) -> Result<String, String> {
    let url = format!("{}/v1/transcribe", api_url);

    let part = multipart::Part::bytes(audio_data)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .map_err(|e| format!("Failed to create multipart: {}", e))?;

    let form = multipart::Form::new()
        .part("file", part)
        .text("language", language.to_string());

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(api_token)
        .multipart(form)
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("Server error {}: {}", status, body));
    }

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    json["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "No text in response".to_string())
}
