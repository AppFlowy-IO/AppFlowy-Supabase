use crate::entities::PostgresConfiguration;
use anyhow::{anyhow, Error};
use refinery::embed_migrations;
use tokio_postgres::{Client, NoTls};

embed_migrations!("./migrations");

const AF_MIGRATION_HISTORY: &str = "af_migration_history";

pub async fn run_all_up_migrations(client: &mut Client) -> Result<(), anyhow::Error> {
  match migrations::runner()
    .set_migration_table_name(AF_MIGRATION_HISTORY)
    .run_async(client)
    .await
  {
    Ok(report) => {
      if !report.applied_migrations().is_empty() {
        println!(
          "✅ Run {} postgres db migration",
          report.applied_migrations().len()
        );
        for migration in report.applied_migrations() {
          println!("✅ Applied migration: {}", migration.name());
        }
      }
      println!("✅ Run migration successfully");
      Ok(())
    },
    Err(e) => Err(anyhow::anyhow!("❌Run migration failed with error: {}", e)),
  }
}

pub async fn run_down_migration(client: &Client) -> Result<(), Error> {
  let sql = include_str!("../migrations/V1__initial.down.sql");
  client.batch_execute(sql).await?;

  let sql = include_str!("../migrations/V2__realtime.down.sql");
  client.batch_execute(sql).await?;

  let sql = include_str!("../migrations/V4__encryption.down.sql");
  client.batch_execute(sql).await?;

  client
    .batch_execute("DROP TABLE IF EXISTS af_migration_history")
    .await?;
  Ok(())
}

/// 1. The .env.dev is used by developers for their work, including building new features and fixing bugs
/// 2. The .env.stage is a replica of the production environment used for testing. It's where you deploy
/// and test changes before they go live in production.
/// 3. The .env.prod is the production environment.
pub async fn get_client(env_file_name: &str) -> Result<Client, anyhow::Error> {
  if dotenv::from_filename(env_file_name).is_err() {
    return Err(anyhow!(
      "Can't find the env file with given name: {}",
      env_file_name
    ));
  }

  let configuration = PostgresConfiguration::from_env().unwrap();
  let mut config = tokio_postgres::Config::new();
  config
    .host(&configuration.url)
    .user(&configuration.user_name)
    .password(&configuration.password)
    .port(configuration.port);

  // Using the https://docs.rs/postgres-openssl/latest/postgres_openssl/ to enable tls connection.
  let (client, connection) = config.connect(NoTls).await?;
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("postgres db connection error: {}", e);
    }
  });

  Ok(client)
}
