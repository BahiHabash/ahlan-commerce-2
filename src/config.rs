use std::{env, net::SocketAddr};

pub const ENV_HOST: &str = "AHLAN_HOST";
pub const ENV_PORT: &str = "AHLAN_PORT";
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_RUN_REFINERY: &str = "RUN_REFINERY_MIGRATIONS";
pub const ENV_REDIS_URL: &str = "REDIS_URL";

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: Option<String>,
    pub redis_url: Option<String>,
    pub run_refinery_migrations: bool,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var(ENV_HOST).unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var(ENV_PORT)
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(3000),
            database_url: env::var(ENV_DATABASE_URL).ok(),
            redis_url: env::var(ENV_REDIS_URL).ok(),
            run_refinery_migrations: env::var(ENV_RUN_REFINERY)
                .map(|value| value == "true" || value == "1")
                .unwrap_or(false),
        }
    }

    pub fn bind_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("valid bind address")
    }
}
