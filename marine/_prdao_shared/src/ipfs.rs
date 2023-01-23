use crate::adapters::curl_request;
use crate::adapters::log;
// use crate::adapters::{fs_put};
// use crate::file_system; 

use std::error::Error;
use crate::types::{IpfsLink, CidObject, DagObject, VoteDataObject, Job, IpldDirectory, PublicDaoObject };
use serde_json::Value;


pub fn cat(cid: &str, remote_ipfs: &String) -> String {

    let url = format!("{}:8080/api/v0/cat/{}", &remote_ipfs, cid);

    // println!("{:?}", url);

    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("GET"),
        url
    ];

    let response = curl_request(curl_args);
  
    String::from_utf8(response.stdout).unwrap()
}


pub fn dag_get(cid: &str, remote_ipfs: &String) -> String {

    // + outputs data .. straight into variable
    // - does nmot follow a path 

    let url = format!("{}:5001/api/v0/dag/get?arg={}&output-codec=dag-json", &remote_ipfs, cid);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        url
    ];
  
    let response = curl_request(curl_args);
  
    String::from_utf8(response.stdout).unwrap()
}

pub fn dag_put(v: DagObject, codec: &str, remote_ipfs: &String) -> String {


    let v = serde_json::to_value(obj).unwrap();

    let url = format!("{}:5001/api/v0/dag/put?store-codec={}&pin=true", remote_ipfs,codec.to_string());

    let data_string = format!("file={}", v);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];
  
    let response = curl_request(curl_args);
  
    String::from_utf8(response.stdout).unwrap()
}

pub fn dag_stat(cid: &String, remote_ipfs: &String) -> String {

    let url = format!("{}:5001/api/v0/dag/stat?arg={}", remote_ipfs, cid);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        url
    ];
  
    let response = curl_request(curl_args);
  
    String::from_utf8(response.stdout).unwrap()
}


// impl dag_put als trait on the VoteDataObject struct ?? 
pub fn dag_put_vote(obj: VoteDataObject, codec: &str, remote_ipfs: &String) -> Result<String,Box<dyn Error>> {

    let v = serde_json::to_value(obj).unwrap();

    let url = format!("{}:5001/api/v0/dag/put?store-codec={}&pin=true", remote_ipfs,codec.to_string());

    let data_string = format!("file={}", v);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];
  
    let response = curl_request(curl_args);
  
    let response = extract_cid(&String::from_utf8(response.stdout).unwrap());

    Ok(response)
}


pub fn dag_put_serde_value(obj: Value, codec: &str, remote_ipfs: &String) -> Result<String,Box<dyn Error>> {

    let url = format!("{}:5001/api/v0/dag/put?store-codec={}&pin=true", remote_ipfs, codec.to_string());

    let data_string = format!("file={}", obj);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];
  
    let response = curl_request(curl_args);
  
  //  let response = extract_cid(&String::from_utf8(response.stdout).unwrap());
    let response = String::from_utf8(response.stdout).unwrap();

    Ok(response)
}


pub fn data_put(obj: String, codec: &str, remote_ipfs: &String) -> String {


    let v = serde_json::to_value(obj).unwrap();

    let url = format!("{}:5001/api/v0/dag/put?store-codec={}&pin=true", remote_ipfs,codec.to_string());

    let data_string = format!("file={}", v);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];
  
    let response = curl_request(curl_args);
  
    String::from_utf8(response.stdout).unwrap()
}

pub fn car_get(cid: &str, remote_ipfs: &String) -> String {

    // currently no rust implementation to pak / unpack car files 

    let url = format!("{}:5001/api/v0/dag/export?arg={}", &remote_ipfs, cid);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        url,
        String::from("-o"),
        String::from("./tmp-html/dag.car")
    ];
  
    let response = curl_request(curl_args);
  
    String::from_utf8(response.stdout).unwrap()
}

pub fn block_get(cid: &str, remote_ipfs: &String) -> String {

    // + works with path 
    // - in case of file .. its returns file headers (?)  -- can be trimmed of 
    // - may work with 

    let url = format!("{}:5001/api/v0/block/get?arg={}", &remote_ipfs, cid);
  
    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        url
    ];
  
    let response = curl_request(curl_args);
  
    // is this trim generic .. or depended on the file itself?  OR THE NODE ??? 
    String::from_utf8_lossy(&response.stdout[8..(response.stdout.len() - 3)]).to_string()
}

