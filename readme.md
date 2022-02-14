## download_rs
使用rust写的一个简单 `download` 下载库，是`fengzhongyun1992`的[download_rs](https://crates.io/crates/download_rs)一个 fork ，并且对依赖和 Rust语言特性做了更新。

![https://github.com/bic-potato/download_rs/workflows/Rust/badge.svg](https://github.com/bic-potato/download_rs/workflows/Rust/badge.svg)
![https://github.com/fengzhongyun1992/download_rs/blob/master/LICENSE](https://img.shields.io/badge/license-Apache2.0-blue.svg)

## 简单使用

#### sync_download
* cargo:
```
download_rs = {version="0.2.0",features=["sync_download"]}
```
* code
```rust
use download_rs::sync_download::Download;
use std::error::Error;
use std::fs;

fn main() {
    let url = "https://www.baidu.com/img/bd_logo1.png";
    // 当前目录
     let filename = "bd_log1.png";
    // 指定 下载目录
    // let filename = "/download/";
    // 指定下载目录下载文件名,需要手动创建下载文件夹
    // let filename = "download/bd_log1.png";
    let download = Download::new(url,Some(filename),None);

    match download.download() {
        Ok(_) => println!("下载完成"),
        Err(e) => println!("下载出错 ： {}",e.to_string()),
    }

    // 删除图片
    // fs::remove_file(filename).unwrap();
}
```
#### async_download
* cargo:
```
download_rs = "0.2.0"
```
或
```
download_rs = {version="0.2.0",features=["async_download"]}
```
* code
```rust
use download_rs::async_download::Download;
use std::fs;

fn main() {
    let url = "https://www.baidu.com/img/bd_logo1.png";
    // 当前目录
    let filename = "bd_log1.png";
    // 指定 下载目录
    // let filename = "/download/";
    // 指定下载目录下载文件名,需要手动创建下载文件夹
    // let filename = "download/bd_log1.png";
    let download = Download::new(url,Some(filename),None);

    match download.download() {
        Ok(_) => println!("下载完成"),
        Err(e) => println!("下载出错 ： {}",e.to_string()),
    }

    // 删除图片
    // fs::remove_file(filename).unwrap();
}
```
#### async_download_default
* cargo:
```
download_rs = "0.2.0"
```
或
```
download_rs = {version="0.2.0",features=["async_download"]}
```
* code
```rust
use download_rs::async_download::Download;
use std::fs;

#[tokio::main]
async fn main() {
    let url = "https://www.baidu.com/img/bd_logo1.png";
    // 当前目录
    let filename = "bd_log1.png";

    // cargo test sync_download_test
//    let url = "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";
//    let proxy = Some("http://127.0.0.1:7890");
//    let filename  = "sync_bd_logo1.png";

    // 指定 下载目录
    // let filename = "/download/";
    // 指定下载目录下载文件名,需要手动创建下载文件夹
    // let filename = "download/bd_log1.png";
    let download = Download::new(url,Some(filename),None);
//    let download = Download::new(url,Some(filename),proxy);
    match download.download_async().await {
        Err(e) => panic!("error: {}",e.to_string()),
        Ok(()) => println!("ok")
    }
    // 删除图片
    // fs::remove_file(filename).unwrap();
}
```

## 版本说明
#### 0.2.0
* 使用 new方法创建 Download对象
* 添加 proxy 字段
* 和 0.1.0 不兼容 🤭🤭🤭🤭
