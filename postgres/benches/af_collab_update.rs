use criterion::async_executor::FuturesExecutor;
use criterion::Criterion;

async fn insert_row() -> i32 {
  todo!()
}

fn insert_row_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("insert_row");
  group.bench_function("insert_row", |b| {
    b.to_async(FuturesExecutor).iter(|| insert_row())
  });
  group.finish();
}

criterion::criterion_group!(benches, insert_row_benchmark);
criterion::criterion_main!(benches);
