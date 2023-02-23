use super::{
    util::{
        authorization::create_authorization,
        datetime::create_datetime,
        headers::construct_headers,
        host::{create_host, create_url},
        request::request,
    },
    Auth,
};
use anyhow::{Ok, Result};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAllMyBucketsResult {
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "Buckets")]
    pub buckets: Buckets,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Buckets {
    #[serde(rename = "Bucket")]
    pub bucket: Vec<BucketInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BucketInfo {
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: String,
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "StorageClass")]
    pub storage_class: String,
}

pub async fn list_bucket(auth: &Auth) -> Result<Vec<BucketInfo>> {
    let host = create_host(None, "oss-cn-hangzhou").await?;
    let url = create_url(&host).await?;
    let datetime = create_datetime().await?;
    let authorization = create_authorization(auth, "GET", None, &datetime, None, "/").await?;
    let headers = construct_headers(None, None, &host, &datetime, &authorization).await;
    let text = request(Method::GET, &url, headers, None).await?;
    parse_bucket_info(&text).await
}

pub async fn parse_bucket_info(xml: &str) -> Result<Vec<BucketInfo>> {
    let bucketlist: ListAllMyBucketsResult = serde_xml_rs::from_str(xml)?;
    Ok(bucketlist.buckets.bucket)
}
