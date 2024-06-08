use crate::utils::config::Config;
use anyhow::Result;
use log::info;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;

pub async fn create_repository(path: String) -> Result<()> {
    info!("Creating repository for path: {}", path);
    let warthog_dir_path = Path::new(&path).join(".warthog");
    create_dir_all(&warthog_dir_path).await?;

    let snapshots_dir_path = Path::new(&path).join(".warthog/snapshots");
    create_dir_all(&snapshots_dir_path).await?;

    let blobs_dir_path = Path::new(&path).join(".warthog/blobs");
    create_dir_all(&blobs_dir_path).await?;
    Ok(())
}

async fn create_snapshot(path: &str) -> Result<Vec<String>> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let snapshot_dir_path = Path::new(&path).join(format!(".warthog/snapshots/{}", timestamp));
    create_dir_all(&snapshot_dir_path).await?;

    Ok(WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|entry| entry.path().display().to_string())
        .collect())
}

async fn deduplicate_files(
    config: &Config,
    file_paths: Vec<String>,
) -> Result<HashMap<String, Vec<u8>>> {
    let mut block_map = HashMap::new();
    for path in file_paths {
        let mut file = File::open(&path).await?;
        let mut buffer = vec![0u8; config.block_size as usize];
        while let Ok(bytes_read) = file.read(&mut buffer).await {
            if bytes_read == 0 {
                // End of file
                break;
            }

            let data_block = &buffer[..bytes_read];
            let mut hasher = Sha256::new();
            hasher.update(data_block);
            let hash = format!("{:x}", hasher.finalize());

            block_map.entry(hash).or_insert(data_block.to_vec());
        }
    }
    Ok(block_map)
}
