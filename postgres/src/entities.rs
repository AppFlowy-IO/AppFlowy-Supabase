use anyhow::Error;

pub const SUPABASE_DB: &str = "SUPABASE_DB";
pub const SUPABASE_DB_USER: &str = "SUPABASE_DB_USER";
pub const SUPABASE_DB_PASSWORD: &str = "SUPABASE_DB_PASSWORD";
pub const SUPABASE_DB_PORT: &str = "SUPABASE_DB_PORT";

pub const PGHOST: &str = "PGHOST";
pub const PGPORT: &str = "PGPORT";
pub const PGUSER: &str = "PGUSER";
pub const PGPASSWORD: &str = "PGPASSWORD";

#[derive(Debug, Default, Clone)]
pub struct PostgresConfiguration {
  pub url: String,
  pub user_name: String,
  pub password: String,
  pub port: u16,
}

impl PostgresConfiguration {
  pub fn from_env() -> Result<Self, Error> {
    let url = std::env::var(SUPABASE_DB)
      .or(std::env::var(PGHOST))
      .map_err(|_| anyhow::anyhow!("Missing SUPABASE_DB or PGDB"))?;
    let user_name = std::env::var(SUPABASE_DB_USER)
      .or(std::env::var(PGUSER))
      .map_err(|_| anyhow::anyhow!("Missing SUPABASE_DB_USER or PGUSER"))?;
    let password = std::env::var(SUPABASE_DB_PASSWORD)
      .or(std::env::var(PGPASSWORD))
      .map_err(|_| anyhow::anyhow!("Missing SUPABASE_DB_PASSWORD or PGPASSWORD"))?;
    let port = std::env::var(SUPABASE_DB_PORT)
      .or(std::env::var(PGPORT))
      .map_err(|_| anyhow::anyhow!("Missing SUPABASE_DB_PORT or PGPORT"))?
      .parse::<u16>()
      .map_err(|e| anyhow::anyhow!("Invalid SUPABASE_DB_PORT or PGPORT: {}", e))?;

    Ok(Self {
      url,
      user_name,
      password,
      port,
    })
  }
}
