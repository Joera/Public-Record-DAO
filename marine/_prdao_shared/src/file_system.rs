use std::fs;
use std::path::PathBuf;
use crate::adapters::log;

 /// You can read or write files from the file system if there is permission to use directories described in `Config.toml`.
pub fn put(path: &String, name: &String, file_content: Vec<u8>, elasticsearch_url: &String) -> String {
   
    let rpc_tmp_filepath = format!("{}{}{}", crate::TMP_DIR, path, name);

    log(elasticsearch_url, &rpc_tmp_filepath);
 
    let result = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
     
    if let Err(e) = result {

        log(elasticsearch_url, &e.to_string());
        return format!("file can't be written: {}", e);
    }

    //  let c= fs::read(PathBuf::from(rpc_tmp_filepath.clone())).unwrap();
    //  let cc = String::from_utf8_lossy(&c);
    //  println!("{:?}", cc);
 
    String::from("Ok")
}
 

pub fn get(file_name: String) -> Vec<u8> {
//     log::info!("get called with file name: {}\n", file_name);
 
    let tmp_filepath = format!("{}{}", crate::TMP_DIR, file_name);
 
    fs::read(tmp_filepath).unwrap_or_else(|_| b"error while reading file".to_vec())
}
 