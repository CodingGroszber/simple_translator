use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// Default API URL for LibreTranslate - can be overridden with LIBRETRANSLATE_API_URL environment variable
const DEFAULT_API_URL: &str = "http://localhost:5000/translate";

/// Request structure for LibreTranslate API
#[derive(Serialize)]
struct TranslationRequest {
    q: String,      // The text to translate
    source: String, // Source language code
    target: String, // Target language code
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>, // Optional API key if required by your LibreTranslate instance
}

/// Response structure from LibreTranslate API
#[derive(Deserialize)]
struct TranslationResponse {
    translatedText: String, // The translated text
}

/// Main translator struct that handles API communication
pub struct Translator {
    client: Client,          // HTTP client for API calls
    api_url: String,         // LibreTranslate API URL
    api_key: Option<String>, // Optional API key
}

impl Translator {
    /// Create a new translator instance
    pub fn new() -> Self {
        // Get API URL from environment variable or use default
        let api_url =
            env::var("LIBRETRANSLATE_API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());

        // Get API key from environment variable if present
        let api_key = env::var("LIBRETRANSLATE_API_KEY").ok();

        Self {
            client: Client::new(),
            api_url,
            api_key,
        }
    }

    /// Translate text from source language to target language
    pub async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String> {
        // Create the translation request
        let request = TranslationRequest {
            q: text.to_string(),
            source: source_lang.to_string(),
            target: target_lang.to_string(),
            api_key: self.api_key.clone(),
        };

        println!("Sending translation request to: {}", self.api_url);

        // Send the translation request to the API
        let response = self
            .client
            .post(&self.api_url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to LibreTranslate API")?;

        // Handle error responses
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!("Translation API request failed: {}", error_text));
        }

        // Parse the successful response
        let translation: TranslationResponse = response
            .json()
            .await
            .context("Failed to parse translation response")?;

        Ok(translation.translatedText)
    }
}
