use anyhow::{Context, Result};
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
