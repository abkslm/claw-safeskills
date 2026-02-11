use base64::prelude::*;
use clap::Parser;
use keyring::Entry;
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The image prompt
    #[arg(short, long)]
    prompt: String,

    /// Optional filename for the output image
    #[arg(short, long)]
    filename: Option<String>,

    /// Optional resolution (1K, 2K, 4K)
    #[arg(short, long)]
    resolution: Option<String>,
}

fn map_resolution(res: Option<String>) -> String {
    match res.as_deref() {
        Some(r) => {
            let r_lower = r.to_lowercase();
            if r_lower.contains("4k") || r_lower.contains("ultra") || r_lower.contains("high") {
                "4K".to_string()
            } else if r_lower.contains("2k") || r_lower.contains("2048") || r_lower.contains("medium") || r_lower.contains("normal") {
                "2K".to_string()
            } else {
                "1K".to_string() // Default to 1K for "1K", "1080", "low", or unrecognized
            }
        }
        None => "1K".to_string(),
    }
}

fn main() {
    let args = Args::parse();
    let prompt = &args.prompt;
    let resolution = map_resolution(args.resolution);
    
    // Retrieve API Key
    let entry = Entry::new("nano-banana-skill", "API_KEY").unwrap_or_else(|e| {
        eprintln!("Error accessing keychain: {}", e);
        process::exit(1);
    });

    let api_key = match entry.get_password() {
        Ok(key) => key,
        Err(_) => {
            // Fallback to environment variable
            env::var("API_KEY").unwrap_or_else(|_| {
                eprintln!("Error: nano-banana-skill:API_KEY not found in keychain (service: nano-banana-skill, account: API_KEY) or environment variables.");
                process::exit(1);
            })
        }
    };

    let client = Client::new();
    let model = "gemini-3-pro-image-preview";
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let payload = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "imageConfig": {
                "imageSize": resolution
            }
        }
    });

    let response = client.post(&url)
        .json(&payload)
        .send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let text = resp.text().unwrap_or_default();
                let json_resp: Value = serde_json::from_str(&text).unwrap_or(json!({"error": "Invalid JSON response"}));
                
                // Check for image data in candidates
                if let Some(candidates) = json_resp.get("candidates").and_then(|c| c.as_array()) {
                    if let Some(first_candidate) = candidates.get(0) {
                        if let Some(content) = first_candidate.get("content") {
                            if let Some(parts) = content.get("parts").and_then(|p| p.as_array()) {
                                for part in parts {
                                    if let Some(inline_data) = part.get("inlineData") {
                                        if let Some(data_str) = inline_data.get("data").and_then(|d| d.as_str()) {
                                            let mime_type = inline_data.get("mimeType").and_then(|m| m.as_str()).unwrap_or("image/jpeg");
                                            let ext = if mime_type.contains("png") { "png" } else { "jpeg" };
                                            
                                            let filename = match &args.filename {
                                                Some(name) => name.clone(),
                                                None => {
                                                    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                                                    format!("image_{}.{}", timestamp, ext)
                                                }
                                            };
                                            
                                            match BASE64_STANDARD.decode(data_str) {
                                                Ok(decoded_data) => {
                                                    match File::create(&filename) {
                                                        Ok(mut file) => {
                                                            if let Err(e) = file.write_all(&decoded_data) {
                                                                eprintln!("Error writing file: {}", e);
                                                            } else {
                                                                println!("{}", json!({
                                                                    "status": "success",
                                                                    "message": "Image generated and saved.",
                                                                    "mime_type": mime_type,
                                                                    "resolution": resolution,
                                                                    "file_path": filename
                                                                }));
                                                                return;
                                                            }
                                                        },
                                                        Err(e) => eprintln!("Error creating file: {}", e)
                                                    }
                                                },
                                                Err(e) => eprintln!("Error decoding base64: {}", e)
                                            }
                                        }
                                    }
                                    if let Some(file_data) = part.get("fileData") {
                                         println!("{}", json!({
                                            "status": "success",
                                            "message": "Image generated (file).",
                                            "mime_type": file_data.get("mimeType"),
                                            "file_uri": file_data.get("fileUri")
                                        }));
                                        return;   
                                    }
                                }
                            }
                        }
                    }
                }

                // If no image found or different structure, print raw response (or text part)
                 println!("{}", json!({
                    "output": json_resp,
                }));
            } else {
                eprintln!("Error: API request failed with status: {}", resp.status());
                match resp.text() {
                    Ok(text) => eprintln!("Response: {}", text),
                    Err(_) => eprintln!("Response: (empty or error reading)"),
                }
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error performing request: {}", e);
            process::exit(1);
        }
    }
}
