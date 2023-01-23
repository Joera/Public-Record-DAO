use std::sync::atomic::{AtomicUsize, Ordering};
use crate::adapters::{ curl_request };
use crate::requests::{ JsonRpcResult, check_response_string };
use ethers_core::types::{U256, Address, NameOrAddress, H160};
use marine_rs_sdk::marine;
//use serde::Deserialize;

// use ethabi_contract::use_contract;
// use ethabi_derive;
// use_contract!(ens_resolver, "resolver.json");

pub const JSON_RPC: &'static str = "2.0";

#[marine]
#[derive(Debug)]
pub struct EthCurlTx {
    pub jsonrpc: String,
    pub method: String,
    pub params: String,
    pub id: u64
}

#[derive(Debug)]
pub struct EthRequest {
    pub jsonrpc: String,
    pub eth_provider_url: String,
    pub method: String,
    pub id: u64,
}

impl EthRequest {
    pub fn new(eth_provider_url: String, method: String, id: u64) -> Self {
        EthRequest {
            jsonrpc: String::from(JSON_RPC),
            eth_provider_url,
            method,
            id,
        }
    }

    pub fn add_transaction(&self, params: Vec<String>) -> Vec<String> {

        let nonce = get_nonce();
        let data = format!("{{\"jsonrpc\":\"{}\",\"method\":\"{}\", \"params\":{:?}, \"id\":{}}}", self.jsonrpc, self.method, params, nonce);
        
        let args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("--data"),
            data,
            self.eth_provider_url.to_string()
        ];

        args 
    }

    // pub fn format_tx_data(&self, to: &String, params: &String) -> String {

    //     let data = format!("{{\"jsonrpc\":\"{}\",\"method\":\"{}\", \"params\":[{{\"to\":\"{}\",\"data\":\"{}\"}},\"latest\"], \"id\":{}}}", self.jsonrpc, self.method, to, params, self.id);

    //     data
    // }

    pub fn format_call(&self, to: &String, params: &String) -> Vec<String> {

        let data = format!("{{\"jsonrpc\":\"{}\",\"method\":\"{}\", \"params\":[{{\"to\":\"{}\",\"data\":\"{}\"}},\"latest\"], \"id\":{}}}", self.jsonrpc, self.method, to, params, self.id);
        
        let args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("-H"),
            String::from("Content-Type: application/json"),
            String::from("--data"),
            data,
            self.eth_provider_url.to_string()
        ];

        args 
    }


    pub fn format_tx_call(&self, data: &String) -> Vec<String> {
        
        let args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("-H"),
            String::from("Content-Type: application/json"),
            String::from("--data"),
            data.to_string(),
            self.eth_provider_url.to_string()
        ];

        args 
    }
}

// pub fn format_single_curl_tx(to: &String, data: &String) -> EthCurlTx {

//     let nonce = get_nonce();
//  //   let params = format!("[{{\"to\":\"{}\",\"data\":\"{}\"}},\"latest\"]", to, data);

//     let tx = EthCurlTx {
//         jsonrpc: String::from("2.0"),
//         method: String::from("eth_sendRawTransaction"),
//         params: data.to_string(),
//         id: nonce
//     };
        
//     tx
// }

pub fn format_curl_request(data: &String, eth_provider_url: &String) -> Vec<String> {
        
    let args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-H"),
        String::from("Content-Type: application/json"),
        String::from("--data"),
        data.to_string(),
        eth_provider_url.to_string()
    ];

    args 
}

pub static NONCE_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn get_nonce() -> u64 {
    NONCE_COUNTER.fetch_add(1, Ordering::SeqCst) as u64
}

pub fn address_for_from(readable: &str) -> H160 {

    let address = remove_zero_x(readable.to_string()).parse::<Address>().unwrap();
    address.into() 
}


pub fn address_for_to(readable: &str) -> NameOrAddress {

    let address = remove_zero_x(readable.to_string()).parse::<Address>().unwrap();
    address.into() 
}
  

pub fn get_transaction_count(account: &String, eth_provider_url: &String) -> U256 {
     
    let nonce = get_nonce();
      
    let curl_args: Vec<String> = EthRequest::new(

      eth_provider_url.to_string(), 
      String::from("eth_getTransactionCount"), 
      nonce

    ).add_transaction(vec![

      account.to_string(),
      String::from("latest")
    ]);
     
     let response = curl_request(curl_args);
     let response = String::from_utf8(response.stdout).unwrap();
     let response : JsonRpcResult = check_response_string(response, &nonce);
  //   println!("{:?}", response);

     let tx_count = remove_zero_x(response.result);

     tx_count.parse::<U256>().unwrap()
}

pub fn get_transaction_count_i64(account: &String, eth_provider_url: &String) -> i64 {
     
    let nonce = get_nonce();
      
    let curl_args: Vec<String> = EthRequest::new(

      eth_provider_url.to_string(), 
      String::from("eth_getTransactionCount"), 
      nonce

    ).add_transaction(vec![

      account.to_string(),
      String::from("latest")
    ]);
     
     let response = curl_request(curl_args);
     let response = String::from_utf8(response.stdout).unwrap();
     let response : JsonRpcResult = check_response_string(response, &nonce);
  //   println!("{:?}", response);

     let tx_count = remove_zero_x(response.result);

     hex_to_int(tx_count)
}

pub fn gas_price_for_raw_transaction(eth_provider_url: &String) -> U256 {

    let response: JsonRpcResult = gas_price(eth_provider_url);
    
    hex_to_int(response.result).into()
}

pub fn gas_price(eth_provider_url: &String) -> JsonRpcResult {

    let nonce = get_nonce();
     
    let curl_args: Vec<String> = EthRequest::new(
      
      eth_provider_url.to_string(), 
      String::from("eth_gasPrice"),
      nonce
    ).add_transaction(vec![

    ]);

    let response = curl_request(curl_args);
    let response = String::from_utf8(response.stdout).unwrap();
    check_response_string(response, &nonce)
}
 

pub fn hex_to_int(hd: String) -> i64 {
    i64::from_str_radix(remove_zero_x(hd).as_str(), 16).unwrap()
}

pub fn remove_zero_x(s: String) -> String {
    let mut i = 0;
    let mut numbah: String = "".to_string();
    for n in s.chars() {
        if i > 1 { numbah.push(n); }
        i = i + 1;
    }
    numbah
}


