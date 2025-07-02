use anyhow::{Context, Result};
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppSection,
    pub log: LogSection,
    pub data_source: DataSourceSection,
}

#[derive(Debug, Deserialize)]
pub struct AppSection {
    pub host: String,
    pub port: u16,
    pub data_source: String, // e.g., "mysql" or "etcd"
    pub api_key: String,     // API key for authentication
}

#[derive(Debug, Deserialize)]
pub struct LogSection {
    pub log_level: String,
    pub log_dir: String,
    pub log_name: String,
}

#[derive(Debug, Deserialize)]
pub struct DataSourceSection {
    pub mysql: Option<MySQLConfig>,
    pub etcd: Option<EtcdConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MySQLConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct EtcdConfig {
    pub endpoints: Vec<String>,
    pub username: String,
    pub password: String,
}

impl AppConfig {
    pub fn load_from_env() -> Result<Self> {
        let config_dir = std::env::var("CONFIG_DIR").unwrap_or_else(|_| "config".into());
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        let builder = Config::builder()
            .add_source(File::with_name(&format!("{}/default", config_dir)))
            .add_source(File::with_name(&format!("{}/{}", config_dir, run_mode)).required(false))
            .add_source(Environment::with_prefix("APP").separator("__"));

        builder
            .build()
            .context("Failed to build config")?
            .try_deserialize()
            .context("Failed to deserialize config")
    }
}
