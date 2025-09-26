use anyhow::{Result, anyhow};
use clap::Parser;
use log::info;
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about = "A simple translator app using pre-trained MarianMT models", long_about = None)]
struct Args {
    /// The text to translate
    text: String,

    /// Source language (e.g., English)
    #[arg(short = 's', long, default_value = "English")]
    source: String,

    /// Target language (e.g., French)
    #[arg(short = 't', long, default_value = "French")]
    target: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    info!("Starting translation for text: '{}'", args.text);

    // Set custom cache directory for rust-bert assets using std::env::current_dir
    let project_dir = std::env::current_dir()
        .map_err(|e| anyhow!("Could not determine project directory: {}", e))?;
    let cache_dir = project_dir.join("assets");
    fs::create_dir_all(&cache_dir)
        .map_err(|e| anyhow!("Could not create assets directory: {}", e))?;
    unsafe {
        std::env::set_var("RUSTBERT_CACHE", &cache_dir);
    }

    info!("Set rust-bert cache directory to: {:?}", cache_dir);

    let source = language_from_str(&args.source)?;
    let target = language_from_str(&args.target)?;

    // Build the model for the specified language pair (downloads if needed)
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![source])
        .with_target_languages(vec![target])
        .create_model()?;

    // Perform translation
    let outputs = model.translate(&[&args.text], None, target)?;

    println!("Translation: {}", outputs[0]);

    Ok(())
}

fn language_from_str(s: &str) -> Result<Language> {
    match s.to_lowercase().as_str() {
        "english" => Ok(Language::English),
        "french" => Ok(Language::French),
        "spanish" => Ok(Language::Spanish),
        "italian" => Ok(Language::Italian),
        "russian" => Ok(Language::Russian),
        "german" => Ok(Language::German),
        _ => Err(anyhow!(
            "Unsupported language: {}. Supported: English, French, Spanish, Italian, Russian, German",
            s
        )),
    }
}
