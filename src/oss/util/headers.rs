use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, DATE, HOST,
};

/// create request header
pub async fn construct_headers(
    content_md5: Option<&str>,
    contest_length: Option<&str>,
    contest_type: Option<&str>,
    host: &str,
    date: &str,
    authorization: &str,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    insert_header_if_present(&mut headers, CONTENT_LENGTH, contest_length).await;
    insert_header_if_present(&mut headers, CONTENT_TYPE, contest_type).await;
    insert_header_if_present(
        &mut headers,
        HeaderName::from_static("content-md5"),
        content_md5,
    )
    .await;
    insert_header(&mut headers, AUTHORIZATION, authorization);
    insert_header(&mut headers, HOST, host);
    insert_header(&mut headers, DATE, date);
    headers
}

async fn insert_header_if_present(
    headers: &mut HeaderMap,
    header_name: HeaderName,
    header_value: Option<&str>,
) {
    if let Some(header_value) = header_value {
        insert_header(headers, header_name, header_value);
    }
}

fn insert_header(headers: &mut HeaderMap, header_name: HeaderName, header_value: &str) {
    headers.insert(header_name, HeaderValue::from_str(header_value).unwrap());
}
