# object storage

The goal of this library is to make a library that can use both "Aliyun-oss" and "Tencent Cloud-cos", simplifying the steps of use and allowing development to focus on business logic.

## oss

For oss currently only supports viewing the bucket list

```rust
use object_storage::oss::Auth;

#[tokio::main]
async fn main() {
    let auth = Auth::new("your aliyun oss AccessKeyId", "your aliyun oss AccessKeySecret").await;
    let buck_list = auth.list_bucket().await.unwarp();
    println!("{:#?}", buck_list);
}
```
