pub mod authorization;
pub mod service;

/// aliyun oss AccessKey
///
/// # Example
/// ```ignore
/// // accesskeyid is aliyun oss AccessKeyId
/// // accesskeysecret is aliyun oss AccessKeySecret
/// let auth = Auth {
///     accesskeyid: "accesskeyid".to_string(),
///     accesskeysecret: "accesskeysecret".to_string()
/// };
/// ```
#[derive(Debug)]
pub struct Auth {
    pub accesskeyid: String,
    pub accesskeysecret: String,
}

impl Auth {
    pub fn new(accesskeyid: &str, accesskeysecret: &str) -> Self {
        Auth {
            accesskeyid: accesskeyid.to_string(),
            accesskeysecret: accesskeysecret.to_string(),
        }
    }

    pub fn list_bucket(&self) -> Vec<service::BucketInfo> {
        service::list_bucket(self)
    }
}

/// Information needed to operate the bucket
///
/// # Example
/// ```ignore
/// let auth = Auth {
///     accesskeyid: "accesskeyid".to_string(),
///     accesskeysecret: "accesskeysecret".to_string()
/// };
///
/// let bucket = Bucket{
///     auth: auth,
///     endpoint: "oss-cn-chengdu".to_string(),
///     bucket_name: "your bucket name".to_string()
/// };
/// ```
pub struct Bucket {
    pub auth: Auth,
    pub endpoint: String,
    pub bucket_name: String,
}

/// Request Content
/// ```ignore
/// let contest = Content {
///     // MD5 value of the requested content data
///     content_md5: "eB5eJF1ptWaXm4bijSPyxw==".to_string();
///     // Type of request content
///     content_type: "application/octet-stream".to_string();
/// }
/// ```
pub struct Content {
    pub content_md5: String,
    pub content_type: String,
}

/// http request method
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}
