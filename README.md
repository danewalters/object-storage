# object storage

The goal of this library is to make a library that can use both "Aliyun-oss" and "Tencent Cloud-cos", simplifying the steps of use and allowing development to focus on business logic.

## oss

### oss bucket list

```rust
use object_storage::oss::Auth;

#[tokio::main]
async fn main() {
    let auth = Auth::new("your aliyun oss AccessKeyId", "your aliyun oss AccessKeySecret").await;
    let buck_list = auth.list_bucket().await.unwarp();
    println!("{:#?}", buck_list);
}
```
### oss put object

Simple general upload of a single file

```rust
use object_storage::oss::{Auth,Bucket};

#[tokio::main]
async fn main() {
    let auth = Auth::new("your aliyun oss AccessKeyId", "your aliyun oss AccessKeySecret").await;
    let bucket = Bucket::new(auth, "your endpoint", "your bucket name").await.unwrap();
    let object_url = bucket.read_put_object("path").await.unwarp();
}
```

#### tokio

Uploading multiple files at the same time using tokio

```rust
use object_storage::oss::{Auth, Bucket};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let auth = Auth::new("your aliyun oss AccessKeyId", "your aliyun oss AccessKeySecret").await;
    let bucket = Arc::new(Mutex::new(Bucket::new(auth, "your endpoint", "your bucket name").await.unwrap()));

    let mut handles = Vec::new();

    handles.push(tokio::spawn({
        let bucket = bucket.clone();
        async move {
            let guard = bucket.lock().await;
            guard.read_put_object("/Users/joeywang/Downloads/1.jpg").await
        }
    }));
    handles.push(tokio::spawn({
        let bucket = bucket.clone();
        async move {
            let guard = bucket.lock().await;
            guard.read_put_object("/Users/joeywang/Downloads/2.jpg").await
        }
    }));
    handles.push(tokio::spawn({
        let bucket = bucket.clone();
        async move {
            let guard = bucket.lock().await;
            guard.read_put_object("/Users/joeywang/Downloads/3.jpg").await
        }
    }));
    handles.push(tokio::spawn({
        let bucket = bucket.clone();
        async move {
            let guard = bucket.lock().await;
            guard.read_put_object("/Users/joeywang/Downloads/4.png").await
        }
    }));
    handles.push(tokio::spawn({
        let bucket = bucket.clone();
        async move {
            let guard = bucket.lock().await;
            guard.read_put_object("/Users/joeywang/Downloads/5.png").await
        }
    }));
    handles.push(tokio::spawn({
        let bucket = bucket.clone();
        async move {
            let guard = bucket.lock().await;
            guard.read_put_object("/Users/joeywang/Downloads/6.png").await
        }
    }));
    handles.push(tokio::spawn({
        let bucket = bucket.clone();
        async move {
            let guard = bucket.lock().await;
            guard.read_put_object("/Users/joeywang/Downloads/7.jpg").await
        }
    }));
    handles.push(tokio::spawn({
        let bucket = bucket.clone();
        async move {
            let guard = bucket.lock().await;
            guard.read_put_object("/Users/joeywang/Downloads/8.jpg").await
        }
    }));

    for handle in handles {
        match handle.await {
            Ok(result) => {
                match result {
                    Ok(url) => println!("Upload succeeded: {}", url),
                    Err(e) => eprintln!("Upload failed: {}", e),
                }
            },
            Err(e) => eprintln!("Failed to spawn task: {}", e),
        }
    }
}
```