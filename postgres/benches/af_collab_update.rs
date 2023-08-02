use criterion::{criterion_group, criterion_main, Criterion};
use postgres::{
  self,
  sql_ops::{
    insert_into_af_collab_update, insert_into_af_user,
    select_workspace_ids_from_af_workspace_for_owner,
  },
};
use uuid::Uuid;

fn insert_row_benchmark(c: &mut Criterion) {
  let rt = tokio::runtime::Runtime::new().unwrap();
  let mut client = rt.block_on(postgres::connect_to_dev_postgres()).unwrap();

  let uuid = Uuid::new_v4();
  let nano_str = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_nanos();
  let name = format!("{}-{}", "test_user", nano_str);
  let email = format!("{}@test.com", name);
  let uid = rt
    .block_on(insert_into_af_user(&mut client, &uuid, &email, &name))
    .unwrap();
  let workspace_ids = rt
    .block_on(select_workspace_ids_from_af_workspace_for_owner(
      &mut client,
      uid,
    ))
    .unwrap();
  if workspace_ids.len() == 0 {
    panic!("No workspace ids found for user {}", uid);
  }
  let target_workspace_id = workspace_ids[0];
  let target_partition_key = 1;

  let mut group = c.benchmark_group("insert_row");
  group.bench_function("insert_row", |b| {
    b.iter(|| {
      rt.block_on(insert_into_af_collab_update(
        &mut client,
        &Uuid::new_v4(),
        &"value1",
        target_partition_key,
        uid,
        &target_workspace_id,
      ))
    });
  });
  group.finish();
}

criterion_group!(benches, insert_row_benchmark);
criterion_main!(benches);
