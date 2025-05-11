use criterion::{black_box, criterion_group, criterion_main, Criterion};
use host::run::{Args, Mode, run};
use std::path::PathBuf;

fn bench_prove(c: &mut Criterion) {
    let binding = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = binding.parent().unwrap();
    
    let args = Args {
        mode: Mode::Prove,
        path: workspace_root.join("data/generated/ed25519-preprocessed/").to_string_lossy().to_string(),
        query_file: workspace_root.join("query.sparql").to_string_lossy().to_string(),
        output_file: workspace_root.join("sparql_result.json").to_string_lossy().to_string(),
    };

    c.bench_function("prove default", |b| {
        b.iter(|| {
            black_box(run(&args))
        })
    });

    let verify_args = Args {
        mode: Mode::Verify,
        path: workspace_root.join("data/generated/ed25519-preprocessed/").to_string_lossy().to_string(),
        query_file: workspace_root.join("query.sparql").to_string_lossy().to_string(),
        output_file: workspace_root.join("sparql_result.json").to_string_lossy().to_string(),
    };

    c.bench_function("verify default", |b| {
        b.iter(|| {
            black_box(run(&verify_args))
        })
    });
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = bench_prove
}
criterion_main!(benches); 
