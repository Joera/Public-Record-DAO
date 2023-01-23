use serde::{Serialize, Deserialize};
use serde_json;
use crate::types::{ServiceConfig, AdapterConfig};
use crate::{log, store};

pub fn new(name: String, elasticsearch_url: &String) -> ServiceConfig {
    
    configurate(store::fetchCid(&name, elasticsearch_url).to_string(), format!("{}_facade", &name), name, elasticsearch_url)
}

fn configurate(cid:String, facade_name: String, name: String, elasticsearch_url: &String) -> ServiceConfig {

   let mut adapters: Vec<AdapterConfig> = vec!();

    match name.as_str() {

        "prdao_signer_service" => {}
        "prdao_fs_service" => {}
        "prdao_test_service" => {}
        _ => { 

            let curl_cid = store::fetchCid(&"prdao_curl_adapter".to_string(), elasticsearch_url);

            adapters.push(
                AdapterConfig::new("curl_adapter".to_string(), curl_cid, 100, true, true)
            );

            let elastic_cid = store::fetchCid(&"prdao_elasticsearch_adapter".to_string(), elasticsearch_url);

            adapters.push(AdapterConfig::new("elasticsearch_adapter".to_string(), elastic_cid, 10, true, false));  
            
            // let local_storage_cid = store::fetchCid(&"prdao_localstorage_adapter".to_string());
            // adapters.push(AdapterConfig::new("localstorage_adapter".to_string(), local_storage_cid, 100, false, false)); 
        }
    }

    let config = ServiceConfig {
        adapters,
        cid,
        facade_name,
        name
    };

  //  log(elasticsearch_url, &serde_json::to_string(&config).unwrap());

    config
}

