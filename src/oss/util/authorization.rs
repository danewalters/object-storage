use super::crypto::auth_sh1;
use crate::oss::{Auth, Content};
use anyhow::{anyhow, Result};
pub async fn create_authorization(
    auth: &Auth,
    verb: &str,
    content: Option<&Content>,
    datetime: &str,
    caon_header: Option<&str>,
    caon_resource: &str,
) -> Result<String> {
    let (md5, content_type) = match content {
        Some(content) => (content.content_md5.as_str(), content.content_type.as_str()),
        None => ("", ""),
    };
    let caon_header = caon_header.unwrap_or("");
    let data = format!(
        "{}\n{}\n{}\n{}\n{}{}",
        verb, md5, content_type, datetime, caon_header, caon_resource
    );
    let hash = auth_sh1(auth.accesskeysecret.as_bytes(), data.as_bytes())
        .await
        .map_err(|e| anyhow!("Failed to compute auth hash: {}", e))?;
    Ok(format!("OSS {}:{}", auth.accesskeyid, hash))
}
