use anyhow::Result;
use dotenvy::dotenv;
use std::env;

/// Struct for storing the configuration from environment variables.
pub struct Config {
    pub block_size: u32,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenv().ok();

        let block_size = env::var("BLOCK_SIZE")
            .map(|v| v.parse::<u32>().unwrap_or(1024))?;

        Ok(Self {
            block_size,
        })
    }
}
