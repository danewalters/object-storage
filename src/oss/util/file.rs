use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use mime_guess::from_path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)
        .await
        .with_context(|| format!("failed to open file at '{}'", path))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .with_context(|| format!("failed to read file at '{}'", path))?;
    Ok(buffer)
}

pub async fn file_type(path: &str) -> Result<String> {
    let mime_type = from_path(path).first_or_octet_stream();
    Ok(mime_type.to_string())
}

pub async fn file_name(path: &str) -> Result<String> {
    let file_name = PathBuf::from(path)
        .file_name()
        .ok_or_else(|| anyhow!("file name not found in path: {}", path))?
        .to_str()
        .ok_or_else(|| anyhow!("invalid UTF-8 file name in path: {}", path))?
        .to_string();
    Ok(file_name)
}
