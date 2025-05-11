use serde_json;
use std::fs;
use std::path::Path;
use vc_utils::{ed25519_verify_input_from_preprocessed, VerifyInput, Ed25519Preprocessed};

pub fn map_preprocessed_to_verify_input(preprocessed: Vec<Ed25519Preprocessed>) -> Vec<VerifyInput> {
    preprocessed.into_iter().map(|preprocessed| {
        ed25519_verify_input_from_preprocessed(preprocessed)
            .expect("Failed to create verify input")
    }).collect()
}

fn load_preprocessed(path: &str) -> Ed25519Preprocessed {
    // Read the preprocessed JSON file
    let json_str = fs::read_to_string(path)
        .expect(&format!("Failed to read {}", path));

    // Parse the JSON into Ed25519Preprocessed struct
    let preprocessed: Ed25519Preprocessed =
        serde_json::from_str(&json_str).expect(&format!("Failed to parse preprocessed JSON from {}", path));

    preprocessed
}

fn process_dir(path: &Path, verify_inputs: &mut Vec<Ed25519Preprocessed>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            process_entry(&entry, verify_inputs);
        }
    }
}

fn process_entry(entry: &fs::DirEntry, verify_inputs: &mut Vec<Ed25519Preprocessed>) {
    let path = entry.path();
    
    if path.is_dir() {
        process_dir(&path, verify_inputs);
    } else if path.extension().map_or(false, |ext| ext == "json") {
        // Only process JSON files
        let verify_input = load_preprocessed(&path.to_string_lossy());
        verify_inputs.push(verify_input);
    }
}

pub fn load_preprocessed_dir(path: &str) -> Vec<Ed25519Preprocessed> {
    let mut verify_inputs = Vec::new();
    process_dir(Path::new(path), &mut verify_inputs);

    verify_inputs
}
