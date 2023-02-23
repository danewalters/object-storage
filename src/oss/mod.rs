pub mod service;
pub mod util;
use self::service::BucketInfo;
use anyhow::Result;

#[derive(Debug)]
pub struct Auth {
    pub accesskeyid: String,
    pub accesskeysecret: String,
}

impl Auth {
    pub async fn new(accesskeyid: &str, accesskeysecret: &str) -> Self {
        Auth {
            accesskeyid: accesskeyid.to_string(),
            accesskeysecret: accesskeysecret.to_string(),
        }
    }

    pub async fn list_bucket(&self) -> Result<Vec<BucketInfo>> {
        service::list_bucket(self).await
    }
}

pub struct Bucket {
    pub auth: Auth,
    pub endpoint: String,
    pub bucket_name: String,
}

pub struct Content {
    // 文件的md5计算
    pub content_md5: String,
    pub content_type: String,
}
