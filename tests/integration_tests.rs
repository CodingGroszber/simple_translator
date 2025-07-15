use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_english_to_russian_translation() {
    // Setup - get the paths
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_file = project_dir.join("test").join("test_eng_500.md");
    let expected_output_file = project_dir
        .join("test")
        .join("test_eng_500_ru_translated.md");

    println!("Test file path: {:?}", test_file);
    println!("Expected output path: {:?}", expected_output_file);

    // Clean up any previous test output
    if expected_output_file.exists() {
        fs::remove_file(&expected_output_file).expect("Failed to remove old test output file");
    }

    // Run the translator
    let output = Command::new("cargo")
        .current_dir(&project_dir)
        .arg("run")
        .arg("--")
        .arg(&test_file)
        .arg("--source-language")
        .arg("en")
        .arg("--target-language")
        .arg("ru")
        .output()
        .expect("Failed to execute translator command");

    // Print command output for debugging
    println!(
        "Command stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "Command stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check if command executed successfully
    assert!(
        output.status.success(),
        "Translation command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check if the output file was created
    assert!(
        expected_output_file.exists(),
        "Output file was not created: {:?}",
        expected_output_file
    );

    // Read the content of the output file
    let translated_content =
        fs::read_to_string(&expected_output_file).expect("Failed to read translated file");

    // Verify the content is not empty
    assert!(!translated_content.is_empty(), "Translated file is empty");

    // Verify the content appears to be Russian (check for Cyrillic characters)
    let has_cyrillic = translated_content
        .chars()
        .any(|c| matches!(c, '\u{0400}'..='\u{04FF}'));

    assert!(
        has_cyrillic,
        "Translated content does not contain Cyrillic characters"
    );

    println!("Translation test passed successfully!");
}
