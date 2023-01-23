#![allow(
    non_snake_case,
    unused_variables,
    unused_imports,
    unused_parens,
    unused_mut
)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

use prdao_shared::adapters::{ curl_request, log };
// use prdao_shared::requests::{request, check_response_string, JsonRpcResult };

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
 
}



#[marine]
pub fn update(item_id: String, cid: String) -> String {

    let data_string = format!("{{\"type\":\"TXT\",\"data\":\"dnslink=/ipfs/{}\"}}", cid);
    log(&String::from("http://178.128.245.19:9200/fluence/_doc/?pretty"), &data_string);

    let url = format!("https://api.digitalocean.com/v2/domains/public-record.org/records/{}", item_id);

    let curl_args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("PUT"),
            String::from("-H"),
            String::from("Content-Type: application/json"),
            String::from("-H"),
            String::from("Authorization: Bearer fbc4f51df12193f40c49e91071b8c7fea8dddd558ce2b5a5360e2ce94a792414"),
            String::from("--data"),
            data_string,
            url
    ];

    let response = curl_request(curl_args);
     
    let response = String::from_utf8(response.stdout).unwrap();

    log(&String::from("http://178.128.245.19:9200/fluence/_doc/?pretty"), &response);

    response
}
