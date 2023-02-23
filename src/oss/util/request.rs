use anyhow::{anyhow, Result};
use reqwest::{header::HeaderMap, Method};

pub async fn request(
    method: Method,
    url: &str,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
) -> Result<String> {
    let client = reqwest::Client::new();
    let response = match body {
        Some(body) => {
            client
                .request(method, url)
                .headers(headers)
                .body(body)
                .send()
                .await?
        }
        None => client.request(method, url).headers(headers).send().await?,
    };

    let status = response.status();
    let text = response.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "Request failed with status code {}: {}",
            status,
            text
        ));
    }

    Ok(text)
}
