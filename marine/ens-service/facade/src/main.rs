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

use ethers_core::types::{U256 };
use prdao_shared::adapters::{ curl_request, log };
use prdao_shared::requests::{request, check_response_string, JsonRpcResult };
use prdao_shared::eth_utils;


mod ens;


module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}

#[marine]
pub fn get_record(eth_provider_url: String, ens_domain: String, key: String, elasticsearch_url: String) -> String {
    
    let mut msg: String = "get ens record: ".to_owned();
    msg.push_str(&key.as_str());
 //   log(&elasticsearch_url, &msg);
    ens::get_record(eth_provider_url, ens_domain, key, &elasticsearch_url)
}

#[marine]
pub fn prepare_request(eth_provider_url: String, sender: String, ens_domain: String, key: String, value: String, elasticsearch_url: String) -> String {
   
    let request = ens::prepare_request(eth_provider_url, sender, ens_domain, &key, &value);

    let mut msg: String = "prepared ens request for : ".to_owned();
    msg.push_str(&value.as_str());
    msg.push_str("@");
    msg.push_str(&key.as_str());
    log(&elasticsearch_url, &request);

    request
}   


// #[marine]
// pub fn prepare_request_with_u64(eth_provider_url: String, sender: String, ens_domain: String, key: String, value: u64, elasticsearch_url: String) -> String {

//     log(&elasticsearch_url, &"prepare ens request".to_string());
//     ens::prepare_request(eth_provider_url, sender, ens_domain, &key, &value.to_string())
// }


// #[marine]
// pub fn format_tx(params: &String, elasticsearch_url: &String) -> EthCurlTx {
//     ens::format_tx(params)
// }

#[marine] 
pub fn get_tx_count(sender: String, eth_provider_url: String, elasticsearch_url: &String ) -> i64 {
    log(&elasticsearch_url, &String::from("hi"));
    let count: i64 = eth_utils::get_transaction_count_i64(&sender, &eth_provider_url).into();
    log(&elasticsearch_url, &count.to_string());

    count
}

#[marine]
pub fn make_request(eth_provider_url: String, tx_string: String, elasticsearch_url: String) -> JsonRpcResult {

    let mut msg: String = "making ens request with ".to_owned();
    msg.push_str(&tx_string.as_str());
    log(&elasticsearch_url, &msg);

    let rpcResult = ens::make_request(&eth_provider_url, tx_string, &elasticsearch_url);

    log(&elasticsearch_url, &serde_json::to_string(&rpcResult.error).unwrap());
    log(&elasticsearch_url, &serde_json::to_string(&rpcResult.result).unwrap());

    rpcResult
}

#[marine]
pub fn make_batch_request(tx_strings: Vec<String>, eth_provider_url: &String, elasticsearch_url: &String) -> JsonRpcResult {

    let mut msg: String = "about to make batch request ".to_owned();
    log(elasticsearch_url, &msg);

    let rpcResult = ens::make_batch_request(tx_strings, eth_provider_url, elasticsearch_url);

    log(&elasticsearch_url, &serde_json::to_string(&rpcResult.error).unwrap());
    log(&elasticsearch_url, &serde_json::to_string(&rpcResult.result).unwrap());

    rpcResult


}
