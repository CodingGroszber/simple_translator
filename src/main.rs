use anyhow::{Result, anyhow};
use clap::Parser;
use log::info;
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about = "A simple translator app using pre-trained MarianMT models", long_about = None)]
struct Args {
    /// The text to translate
    #[arg(required = true)]
    text: String,

    /// Source language (e.g., en)
    #[arg(short = 's', long, default_value = "en")]
    source: String,

    /// Target language (e.g., fr)
    #[arg(short = 't', long, default_value = "fr")]
    target: String,

    /// Reset any cached models or configurations
    #[arg(long)]
    reset: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let project_dir: PathBuf = std::env::current_dir()?;
    let assets_dir: PathBuf = project_dir.join("assets");

    match set_environment(&assets_dir) {
        Ok(()) => {
            println!("Environment set successfully!");
        }
        Err(e) => {
            eprintln!("Error setting environment: {}", e);
            std::process::exit(1);
        }
    }

    if args.reset {
        match reset_assets_directory(&assets_dir) {
            Ok(()) => println!("Successfully reset assets directory"),
            Err(e) => eprintln!("Failed to reset assets directory: {}", e),
        }
    }

    info!("Starting translation for text: '{}'", args.text);

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

fn set_environment(assets_dir: &PathBuf) -> Result<()> {
    // Set custom cache directory for rust-bert assets using std::env::current_dir
    fs::create_dir_all(&assets_dir)?;
    unsafe {
        std::env::set_var("RUSTBERT_CACHE", &assets_dir);
    }

    log::info!("Set rust-bert cache directory to: {:?}", assets_dir);

    Ok(())
}

fn reset_assets_directory(assets_dir: &PathBuf) -> std::io::Result<()> {
    // Remove the entire 'assets' directory if it exists
    if assets_dir.exists() {
        fs::remove_dir_all(&assets_dir)?;
    }

    // Recreate an empty 'assets' directory
    fs::create_dir(&assets_dir)?;

    println!("Reset: 'assets' directory cleared.");
    Ok(())
}

fn language_from_str(s: &str) -> Result<Language> {
    match s.to_lowercase().as_str() {
        "en" => Ok(Language::English),
        "fr" => Ok(Language::French),
        "es" => Ok(Language::Spanish),
        "it" => Ok(Language::Italian),
        "ru" => Ok(Language::Russian),
        "ge" => Ok(Language::German),
        "hu" => Ok(Language::Hungarian), // Todo: Hungarian uses facebook/m2m100_418M, not Helsinki-NLP/opus-mt-en-hu
        _ => Err(anyhow!("Unsupported language: {}.", s)),
    }
}
