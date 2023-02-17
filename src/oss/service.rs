use serde::{Deserialize, Serialize};

use super::{
    authorization::{
        construct_headers, create_authorization, create_datetime, create_host, create_url,
    },
    Auth,
};

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

pub fn list_bucket(auth: &Auth) -> Vec<BucketInfo> {
    // The default endpoint of list_bucket is "oss-cn-hangzhou".
    let host = create_host(None, "oss-cn-hangzhou");
    let url = create_url(&host);
    let verb = "GET";
    let content = None;
    let datetime = create_datetime();
    let caon_header = None;
    let caon_resource = "/";
    let authorization =
        create_authorization(auth, verb, content, &datetime, caon_header, caon_resource);
    let headers = construct_headers(None, None, &host, &datetime, &authorization);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap();
    parse_bucket_info(&res)
}

pub fn parse_bucket_info(xml: &str) -> Vec<BucketInfo> {
    let bucketlist: ListAllMyBucketsResult = serde_xml_rs::from_str(xml).unwrap();
    let mut result = vec![];
    for bucket in bucketlist.buckets.bucket {
        result.push(bucket);
    }
    result
}
