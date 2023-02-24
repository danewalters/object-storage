use super::{
    util::{
        authorization::create_authorization,
        datetime::create_datetime,
        headers::construct_headers,
        host::{create_host, create_resource, create_url},
        request::request,
    },
    Bucket, Content,
};
use anyhow::{anyhow, Result};
use reqwest::Method;

pub async fn put_object(
    bucket: &Bucket,
    file: &[u8],
    content: &Content,
    file_name: &str,
) -> Result<String> {
    let host = create_host(Some(&bucket.bucket_name), &bucket.endpoint)
        .await
        .map_err(|e| anyhow!("{}", e))?;
    let url = create_url(&host, Some(file_name))
        .await
        .map_err(|e| anyhow!("{}", e))?;
    let datetime = create_datetime().await.map_err(|e| anyhow!("{}", e))?;
    let caon_resource = create_resource(&bucket.bucket_name, file_name)
        .await
        .map_err(|e| anyhow!("{}", e))?;
    let authorization = create_authorization(
        &bucket.auth,
        "PUT",
        Some(content),
        &datetime,
        None,
        &caon_resource,
    )
    .await
    .map_err(|e| anyhow!("{}", e))?;
    let content_length = file.len().to_string();
    let headers = construct_headers(
        Some(&content.content_md5),
        Some(&content_length),
        Some(&content.content_type),
        &host,
        &datetime,
        &authorization,
    )
    .await;
    let _text = request(Method::PUT, &url, headers, Some(file.to_vec()))
        .await
        .map_err(|e| anyhow!("{}", e))?;
    Ok(url)
}
