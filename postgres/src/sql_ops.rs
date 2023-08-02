use tokio_postgres::Client;
use uuid::Uuid;

pub async fn insert_into_af_collab_update(
  client: &mut Client,
  oid: &Uuid,
  value: &str,
  partition_key: i32,
  uid: i64,
  workspace_id: &Uuid,
) -> Result<u64, tokio_postgres::Error> {
  let statement = client
    .prepare(
      "
        INSERT INTO af_collab_update (oid, value, value_size, partition_key, uid, md5, workspace_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    ",
    )
    .await?;
  let md5 = format!("{:x}", md5::compute(value.as_bytes()));
  client
    .execute(
      &statement,
      &[
        &oid.to_string(),
        &value.as_bytes(),
        &(value.len() as i32),
        &partition_key,
        &uid,
        &md5,
        &workspace_id,
      ],
    )
    .await
}

pub async fn insert_into_af_user(
  client: &mut Client,
  uuid: &Uuid,
  email: &str,
  name: &str,
) -> Result<i64, tokio_postgres::Error> {
  Ok(
    client
      .query_one(
        "
        INSERT INTO af_user (uuid, email, name)
        VALUES ($1, $2, $3)
        RETURNING uid
    ",
        &[&uuid, &email, &name],
      )
      .await?
      .get(0),
  )
}

pub async fn select_workspace_ids_from_af_workspace_for_owner(
  client: &mut Client,
  owner_uid: i64,
) -> Result<Vec<Uuid>, tokio_postgres::Error> {
  Ok(
    client
      .query(
        "
        SELECT workspace_id
        FROM af_workspace
        WHERE owner_uid = $1
    ",
        &[&owner_uid],
      )
      .await?
      .iter()
      .map(|row| row.get(0))
      .collect::<Vec<Uuid>>(),
  )
}
