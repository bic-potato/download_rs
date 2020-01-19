## download_rs
使用rust写的一个简单 `download` 下载库

![https://github.com/fengzhongyun1992/download_rs/actions](https://github.com/fengzhongyun1992/download_rs/workflows/Rust/badge.svg)
![https://github.com/fengzhongyun1992/download_rs/blob/master/LICENSE](https://img.shields.io/badge/license-Apache2.0-blue.svg)

## 简单使用

#### sync_download
* cargo:
```
download_rs = {version="0.1.0",features=["sync_download"]}
```
* code
```rust
use download_rs::sync_download::Download;
use std::error::Error;

fn main() {
    let url = "https://www.baidu.com/img/bd_logo1.png";
    // 当前目录
     let filename = "bd_log1.png";
    // 指定 下载目录
    // let filename = "/download/";
    // 指定下载目录下载文件名,需要手动创建下载文件夹
    // let filename = "download/bd_log1.png";
    let download = Download{
        url,
        out: Some(filename)
    };
    match download.download() {
        Ok(_) => println!("下载完成"),
        Err(e) => println!("下载出错 ： {}",e.to_string()),
    }
}
```
#### async_download
* cargo:
```
download_rs = "0.1.0"
```
或
```
download_rs = {version="0.1.0",features=["async_download"]}
```
* code
```rust
use download_rs::async_download::Download;

fn main() {
    let url = "https://www.baidu.com/img/bd_logo1.png";
    // 当前目录
    let filename = "bd_log1.png";
    // 指定 下载目录
    // let filename = "/download/";
    // 指定下载目录下载文件名,需要手动创建下载文件夹
    // let filename = "download/bd_log1.png";
    let download = Download{
        url,
        out: Some(filename)
    };
    match download.download() {
        Ok(_) => println!("下载完成"),
        Err(e) => println!("下载出错 ： {}",e.to_string()),
    }
}
```
#### async_download_default
* cargo:
```
download_rs = "0.1.0"
```
或
```
download_rs = {version="0.1.0",features=["async_download"]}
```
* code
```rust
use download_rs::async_download::Download;

#[tokio::main]
async fn main() {
    let url = "https://www.baidu.com/img/bd_logo1.png";
    // 当前目录
    let filename = "bd_log1.png";
    // 指定 下载目录
    // let filename = "/download/";
    // 指定下载目录下载文件名,需要手动创建下载文件夹
    // let filename = "download/bd_log1.png";
    let download = Download{
        url,
        out: Some(filename)
    };
    match download.download_async().await {
        Err(e) => panic!("error: {}",e.to_string()),
        Ok(()) => println!("ok")
    }
}
```