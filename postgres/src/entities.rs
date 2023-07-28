use anyhow::Error;

pub const SUPABASE_DB: &str = "SUPABASE_DB";
pub const SUPABASE_DB_USER: &str = "SUPABASE_DB_USER";
pub const SUPABASE_DB_PASSWORD: &str = "SUPABASE_DB_PASSWORD";
pub const SUPABASE_DB_PORT: &str = "SUPABASE_DB_PORT";

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
            .map_err(|_| anyhow::anyhow!("Missing SUPABASE_DB"))?;
        let user_name = std::env::var(SUPABASE_DB_USER)
            .map_err(|_| anyhow::anyhow!("Missing SUPABASE_DB_USER"))?;
        let password = std::env::var(SUPABASE_DB_PASSWORD)
            .map_err(|_| anyhow::anyhow!("Missing SUPABASE_DB_PASSWORD"))?;
        let port = std::env::var(SUPABASE_DB_PORT)
            .map_err(|_| anyhow::anyhow!("Missing SUPABASE_DB_PORT"))?
            .parse::<u16>()
            .map_err(|_e| anyhow::anyhow!("Missing SUPABASE_DB_PORT"))?;

        Ok(Self {
            url,
            user_name,
            password,
            port,
        })
    }

}