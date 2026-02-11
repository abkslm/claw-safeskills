use clap::Parser;
use keyring::Entry;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The search query
    query: String,
}

#[derive(Deserialize, Debug)]
struct ExaResponse {
    answer: Option<String>,
    citations: Option<Vec<Citation>>,
    error: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Citation {
    // Define fields if needed, or use Value for flexibility
    // implementation details might vary based on API response
}

fn main() {
    let args = Args::parse();
    let query = &args.query;

    // Retrieve API Key
    let entry = Entry::new("exa-search-skill", "API_KEY").unwrap_or_else(|e| {
        eprintln!("Error accessing keychain: {}", e);
        process::exit(1);
    });

    let api_key = match entry.get_password() {
        Ok(key) => key,
        Err(_) => {
            // Fallback to environment variable
            env::var("API_KEY").unwrap_or_else(|_| {
                eprintln!("Error: exa-search-skill:API_KEY not found in keychain (service: exa-search-skill, account: API_KEY) or environment variables.");
                process::exit(1);
            })
        }
    };

    let client = Client::new();
    let url = "https://api.exa.ai/answer";

    let payload = json!({
        "query": query,
        // Add other optional parameters if needed, e.g., use_content: true
    });

    let response = client.post(url)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let text = resp.text().unwrap_or_default();
                // Directly output the raw JSON from Exa, formatted nicely
                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(json_val) => println!("{}", serde_json::to_string_pretty(&json_val).unwrap()),
                    Err(_) => println!("{}", text),
                }
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
