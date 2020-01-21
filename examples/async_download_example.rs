//! `cargo run --example async_download_example`
//! `cargo run --example async_download_example --features="async_download"`

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
    let download = Download{
        url,
        out: Some(filename)
    };
    match download.download() {
        Ok(_) => println!("下载完成"),
        Err(e) => println!("下载出错 ： {}",e.to_string()),
    }

    // 删除图片
    fs::remove_file(filename).unwrap();
}