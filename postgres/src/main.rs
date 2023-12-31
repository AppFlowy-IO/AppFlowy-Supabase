mod entities;
mod migration;
pub mod sql_ops;

use crate::migration::{get_client, run_all_up_migrations, run_down_migration};
use clap::{Arg, ArgAction, Command};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use entities::PostgresConfiguration;
use tokio_postgres::{Client, NoTls};

/// Run migration with given name:
///   cargo run migration run ".env.dev"
/// Reset the database
///   cargo run database reset ".env.dev"
#[tokio::main]
#[allow(dead_code)]
#[allow(clippy::collapsible_match)]
#[allow(clippy::single_match)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let app = Command::new("supabase")
    .about("Tool for manager supabase")
    .subcommand(
      Command::new("database").subcommand(
        Command::new("reset")
          .about("Reset the database")
          .arg(
            Arg::new("env")
              .action(ArgAction::Set)
              .value_name("ENV File Name")
              .required(true),
          )
          .arg(Arg::new("no-verify").action(ArgAction::Set).required(false)),
      ),
    )
    .subcommand(
      Command::new("migration").subcommand(
        Command::new("run")
          .about("Run migration with given name")
          .arg(
            Arg::new("env")
              .action(ArgAction::Set)
              .value_name("ENV File Name")
              .required(true),
          ),
      ),
    );

  let matches = app.get_matches();

  // Match on the provided command and perform appropriate actions
  if let Some(subcommand) = matches.subcommand() {
    match subcommand {
      ("database", supabase_matches) => {
        if let Some(subcommand) = supabase_matches.subcommand() {
          match subcommand {
            ("reset", migration_matches) => {
              let env_file_name = migration_matches
                .try_get_one::<String>("env")
                .expect("Missing migration env")
                .unwrap();

              if env_file_name.ends_with(".prod") {
                println!("Can't reset the production database");
                return Ok(());
              }

              let reset_fn = || async {
                println!("Reset databases from env: {:?}", env_file_name);
                let mut client = get_client(env_file_name).await?;
                run_down_migration(&client).await?;
                run_all_up_migrations(&mut client).await?;
                Ok::<(), anyhow::Error>(())
              };

              let is_no_verify = migration_matches
                .try_contains_id("no-verify")
                .unwrap_or(false);
              if is_no_verify {
                reset_fn().await?;
              } else {
                let selection = Select::with_theme(&ColorfulTheme::default())
                  .with_prompt(format!(
                    "Are you sure to reset the database: {}?",
                    env_file_name
                  ))
                  .default(0)
                  .items(&["No", "Yes"])
                  .interact()
                  .unwrap();
                match selection {
                  0 => println!("Cancel"),
                  1 => {
                    reset_fn().await?;
                  },
                  _ => unreachable!(), // default case; shouldn't happen
                }
              }
            },
            _ => (),
          }
        }
      },
      ("migration", supabase_matches) => {
        if let Some(subcommand) = supabase_matches.subcommand() {
          match subcommand {
            ("run", migration_matches) => {
              let env_file_name = migration_matches
                .try_get_one::<String>("env")
                .expect("Missing migration env")
                .unwrap();
              println!("Running migration from env: {:?}", env_file_name);
              let mut client = get_client(env_file_name).await?;
              run_all_up_migrations(&mut client).await?;
            },
            _ => (),
          }
        }
      },
      _ => (),
    }
  }

  Ok(())
}

pub async fn connect_to_dev_postgres() -> Result<Client, tokio_postgres::Error> {
  if dotenv::from_filename(".env.dev").is_err() {
    tracing::warn!("no .env.dev file found");
  }
  let configuration = PostgresConfiguration::from_env().unwrap();
  let mut config = tokio_postgres::Config::new();
  config
    .host(&configuration.url)
    .user(&configuration.user_name)
    .password(&configuration.password)
    .port(configuration.port);

  match config.connect(NoTls).await {
    Ok((client, connection)) => {
      tokio::spawn(async move {
        if let Err(e) = connection.await {
          panic!("postgres db connection error: {}", e);
        }
      });

      Ok(client)
    },
    Err(e) => {
      // print config details
      println!("config: {:?}", config);
      panic!("postgres db connection error: {}", e)
    },
  }
}
