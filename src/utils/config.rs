use anyhow::Result;
use dotenvy::dotenv;
use std::env;

/// Struct for storing the configuration from environment variables.
pub struct Config {
    pub database_url: String,
    pub block_size: u32,
    pub pipe_path: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").unwrap_or(String::from("sqlite://warthog.db"));
        let block_size = env::var("BLOCK_SIZE").unwrap_or(String::from("1024")).parse::<u32>()?;
        let pipe_path = env::var("PIPE_PATH").unwrap_or(String::from("/tmp/warthog.sock"));

        Ok(Self {
            database_url,
            block_size,
            pipe_path,
        })
    }
}
