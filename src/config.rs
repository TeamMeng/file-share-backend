use anyhow::Result;
use dotenv::var;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub jwt_secret_key: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self> {
        let db_url = var("DATABASE_URL")?;
        let jwt_secret_key = var("JWT_SECRET_KEY")?;
        let jwt_maxage = var("JWT_MAXAGE")?.parse::<i64>()?;
        Ok(Self {
            db_url,
            jwt_secret_key,
            jwt_maxage,
            port: 8080,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_should_work() -> Result<()> {
        let config = Config::new()?;
        assert_eq!(
            config.db_url,
            "postgresql://postgres:postgres@localhost:5432/file_share"
        );
        assert_eq!(config.jwt_secret_key, "my_ultra_secure_jwt_secret_key");
        assert_eq!(config.jwt_maxage, 60);
        assert_eq!(config.port, 8080);
        Ok(())
    }
}
