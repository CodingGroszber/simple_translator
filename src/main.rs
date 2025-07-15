use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

// Import our modules
mod file_handler;
mod language;
mod translator;

// Import the components we need from each module
use file_handler::FileHandler;
use language::detect_language;
use translator::Translator;

/// Pretty print macro for debugging
#[allow(unused_macros)]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

/// Command-line arguments for our translator app
#[derive(Parser, Debug)]
#[clap(
    name = "simple_translator_v2",
    about = "A CLI tool for translating text files between English, Russian, and German"
)]
struct Args {
    /// Path to the file to be translated (.md or .txt)
    #[clap(name = "FILE")]
    file: PathBuf,

    /// Source language code (e.g., 'en', 'ru', 'de')
    #[clap(long)]
    source_language: Option<String>,

    /// Target language code (e.g., 'en', 'ru', 'de')
    #[clap(long)]
    target_language: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // debug_println!("Aruments used:{:#?}\n", args);

    // Validate the input file exists and has the right extension
    if !args.file.exists() {
        return Err(anyhow::anyhow!("File not found: {:?}", args.file));
    }

    let extension = args
        .file
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    if extension != "md" && extension != "txt" {
        return Err(anyhow::anyhow!(
            "Unsupported file format: {}. Only .md and .txt files are supported.",
            extension
        ));
    }

    // Create a file handler to read/write files
    let file_handler = FileHandler::new(&args.file);
    let content = file_handler.read().context("Failed to read file")?;

    // debug_println!("{:#?}\n", content);

    // Determine source language - use provided value or detect
    let source_lang = if let Some(lang) = args.source_language {
        lang
    } else {
        detect_language(&content).context("Failed to detect language")?
    };

    println!("Source language: {}", source_lang);

    // Determine target language - use provided value or default based on source
    let target_lang = if let Some(lang) = args.target_language {
        lang
    } else {
        match source_lang.as_str() {
            "en" => "ru".to_string(),
            _ => "en".to_string(),
        }
    };

    println!("Target language: {}", target_lang);

    // Create translator and translate the content
    let translator = Translator::new();
    // debug_println!("{:#?}\n", translator);
    let translated_text = translator
        .translate(&content, &source_lang, &target_lang)
        .await
        .context("Translation failed")?;

    // Write the translated content to a new file
    file_handler
        .write_translated(&translated_text, &target_lang)
        .context("Failed to write translated file")?;

    println!("Translation complete!");
    Ok(())
}
