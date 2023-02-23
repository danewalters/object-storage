use super::crypto::auth_sh1;
use crate::oss::{Auth, Content};
use anyhow::{Ok, Result};
pub async fn create_authorization(
    auth: &Auth,
    verb: &str,
    content: Option<&Content>,
    datetime: &str,
    caon_header: Option<&str>,
    caon_resource: &str,
) -> Result<String> {
    if let Some(_content) = content {
        panic!("Currently does not support validating the hash of uploaded files");
    }
    let data = format!(
        "{}\n\n\n{}\n{}{}",
        verb,
        datetime,
        caon_header.unwrap_or_default(),
        caon_resource
    );
    let hash = auth_sh1(auth.accesskeysecret.as_bytes(), &data).await?;
    Ok(format!("OSS {}:{}", auth.accesskeyid, hash))
}
