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
    fs::remove_file(filename).unwrap();
}