pub fn add_file(data: &String, file_name: &String, remote_ipfs_url: &String, elasticsearch_url: &String) -> Result<String,Box<dyn Error>> {
    
  //  store_as_file_in_temp(file_name, data, elasticsearch_url);

    // let r = fs_put(&crate::TMP_DIR.to_string(), &file_name, data.as_bytes().to_vec());

    // log(elasticsearch_url, &r);

    // is de filename nodig .. of kun je in je folder structuur de filename gebruiken als "name"? 
    // of met IpfsLink ?? 
  
    let file_string = format!("file={:?}", data);

    // log(elasticsearch_url, &file_string);
  
    let url = format!("{}:5001/api/v0/add", remote_ipfs_url);
  
    let curl_args = vec![
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        file_string.clone(),
        url
    ];

    let response = curl_request(curl_args);

 //   log(&elasticsearch_url, &String::from_utf8(response.stderr).unwrap());
    
    let response = extract_hash(&String::from_utf8(response.stdout).unwrap());

    Ok(response) // file_string
    
}

pub fn add_file_returning_object(data: &String, file_name: &String, remote_ipfs_url: &String, elasticsearch_url: &String) -> Result<Value,Box<dyn Error>> {
    
      let file_string = format!("file={:?}", data);
  
   //   log(elasticsearch_url, &file_string);
    
      let url = format!("{}:5001/api/v0/add", remote_ipfs_url);
    
      let curl_args = vec![
          String::from("-X"),
          String::from("POST"),
          String::from("-F"),
          file_string.clone(),
          url
      ];
  
      let response = curl_request(curl_args);
  
   //   log(&elasticsearch_url, &String::from_utf8(response.stderr).unwrap());
      Ok(serde_json::from_str(&String::from_utf8(response.stdout).unwrap()).unwrap()) // file_string
      
  }

pub fn extract_hash (response: &String) -> String {

    let v : serde_json::Value = serde_json::from_str(response).unwrap();

    v["Hash"].as_str().unwrap().to_string()

}

pub fn extract_cid (response: &String) -> String {

    let v : serde_json::Value = serde_json::from_str(response).unwrap();

    v["Cid"]["/"].as_str().unwrap().to_string()

}

pub fn format_link (response: &String) -> IpfsLink {

    let data : serde_json::Value = serde_json::from_str(response).unwrap();
    
    let obj = data.as_object().unwrap();

    let Hash = CidObject {
        Cid: remove_escaped_quote(obj.get("Hash").unwrap().to_string())
    };

    let Tsize = remove_escaped_quote(obj.get("Size").unwrap().to_string()).parse::<i32>().unwrap();

    IpfsLink {

        Name: remove_escaped_quote(obj.get("Name").unwrap().to_string()),
        Tsize,
        Hash
    }
}


// for file 
pub fn format_link_with_name (data: &Value, Name: String) -> IpfsLink {

//    let data : serde_json::Value = serde_json::from_str(response).unwrap();
    
    let obj = data.as_object().unwrap();

    let Hash = CidObject {
        Cid: remove_escaped_quote(obj.get("Hash").unwrap().to_string())
    };

    let Tsize = remove_escaped_quote(obj.get("Size").unwrap().to_string()).parse::<i32>().unwrap();

    IpfsLink {

        Name,
        Tsize,
        Hash
    }
}

pub fn format_link_for_node (cid: String, Name: String) -> IpfsLink {
    
        IpfsLink {
    
            Name,
            Tsize: 0,
            Hash: CidObject {
                Cid: cid
            }
        }
}

fn remove_escaped_quote(string: String) -> String {

    string.replace("\"","").to_string()
    
}

// fn store_as_file_in_temp(file_name: &String, content: &String, elasticsearch_url: &String) -> () {
  
//     fs_put(&"".to_string(), file_name, content.as_bytes().to_vec(), elasticsearch_url);
// }

pub fn filter_links(mut root: DagObject, name: String) -> DagObject {

    let mut i = 0;
    while i < root.Links.len() {
        if root.Links[i].Name == name {
            root.Links.remove(i);
        } else {
            i += 1;
        }
    }

    root
}