// use crate::curl_request;
// use crate::curl::{request, request_with_log};
use crate::eth_utils::{
  get_nonce, 
  gas_price_for_raw_transaction, 
  get_transaction_count, 
  address_for_from,
  address_for_to,
  EthRequest,
  format_curl_request,
  remove_zero_x
};


use crate::{ log, JsonRpcResult, check_response_string, request};

use ethers_core::types::{ Address, Signature, U256, U64, TransactionRequest, PrivateKey, Bytes as EthBytes }; // Bytes
use ethers_core::utils::serialize;
use ethers_core::secp256k1::Message;

use serde::Serialize as serde_serialize;
use serde::Deserialize as deserialize;
use serde_json::json;
use serde_json::Value;

use hex::{encode as hex_encode};
use tiny_keccak::{Keccak};

use ethabi::{Token, encode, ethereum_types::H256};
use ethabi_contract::use_contract;
use ethabi_derive;
use_contract!(ens_resolver, "resolver.json");

use slugify::slugify;

const RESOLVER: &str = "0xf6305c19e814d2a75429Fd637d01F7ee0E77d615";

pub fn get_record(eth_provider_url: String, ens_domain: String, key: String, elasticsearch_url: &String) -> String {
  
  let nonce = get_nonce();

  let curl_args: Vec<String> = EthRequest::new(
    
    eth_provider_url, 
    String::from("eth_call"), 
    nonce
  
  ).format_call(

    &RESOLVER.to_string(), 
    &format!(
      "0x{}", 
      hex_encode(
        ens_resolver::functions::text::encode_input(
          H256::from_slice(&namehash(&ens_domain.as_str())[..]), // ens_node
          slugify!(&key) // property name
        )
      )
    )
  );

  let response: JsonRpcResult = request(curl_args, nonce, elasticsearch_url );
  
  ethUnformattedDataToCid(response)
}

pub fn prepare_request(eth_provider_url: String, sender: String, ens_domain: String, key: &String, value: &String) -> String {

  let slug = slugify!(key);
  // build contract specific part of the transaction 
  let input_data_bytes: EthBytes = ens_resolver::functions::set_text::encode_input(
    
  
    H256::from_slice(&namehash(&ens_domain.as_str())[..]), // ens_node
    &slug,  // property name 
    value // property value
  
  ).into(); 
  
  // // build the transaction 
  let mut tx_request = TransactionRequest {
        from: None,
        to: Some(address_for_to(&RESOLVER)),
        gas: Some(200_000.into()),
        gas_price: Some(gas_price_for_raw_transaction(&eth_provider_url)),
        value: None,
        data : Some(input_data_bytes), 
        nonce: Some(get_transaction_count(&sender.to_string(), &eth_provider_url))
        
  };

  serde_json::to_string(&tx_request).unwrap()

} 

pub fn make_request(eth_provider_url: &String, tx_string: String, elasticsearch_url: &String) -> JsonRpcResult {
 
  let nonce = get_nonce();
  let curl_args = format_curl_request(&tx_string, eth_provider_url); 
  request(curl_args, nonce, elasticsearch_url)   
}

pub fn make_batch_request(txs: Vec<String>, eth_provider_url: &String, elasticsearch_url: &String) -> JsonRpcResult {
  
  let mut data : String = String::from("[");
  
  
  for params in txs {
      let nonce = get_nonce();
      let ftx = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_sendRawTransaction\", \"params\":[\"{}\"], \"nonce\":{}}}", params, nonce);
      data.push_str(&ftx.as_str());
      data.push_str(",");
  }

  data.pop();
  data.push_str("]");
  // let nonce = get_nonce();
  // let params = txs.iter().map(|x| x.to_string() + ",").collect::<String>();
  // let params = params.trim_end_matches(",");
  // let data = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_sendRawTransaction\", \"params\":[\"{}\"], \"nonce\":{}}}", params, nonce);

  log(elasticsearch_url, &data);
  let nonce = get_nonce();
  let curl_args = format_curl_request(&data, eth_provider_url); 
  request(curl_args, nonce, elasticsearch_url)  
}


// source: https://github.com/InstateDev/namehash-rust/blob/master/src/lib.rs
fn namehash(name: &str) -> Vec<u8> {
  let mut node = vec![0u8; 32];
  if name.is_empty() {
      return node;
  }
  let mut labels: Vec<&str> = name.split(".").collect();
  labels.reverse();
  for label in labels.iter() {
      let mut labelhash = [0u8; 32];
      Keccak::keccak256(label.as_bytes(), &mut labelhash);
      node.append(&mut labelhash.to_vec());
      labelhash = [0u8; 32];
      Keccak::keccak256(node.as_slice(), &mut labelhash);
      node = labelhash.to_vec();
  }
  node
}

fn ethUnformattedDataToCid(response: JsonRpcResult) -> String {

  let result = hex::decode(remove_zero_x(response.result.to_string())).unwrap();
  ens_resolver::functions::text::decode_output(&result).unwrap()
}