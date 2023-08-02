use criterion::{criterion_group, criterion_main, Criterion};

fn insert_row_benchmark(c: &mut Criterion) {
  c.bench_function("insert_row", |b| b.iter(|| todo!()));
}

criterion_group!(benches, insert_row_benchmark);
criterion_main!(benches);
