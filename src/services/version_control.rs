use anyhow::{Context, Result};
use std::collections::HashMap;
use sha2::{Digest, Sha256};
use tokio::fs;
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;
use crate::utils::config::Config;

async fn create_snapshot(project_path: &str) -> Vec<String> {
    WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|entry| entry.path().display().to_string())
        .collect()
}

async fn deduplicate_files(config: &Config, file_paths: Vec<String>) -> Result<HashMap<String, Vec<u8>>> {
    let mut block_map = HashMap::new();
    for path in file_paths {
        let mut file = fs::File::open(&path).await?;
        let mut buffer = vec![0u8; config.block_size as usize];
        while let Ok(bytes_read) = file.read(&mut buffer).await {
            if bytes_read == 0 { break; } // End of file

            let data_block = &buffer[..bytes_read];
            let mut hasher = Sha256::new();
            hasher.update(data_block);
            let hash = format!("{:x}", hasher.finalize());

            block_map.entry(hash).or_insert(data_block.to_vec());
        }
    }
    Ok(block_map)
}