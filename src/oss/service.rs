use serde::{Deserialize, Serialize};

use super::{
    authorization::{
        construct_headers, create_authorization, create_datetime, create_host, create_url,
    },
    Auth, BucketList,
};
#[derive(Serialize, Deserialize, Debug)]
struct ListAllMyBucketsResult {
    #[serde(rename = "Owner")]
    owner: Owner,
    #[serde(rename = "Buckets")]
    buckets: Buckets,
}

#[derive(Serialize, Deserialize, Debug)]
struct Owner {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "DisplayName")]
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Buckets {
    #[serde(rename = "Bucket")]
    bucket: Vec<Bucket>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bucket {
    #[serde(rename = "Comment")]
    comment: String,
    #[serde(rename = "CreationDate")]
    creation_date: String,
    #[serde(rename = "ExtranetEndpoint")]
    extranet_endpoint: String,
    #[serde(rename = "IntranetEndpoint")]
    intranet_endpoint: String,
    #[serde(rename = "Location")]
    location: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Region")]
    region: String,
    #[serde(rename = "StorageClass")]
    storage_class: String,
}

pub fn list_bucket(auth: &Auth) -> BucketList {
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

pub fn parse_bucket_info(xml: &str) -> Vec<(String, String, String)> {
    let bucketlist: ListAllMyBucketsResult = serde_xml_rs::from_str(xml).unwrap();
    let mut result = vec![];
    for bucket in bucketlist.buckets.bucket {
        result.push((bucket.location, bucket.name, bucket.region));
    }
    result
}
