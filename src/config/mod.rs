use std::path::Path;
use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use figment::providers::{Env, Format, Toml};
use figment::Figment;
use serde::Deserialize;

mod log_config;
pub use log_config::LogConfig;
mod db_config;
pub use db_config::DbConfig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub fn init() {
    CONFIG.get_or_init(|| {
        let raw_config = Figment::new()
            .merge(Toml::file(
                Env::var("APP_CONFIG").as_deref().unwrap_or("config.toml"),
            ))
            .merge(Env::prefixed("APP_").global());

        let mut config = match raw_config.extract::<ServerConfig>() {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "It looks like your config is invalid. The following error occurred: {e}"
                );
                std::process::exit(1);
            }
        };
        if config.db.url.is_empty() {
            config.db.url = std::env::var("DATABASE_URL").unwrap_or_default();
        }
        if config.db.url.is_empty() {
            eprintln!("DATABASE_URL is not set");
            std::process::exit(1);
        }
        config
    });
}

pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    pub db: DbConfig,
    pub log: LogConfig,
    pub jwt: JwtConfig,
    pub redis: RedisConfig,
    pub tls: Option<TlsConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RedisConfig {
    pub host: String,
    pub password: Option<String>,
    pub port: u16,
    pub timeout: u64,
    pub database: u8,
    pub max_active: u32,
    pub max_wait: i64,
    pub max_idle: u32,
    pub min_idle: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

#[allow(dead_code)]
pub fn default_false() -> bool {
    false
}

#[allow(dead_code)]
pub fn default_true() -> bool {
    true
}

fn default_listen_addr() -> String {
    "127.0.0.1:8008".into()
}

impl ServerConfig {
    pub fn validate(&self) -> Result<()> {
        if self.listen_addr.trim().is_empty() {
            return Err(anyhow!("listen_addr 不能为空"));
        }
        self.db.validate()?;
        self.jwt.validate()?;
        self.redis.validate()?;
        if let Some(tls) = &self.tls {
            tls.validate()?;
        }
        Ok(())
    }
}

impl DbConfig {
    pub fn validate(&self) -> Result<()> {
        if self.url.trim().is_empty() {
            return Err(anyhow!("数据库配置 url 不能为空"));
        }
        Ok(())
    }
}

impl JwtConfig {
    pub fn validate(&self) -> Result<()> {
        if self.secret.trim().is_empty() {
            return Err(anyhow!("jwt.secret 不能为空"));
        }
        Ok(())
    }
}

impl RedisConfig {
    pub fn validate(&self) -> Result<()> {
        if self.host.trim().is_empty() {
            return Err(anyhow!("redis.host 不能为空"));
        }
        if self.port == 0 {
            return Err(anyhow!("redis.port 不能为 0"));
        }
        Ok(())
    }
}

impl TlsConfig {
    pub fn validate(&self) -> Result<()> {
        if self.cert.trim().is_empty() || self.key.trim().is_empty() {
            return Err(anyhow!("tls.cert/tls.key 不能为空"));
        }
        if !Path::new(&self.cert).exists() {
            return Err(anyhow!("tls.cert 文件不存在: {}", self.cert));
        }
        if !Path::new(&self.key).exists() {
            return Err(anyhow!("tls.key 文件不存在: {}", self.key));
        }
        Ok(())
    }
}
