use anyhow::Result;

pub async fn create_host(bucket_name: Option<&str>, endpoint: &str) -> Result<String> {
    let base_url = format!("{}.{}", endpoint, "aliyuncs.com");
    let host = match bucket_name {
        Some(bucket_name) => format!("{}.{}", bucket_name, base_url),
        None => base_url,
    };
    Ok(host)
}

pub async fn create_url(host: &str) -> Result<String> {
    let url = format!("https://{}", host);
    Ok(url)
}
