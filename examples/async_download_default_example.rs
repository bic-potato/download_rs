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