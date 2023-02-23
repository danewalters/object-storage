use anyhow::{Ok, Result};
use base64::{engine::general_purpose, Engine};
use hmac::{Hmac, Mac};
use sha1::Sha1;
type HmacSha1 = Hmac<Sha1>;

pub async fn auth_sh1(sha1_key: &[u8], data: &str) -> Result<String> {
    let mut mac = HmacSha1::new_from_slice(sha1_key)?;
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(general_purpose::STANDARD.encode(code_bytes))
}

// pub fn content_md5
