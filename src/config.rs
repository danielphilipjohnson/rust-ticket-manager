use anyhow::Result;

pub struct DatabaseConfig {
    pub connection_string: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            return Ok(Self {
                connection_string: url,
            });
        }

        let host = std::env::var("PGHOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("PGPORT").unwrap_or_else(|_| "5432".to_string());
        let user = std::env::var("PGUSER").expect("PGUSER must be set");
        let password = std::env::var("PGPASSWORD").expect("PGPASSWORD must be set");
        let database = std::env::var("PGDATABASE").expect("PGDATABASE must be set");

        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );

        Ok(Self { connection_string })
    }
}
