use std::fs;
use std::path::{Path, PathBuf};
use reqwest::{Url, header};

/// Download
/// url ： 下载链接
/// out: 输出目录或输出文件路径
pub struct Download<'a>{
    pub url: &'a str,
    pub out: Option<&'a str>
}

impl<'a> Download<'a>{
    /// 同步方法下载
   /// 没有下载进度条
   /// # Examples
   ///
   /// ```rust
   /// use download_rs::sync_download::Download;
    ///use std::error::Error;
    ///
    ///fn main() {
    ///    let url = "https://www.baidu.com/img/bd_logo1.png";
    ///    // 当前目录
    ///     let filename = "bd_log1.png";
    ///    // 指定 下载目录
    ///    // let filename = "/download/";
    ///    // 指定下载目录下载文件名,需要手动创建下载文件夹
    ///    // let filename = "download/bd_log1.png";
    ///    let download = Download{
    ///        url,
    ///        out: Some(filename)
    ///    };
    ///    match download.download() {
    ///        Ok(_) => println!("下载完成"),
    ///        Err(e) => println!("下载出错 ： {}",e.to_string()),
    ///    }
    /// }
   ///
   /// ```
    pub fn download(&self) ->Result<(), Box<dyn std::error::Error>> {

        let mut out_dir = "";

        let path_url = Url::parse(self.url)?;
        let mut filename = path_url.path_segments().and_then(std::iter::Iterator::last).unwrap_or("tmp.bin");

        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36"));
        let client =reqwest::blocking::Client::builder()
            .default_headers(headers)
            //.no_proxy()
            .build()?;

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

        let mut resp = client.get(self.url).send()?;
        let total_size = resp.content_length().unwrap();
        println!("url is {}",self.url);
        println!("total_size is {}",total_size);

        // 创建获取追加文件
        let mut dest = fs::OpenOptions::new().create(true).append(true).open(&out_filename)?;

        std::io::copy(&mut resp,&mut dest)?;

        Ok(())
    }
}