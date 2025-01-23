// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;
use reqwest::Client;
use sha2::{Digest, Sha256};
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine; // Add this line to bring the Engine trait into scope
use rand::Rng;

fn load_env_variables() {
    // Loads the .env file into the environment variables
    if let Err(e) = dotenv() {
        eprintln!("Warning: Failed to load .env file: {}", e);
    }
}
lazy_static::lazy_static! {
    static ref CODE_VERIFIER: Mutex<Option<String>> = Mutex::new(None);
}

// Generate a random code verifier
fn generate_code_verifier() -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
    let mut rng = rand::thread_rng();
    (0..128)
        .map(|_| charset[rng.gen_range(0..charset.len())] as char)
        .collect()
}

// Generates a code challenge (SHA256 hash of the code verifier, base64-encoded)
fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    base64_engine.encode(hash)
}

#[tauri::command]
fn generate_spotify_auth_url() -> String {

    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI must be set");
    // Generate a new code verifier and challenge
    let verifier = generate_code_verifier();
    let challenge = generate_code_challenge(&verifier);

    // Store the verifier securely in memory
    CODE_VERIFIER.lock().unwrap().replace(verifier);

    format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope=user-read-private user-read-email&redirect_uri={}&code_challenge_method=S256&code_challenge={}",
        client_id, redirect_uri, challenge
    )
}

#[tauri::command]
async fn exchange_spotify_token(code: String) -> Result<String, String> {

    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI must be set");
    // Retrieve the code verifier from memory
    let verifier = CODE_VERIFIER
        .lock()
        .unwrap()
        .take()
        .ok_or_else(|| "Code verifier not found".to_string())?;

    let payload = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", &redirect_uri),
        ("client_id", &client_id),
        ("code_verifier", &verifier),
    ];

    // Send POST request to Spotify's token endpoint
    let response = Client::new()
        .post("https://accounts.spotify.com/api/token")
        .form(&payload)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                Ok(resp.text().await.unwrap()) // Return tokens as a string
            } else {
                Err(format!("Token exchange failed: {}", resp.status()))
            }
        }
        Err(err) => Err(format!("Error occurred: {}", err)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    load_env_variables();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_spotify_auth_url,
            exchange_spotify_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}