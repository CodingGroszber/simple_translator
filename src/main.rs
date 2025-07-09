use anyhow::{Context, Result};
use clap::Parser;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use std::fs;
use std::path::Path;

// Define supported languages
#[derive(Debug, Clone, Copy)]
enum SupportedLanguage {
    English,
    Russian,
    German,
}

impl SupportedLanguage {
    fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" => Some(SupportedLanguage::English),
            "ru" => Some(SupportedLanguage::Russian),
            "de" => Some(SupportedLanguage::German),
            _ => None,
        }
    }

    fn to_code(&self) -> &'static str {
        match self {
            SupportedLanguage::English => "en",
            SupportedLanguage::Russian => "ru",
            SupportedLanguage::German => "de",
        }
    }

    fn to_name(&self) -> &'static str {
        match self {
            SupportedLanguage::English => "English",
            SupportedLanguage::Russian => "Russian",
            SupportedLanguage::German => "German",
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A markdown file language detector and translator"
)]
struct Args {
    /// Input markdown file path
    #[arg(short, long)]
    input_file: String,
}

// Language detection function
fn detect_language(text: &str) -> Result<SupportedLanguage> {
    // Configure the detector with the languages we care about
    let languages = vec![Language::English, Language::Russian, Language::German];

    let detector = LanguageDetectorBuilder::from_languages(&languages).build();

    let detected = match detector.detect_language_of(text) {
        Some(Language::English) => SupportedLanguage::English,
        Some(Language::Russian) => SupportedLanguage::Russian,
        Some(Language::German) => SupportedLanguage::German,
        _ => {
            println!("Warning: Language could not be confidently detected. Assuming English.");
            SupportedLanguage::English
        }
    };

    Ok(detected)
}

// This is a placeholder for the translation function
// In the future, we'll implement actual translation here
fn placeholder_translate(
    text: &str,
    source_lang: SupportedLanguage,
    target_lang: SupportedLanguage,
) -> Result<String> {
    // For demonstration only - real implementation would go here

    println!("FUTURE IMPLEMENTATION NOTE:");
    println!("For actual translation, we would:");
    println!(
        "1. Load a translation model for {}-{}",
        source_lang.to_code(),
        target_lang.to_code()
    );
    println!("2. Process the text in chunks to preserve markdown structure");
    println!("3. Return properly translated content");

    // For now, we'll just add a note about the translation
    let mut lines = Vec::new();

    lines.push(format!(
        "# Translation from {} to {}",
        source_lang.to_name(),
        target_lang.to_name()
    ));
    lines.push(String::new());
    lines.push("*This is a placeholder for translation functionality.*".to_string());
    lines.push(String::new());
    lines.push("## Original text:".to_string());
    lines.push(String::new());
    lines.push(text.to_string());

    Ok(lines.join("\n"))
}

// Function to determine target language based on source
fn determine_target_language(source: SupportedLanguage) -> SupportedLanguage {
    match source {
        SupportedLanguage::English => SupportedLanguage::Russian, // Default to Russian
        _ => SupportedLanguage::English, // For non-English, translate to English
    }
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Read the source file
    let content = fs::read_to_string(&args.input_file)
        .context(format!("Failed to read input file: {}", args.input_file))?;

    // Detect language
    let detected_lang = detect_language(&content)?;
    println!("Detected language: {}", detected_lang.to_name());

    // Determine target language
    let target_lang = determine_target_language(detected_lang);
    println!("Target language for translation: {}", target_lang.to_name());

    // Use placeholder translation for now
    let translated_content = placeholder_translate(&content, detected_lang, target_lang)?;

    // Generate output filename
    let input_path = Path::new(&args.input_file);
    let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
    let parent = input_path.parent().unwrap_or(Path::new(""));
    let output_filename = parent.join(format!("{}_translated.md", file_stem));

    // Write translated content to file
    fs::write(&output_filename, translated_content).context(format!(
        "Failed to write to output file: {}",
        output_filename.display()
    ))?;
    println!("Output saved to: {}", output_filename.display());

    Ok(())
}
