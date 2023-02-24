use anyhow::{Ok, Result};
use base64::{engine::general_purpose, Engine};
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use sha1::Sha1;
type HmacSha1 = Hmac<Sha1>;

pub async fn auth_sh1(sha1_key: &[u8], data: &[u8]) -> Result<String> {
    let mut mac = HmacSha1::new_from_slice(sha1_key)?;
    mac.update(data);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(general_purpose::STANDARD.encode(code_bytes))
}

// pub fn content_md5
pub async fn content_md5(file: &[u8]) -> Result<String> {
    let mut hasher = Md5::new();
    hasher.update(file);
    let result = hasher.finalize();
    Ok(general_purpose::STANDARD.encode(result))
}
