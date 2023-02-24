use anyhow::{Ok, Result};

pub async fn create_host(bucket_name: Option<&str>, endpoint: &str) -> Result<String> {
    let base_url = format!("{}.{}", endpoint, "aliyuncs.com");
    let host = match bucket_name {
        Some(bucket_name) => format!("{}.{}", bucket_name, base_url),
        None => base_url,
    };
    Ok(host)
}

pub async fn create_url(host: &str, file_name: Option<&str>) -> Result<String> {
    let base_url = format!("https://{}", host);
    let url = match file_name {
        Some(file_name) => format!("{}/{}", base_url, file_name),
        None => base_url,
    };
    Ok(url)
}

pub async fn create_resource(bucket_name: &str, object_name: &str) -> Result<String> {
    Ok(format!("/{}/{}", bucket_name, object_name))
}
