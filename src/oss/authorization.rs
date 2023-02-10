use super::{Auth, Content};
use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, DATE, HOST,
};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;
const GMT_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";

/// Create a authorization token
///
/// # Example
///
/// ```
/// use object_storage::oss::{Auth, authorization::create_authorization};
/// let auth = Auth {accesskeyid: "accesskeyid".to_string(),accesskeysecret: "accesskeysecret".to_string()};
/// let verb = "GET";
/// let content = None;
/// let datetime = "Mon, 13 Feb 2023 03:03:55 GMT";
/// let caon_header = None;
/// let caon_resource = "/";
/// let a = create_authorization(auth, verb, content, &datetime, caon_header, caon_resource);
/// assert_eq!("OSS accesskeyid:2544AF5j8Ji7EIlrdCxLt/J4ru4=".to_string(), a)
/// ```
pub fn create_authorization(
    auth: &Auth,
    verb: &str,
    content: Option<&Content>,
    datetime: &str,
    caon_header: Option<&str>,
    caon_resource: &str,
) -> String {
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
    let mut mac = HmacSha1::new_from_slice(auth.accesskeysecret.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let signature = general_purpose::STANDARD.encode(code_bytes);
    format!("OSS {}:{}", auth.accesskeyid, signature)
}
/// Create Datetime for GMT
///
/// # Example
/// ```ignore
/// let datetime = create_datetime();
/// ```
pub fn create_datetime() -> String {
    let now = Utc::now();
    now.format(GMT_FORMAT).to_string()
}

/// create request host
///
/// # Example
///
/// ```
/// use object_storage::oss::authorization::create_host;
/// let url = create_host(None, "oss-cn-hangzhou");
/// let test_url = "oss-cn-hangzhou.aliyuncs.com".to_string();
/// assert_eq!(test_url, url);
/// ```
pub fn create_host(bucket_name: Option<&str>, endpoint: &str) -> String {
    let base_url = format!("{}.{}", endpoint, "aliyuncs.com");
    match bucket_name {
        Some(bucket_name) => format!("{}.{}", bucket_name, base_url),
        None => base_url,
    }
}

/// create request url
pub fn create_url(host: &str) -> String {
    format!("https://{}", host)
}

/// create request header
pub fn construct_headers(
    contest_length: Option<&str>,
    contest_type: Option<&str>,
    host: &str,
    date: &str,
    authorization: &str,
) -> HeaderMap {
    let mut headers = HeaderMap::new();

    insert_header_if_present(&mut headers, CONTENT_LENGTH, contest_length);
    insert_header_if_present(&mut headers, CONTENT_TYPE, contest_type);
    insert_header(&mut headers, AUTHORIZATION, authorization);
    insert_header(&mut headers, HOST, host);
    insert_header(&mut headers, DATE, date);

    headers
}

fn insert_header_if_present(
    headers: &mut HeaderMap,
    header_name: HeaderName,
    header_value: Option<&str>,
) {
    if let Some(header_value) = header_value {
        insert_header(headers, header_name, header_value);
    }
}

fn insert_header(headers: &mut HeaderMap, header_name: HeaderName, header_value: &str) {
    headers.insert(
        header_name,
        HeaderValue::from_bytes(header_value.as_bytes()).unwrap(),
    );
}
