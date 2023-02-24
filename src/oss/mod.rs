pub mod object;
pub mod service;
pub mod util;
use self::{
    service::BucketInfo,
    util::{crypto::content_md5, file::file_name},
};
use anyhow::{Ok, Result};
use util::file::{file_type, read_file};
#[derive(Debug)]
pub struct Auth {
    pub accesskeyid: String,
    pub accesskeysecret: String,
}

impl Auth {
    pub async fn new(accesskeyid: &str, accesskeysecret: &str) -> Self {
        Auth {
            accesskeyid: accesskeyid.to_owned(),
            accesskeysecret: accesskeysecret.to_owned(),
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

impl Bucket {
    pub async fn new(auth: Auth, endpoint: &str, bucket_name: &str) -> Result<Bucket> {
        Ok(Bucket {
            auth,
            endpoint: endpoint.to_owned(),
            bucket_name: bucket_name.to_owned(),
        })
    }
}

impl Bucket {
    pub async fn read_put_object(&self, path: &str) -> Result<String> {
        let file = read_file(path).await?;
        let file_type = file_type(path).await?;
        let file_name = file_name(path).await?;
        let content = Content::new(&file, &file_type).await?;
        let b = object::put_object(&self, &file, &content, &file_name).await?;
        Ok(b)
    }

    pub async fn put_object(
        &self,
        file: Vec<u8>,
        file_type: &str,
        file_name: &str,
    ) -> Result<String> {
        let content = Content::new(&file, file_type).await?;
        let b = object::put_object(&self, &file, &content, &file_name).await?;
        Ok(b)
    }
}

#[derive(Debug)]
pub struct Content {
    pub content_md5: String,
    pub content_type: String,
}

impl Content {
    pub async fn new(file: &[u8], file_type: &str) -> Result<Self> {
        Ok(Content {
            content_md5: content_md5(file).await?,
            content_type: file_type.to_string(),
        })
    }
}
