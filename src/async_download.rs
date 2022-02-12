use tokio::{fs, io::AsyncWriteExt};
use reqwest::{Url, header};
//use tokio::prelude::*;
use std::path::{Path, PathBuf};
use tokio::runtime::Runtime;
use indicatif::{ProgressBar, ProgressStyle};

/// Download
/// url ： 下载链接
/// out: 输出目录或输出文件路径
/// proxy: 使用代理，发现 reqwest 自动使用系统代理不好用 ,
///         目前只支持 http 代理 如：`http://127.0.0.1:7890` ,不支持 https,socks5代理（懒）
pub struct Download<'a>{
     url: &'a str,
     out: Option<&'a str>,
    proxy: Option<&'a str>,
}

impl<'a> Download<'a>{

    /// 创建 Download对象
    /// url: 需要下载的url
    /// out: 保存地址（具体文件夹或具体文件名）
    /// proxy: 使用代理，发现 reqwest 自动使用系统代理不好用 ,
    ///         目前只支持 http 代理 如：`http://127.0.0.1:7890` ,不支持 https,socks5代理（懒）
    pub fn new(url:&'a str,out: Option<&'a str>,proxy: Option<&'a str>) -> Download<'a> {
        return  Download{
            url,
            out,
            proxy,
        }
    }

   /// 异步方法下载
   /// 未使用 `#[toikio::main]` 方式 而使用 `Runtime` 的 `block_on`方法
   /// 避免修改主函数
   /// # Examples
   ///
   /// ```rust
   /// use std::error::Error;
   ///use download_rs::async_download::Download;
   ///
   ///fn main() {
   ///    let url = "https://www.baidu.com/img/bd_logo1.png";
   ///    // 当前目录
   ///    let filename = "bd_log1.png";
   ///    // 指定 下载目录
   ///    // let filename = "/download/";
   ///    // 指定下载目录下载文件名,需要手动创建下载文件夹
   ///    // let filename = "download/bd_log1.png";
   ///    //let proxy = Some("http://127.0.0.1:7089");
   ///    //let download = Download::new(url,Some(filename),proxy);
   ///    let download = Download::new(url,Some(filename),None);
   ///    match download.download() {
   ///        Ok(_) => println!("下载完成"),
   ///        Err(e) => println!("下载出错 ： {}",e.to_string()),
   ///    }
   ///}
   ///
   /// ```
    pub fn download(&self) ->Result<(),  Box<dyn std::error::Error>> {
        let mut rt = Runtime::new()?;
        rt.block_on(self.download_async())?;

       Ok(())
    }

    /// 异步下载方法
    /// 参考：https://github.com/otavio/rsget.git
    /// # Example
    ///
    /// ```rust
    /// use download_rs::async_download::Download;
    /// #[tokio::main]
    ///async fn main() {
    ///    let url = "https://www.baidu.com/img/bd_logo1.png";
    ///    // 当前目录
    ///    let filename = "bd_log1.png";
    ///    // 指定 下载目录
    ///    // let filename = "/download/";
    ///    // 指定下载目录下载文件名,需要手动创建下载文件夹
    ///    // let filename = "download/bd_log1.png";
   ///    //let proxy = Some("http://127.0.0.1:7089");
   ///    //let download = Download::new(url,Some(filename),proxy);
    ///    let download = Download::new(url,Some(filename),None);
    ///    match download.download_async().await {
    ///        Err(e) => panic!("error: {}",e.to_string()),
    ///        Ok(()) => println!("ok")
    ///    }
    ///}
    ///
    /// ```
    pub async fn download_async(&self) ->Result<(),  Box<dyn std::error::Error>> {


        let mut out_dir = "";

        let path_url = Url::parse(self.url)?;
        let mut filename = path_url.path_segments().and_then(std::iter::Iterator::last).unwrap_or("tmp.bin");

        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36"));

        let client_builder = match self.proxy {
            None => reqwest::Client::builder().no_proxy(),
            Some(proxy) => reqwest::Client::builder().proxy(reqwest::Proxy::all(proxy)?),
        };

        let client =client_builder
            .default_headers(headers)
            .build()?;
        // 输出文件夹
        if let Some(output) = self.out {
            if Path::new(output).is_dir() {
                out_dir = output;
            }else {
                filename = output;
            }
        }
        let mut out_filename = PathBuf::from(out_dir);
        out_filename.push(filename);
        println!("path: {}",out_filename.display());

        let resp = client.head(self.url).send().await?;
        let total_size = resp.content_length().unwrap();
        println!("url is {}",self.url);
        println!("total_size is {}",total_size);

        // 进度条
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"));

        let  request = client.get(self.url);


        // 获取文件内容
        let mut source = request.send().await?;
        // 创建获取追加文件
        let mut dest = fs::OpenOptions::new().create(true).append(true).open(&out_filename).await?;

        while let Some(chunk) = source.chunk().await? {
            dest.write_all(&chunk).await?;
            pb.inc(chunk.len() as u64);
        }
        println!("Download of '{}' has been completed.", out_filename.to_str().unwrap());
        Ok(())
    }
}