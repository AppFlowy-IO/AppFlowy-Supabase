use criterion::{criterion_group, criterion_main, Criterion};
use postgres::{
  self,
  sql_ops::{
    delete_from_af_collab_update, insert_into_af_collab_update, insert_into_af_user,
    select_keys_from_af_collab_update, select_workspace_ids_from_af_workspace_for_owner,
  },
};
use rand::thread_rng;
use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

fn insert_delete_row_benchmark(c: &mut Criterion) {
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
  let target_value = generate_random_str(200);
  let target_oid = Uuid::new_v4();

  {
    let mut insert_group = c.benchmark_group("insert_row_group");
    insert_group.bench_function("insert_row_function", |b| {
      b.iter(|| {
        rt.block_on(insert_into_af_collab_update(
          &mut client,
          &target_oid,
          &target_value,
          target_partition_key,
          uid,
          &target_workspace_id,
        ))
        .unwrap();
      });
    });
    insert_group.finish();
  }

  {
    let mut keys = rt
      .block_on(select_keys_from_af_collab_update(
        &mut client,
        &target_oid,
        target_partition_key,
        &target_workspace_id,
      ))
      .unwrap();
    let mut rng = thread_rng();
    keys.shuffle(&mut rng);
    let mut delete_group = c.benchmark_group("delete_row_group");
    delete_group.sample_size(keys.len());
    delete_group.bench_function("delete_row", |b| {
      b.iter(|| {
        let key = match keys.pop() {
          Some(k) => k,
          None => return, // No more keys to delete
        };
        rt.block_on(delete_from_af_collab_update(
          &mut client,
          &target_oid,
          key,
          target_partition_key,
          &target_workspace_id,
        ))
        .unwrap();
      })
    });
    delete_group.finish();
  }
}

fn generate_random_str(length: usize) -> String {
  const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let mut rng = rand::thread_rng();
  let random_str: String = (0..length)
    .map(|_| {
      let index = rng.gen::<usize>() % CHARSET.len();
      CHARSET[index] as char
    })
    .collect();

  random_str
}

criterion_group!(benches, insert_delete_row_benchmark);
criterion_main!(benches);
