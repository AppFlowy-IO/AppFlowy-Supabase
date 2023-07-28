use tokio_postgres::{Client, NoTls};
use crate::entities::PostgresConfiguration;
use crate::migration::run_migrations;

mod migration;
mod entities;

#[tokio::main]
async fn main() {
    if dotenv::from_filename(".env.test").is_err() {
        return;
    }

    let configuration = PostgresConfiguration::from_env().unwrap();
    let mut config = tokio_postgres::Config::new();
    config
        .host(&configuration.url)
        .user(&configuration.user_name)
        .password(&configuration.password)
        .port(configuration.port);

    // Using the https://docs.rs/postgres-openssl/latest/postgres_openssl/ to enable tls connection.
    if let Ok((mut client, connection)) = config.connect(NoTls).await {
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("postgres db connection error: {}", e);
            }
        });

        match run_migrations(&mut client).await {
            Ok(_) => println!("migrations success"),
            Err(e) => println!("migrations error: {}", e),
        }
    }

}

#[allow(dead_code)]
async fn run_initial_drop(client: &Client) {
    let sql = include_str!("migrations/initial/initial_down.sql");
    client.batch_execute(sql).await.unwrap();
    client
        .batch_execute("DROP TABLE IF EXISTS af_migration_history")
        .await
        .unwrap();
}