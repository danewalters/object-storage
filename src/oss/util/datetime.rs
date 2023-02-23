use anyhow::{Ok, Result};
use chrono::Utc;
const GMT_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";

pub async fn create_datetime() -> Result<String> {
    let now = Utc::now();
    Ok(now.format(GMT_FORMAT).to_string())
}
