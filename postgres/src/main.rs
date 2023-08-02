mod entities;
mod migration;

use crate::migration::{get_client, run_all_up_migrations, run_down_migration};
use clap::{Arg, ArgAction, Command};

/// Run migration with given name:
///   cargo run migration run ".env.dev"
/// Reset the database
///   cargo run database reset ".env.dev"
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let app = Command::new("supabase")
    .about("Tool for manager supabase")
    .subcommand(
      Command::new("database").subcommand(
        Command::new("reset").about("Reset the database").arg(
          Arg::new("env")
            .action(ArgAction::Set)
            .value_name("ENV File Name")
            .required(true),
        ),
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
              println!("Reset databases from env: {:?}", env_file_name);
              let mut client = get_client(env_file_name).await?;
              run_down_migration(&client).await?;
              run_all_up_migrations(&mut client).await?;
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
