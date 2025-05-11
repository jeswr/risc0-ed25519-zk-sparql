use criterion::{black_box, criterion_group, criterion_main, Criterion};
use host::{util::run_internal, run::Mode};

fn bench_prove(c: &mut Criterion) {
    c.bench_function("prove-default", |b| {
        b.iter(|| {
            black_box(run_internal(Mode::Prove, "query.sparql"))
        })
    });

    c.bench_function("verify-default", |b| {
        b.iter(|| {
            black_box(run_internal(Mode::Verify, "query.sparql"))
        })
    });

    c.bench_function("prove-can-drive", |b| {
      b.iter(|| {
          black_box(run_internal(Mode::Prove, "data/queries/can-drive.rq"))
      })
  });

  c.bench_function("verify-can-drive", |b| {
      b.iter(|| {
          black_box(run_internal(Mode::Verify, "data/queries/can-drive.rq"))
      })
  });

  c.bench_function("prove-employment-status", |b| {
      b.iter(|| {
          black_box(run_internal(Mode::Prove, "data/queries/employment-status.rq"))
      })
  }); 

  c.bench_function("verify-employment-status", |b| {
      b.iter(|| {
          black_box(run_internal(Mode::Verify, "data/queries/employment-status.rq"))
      })
  });
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = bench_prove
}
criterion_main!(benches); 
