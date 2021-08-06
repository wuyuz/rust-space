use config::{Config as RConfig, Environment, File};
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use std::env;

#[derive(Debug,Serialize,Deserialize)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub auth_token: String,
    pub database: Database,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut s = RConfig::default();

        s.merge(File::with_name("config/default"))
            .context("Unable to load the default config")?;

        let env = env::var("ENV").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))
            .context(format!("Unable to load config/{}.json", env))?;  // context会根据上下文显示出具体的错误原因
 
        s.merge(Environment::new().separator("_".into()))?;  // 通过：ENV=test cargo run 可以加载test配置，通过ENV=test DATABASE_URL=xxxx cargo run，可加载对应的变量
        s.try_into().context("Unable to instantiate Config struct")
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new().unwrap();
}