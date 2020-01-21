#[cfg(test)]
mod sync_tests {
    use std::path::Path;
    use std::fs;
    use download_rs::sync_download::Download;

    #[test]
    #[cfg(feature = "sync_download")]
    fn sync_download_test() {
        let url = "https://www.baidu.com/img/bd_logo1.png";
        let filename  = "sync_bd_logo1.png";
        let download = Download::new(url,Some(filename),None);
        let ceshi_file = Path::new(filename);
        if ceshi_file.exists() {
            fs::remove_file(filename).unwrap();
        }
        match download.download(){
            Err(e) => panic!("error: {}",e.to_string()),
            Ok(()) => println!("ok")
        }
        if ceshi_file.exists() {
            fs::remove_file(filename).unwrap();
            assert!(true)
        }
    }

}
