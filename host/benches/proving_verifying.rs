use criterion::{black_box, criterion_group, criterion_main, Criterion};
use host::{util::run_internal, run::Mode};

fn bench_prove(c: &mut Criterion) {
    c.bench_function("prove default", |b| {
        b.iter(|| {
            black_box(run_internal(Mode::Prove))
        })
    });

    c.bench_function("verify default", |b| {
        b.iter(|| {
            black_box(run_internal(Mode::Verify))
        })
    });
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = bench_prove
}
criterion_main!(benches); 
