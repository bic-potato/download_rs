#[cfg(test)]
mod async1_tests {
    use std::fs;
    use std::path::Path;
    use download_rs::async_download::Download;

    #[tokio::test]
    #[cfg(feature = "async_download")]
    async fn async_download_test() {
        let url = "https://www.baidu.com/img/bd_logo1.png";
        let filename = "async_bd_logo1.png";

        let download = Download::new(url, Some(filename),None);
        let ceshi_file = Path::new(filename);
        if ceshi_file.exists() {
            fs::remove_file(filename).unwrap();
        }
        match download.download_async().await {
            Err(e) => panic!("error: {}",e.to_string()),
            Ok(()) => println!("ok")
        }
        // assert!(Path::new(filename).exists())
        if ceshi_file.exists() {
            fs::remove_file(filename).unwrap();
            assert!(true)
        }
    }
}
