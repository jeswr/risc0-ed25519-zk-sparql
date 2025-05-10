use serde_json;
use std::fs;
use std::path::Path;
use vc_utils::{ed25519_verify_input_from_preprocessed, VerifyInput, Ed25519Preprocessed};

fn load_preprocessed(path: &str) -> VerifyInput {
    // Read the preprocessed JSON file
    let json_str = fs::read_to_string(path)
        .expect(&format!("Failed to read {}", path));

    // Parse the JSON into Ed25519Preprocessed struct
    let preprocessed: Ed25519Preprocessed =
        serde_json::from_str(&json_str).expect(&format!("Failed to parse preprocessed JSON from {}", path));

    // Convert to VerifyInput
    let verify_input = ed25519_verify_input_from_preprocessed(preprocessed)
        .expect(&format!("Failed to create verify input from {}", path));

    verify_input
}

fn process_dir(path: &Path, verify_inputs: &mut Vec<VerifyInput>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            process_entry(&entry, verify_inputs);
        }
    }
}

fn process_entry(entry: &fs::DirEntry, verify_inputs: &mut Vec<VerifyInput>) {
    let path = entry.path();
    
    if path.is_dir() {
        process_dir(&path, verify_inputs);
    } else if path.extension().map_or(false, |ext| ext == "json") {
        // Only process JSON files
        let verify_input = load_preprocessed(&path.to_string_lossy());
        verify_inputs.push(verify_input);
    }
}

pub fn load_preprocessed_dir(path: &str) -> Vec<VerifyInput> {
    let mut verify_inputs = Vec::new();
    process_dir(Path::new(path), &mut verify_inputs);

    verify_inputs
}
