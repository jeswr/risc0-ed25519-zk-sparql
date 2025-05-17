use crate::run::{run, Args, Mode, QueryMode};
use std::path::PathBuf;

fn root_dir() -> PathBuf {
    let binding = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    binding.parent().unwrap().to_path_buf()
}

fn args(mode: Mode, query_file: &str) -> Args {
    let workspace_root = root_dir();
    Args {
      mode: mode,
      query_mode: QueryMode::String,
      path: Some(workspace_root.join("data/generated/ed25519-preprocessed/").to_string_lossy().to_string()),
      query_file: Some(workspace_root.join(query_file).to_string_lossy().to_string()),
      output_file: workspace_root.join("sparql_result.json").to_string_lossy().to_string(),
  }
}

pub fn run_internal(mode: Mode, query_file: &str) {
  run(&args(mode, query_file))
}

// #[test]
// fn test_prove() {
//   run_internal(Mode::Prove);
// }

// #[test]
// fn test_verify() {
//   run_internal(Mode::Verify);
// }
