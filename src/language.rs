use anyhow::{Result, anyhow};
use whatlang::detect;

/// Detects the language of the given text and returns its ISO code
pub fn detect_language(text: &str) -> Result<String> {
    // Use whatlang library to detect the language
    let detection = detect(text).ok_or_else(|| {
        anyhow!("Language detection failed. Text might be too short or ambiguous.")
    })?;

    let lang = detection.lang();

    // Convert whatlang's language enum to ISO code strings
    // We only support English, Russian, and German for now
    let iso_code = match lang {
        whatlang::Lang::Eng => "en",
        whatlang::Lang::Rus => "ru",
        whatlang::Lang::Deu => "de",
        _ => return Err(anyhow!("Unsupported language detected: {:?}", lang)),
    };

    Ok(iso_code.to_string())
}

// Checks if the language pair is supported by our translator
// Currently we only support English-Russian and English-German pairs (both directions)
// #[allow(dead_code)]
// pub fn is_supported_pair(source: &str, target: &str) -> bool {
//     matches!(
//         (source, target),
//         ("en", "ru") | ("ru", "en") | ("en", "de") | ("de", "en")
//     )
// }

// Returns all supported language codes
// #[allow(dead_code)]
// pub fn supported_languages() -> Vec<&'static str> {
//     vec!["en", "ru", "de"]
// }
