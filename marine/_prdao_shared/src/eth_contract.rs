// use ethabi_contract::use_contract;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use ethabi::{Contract, Function, Int, Token};
use ethabi::ethereum_types::U256;
use crate::ipfs;
use crate::eth_utils::{
    EthRequest,
    get_nonce, 
    remove_zero_x
  };
use hex::{encode as hex_encode};
use crate::requests::{ request, check_response_string, JsonRpcResult };

use serde_json::Value;

#[derive(Debug)]
pub struct EthContract {
    pub provider: String,
    pub address: String,
    pub contract: Contract,
}

impl EthContract {
    pub fn new(_eth_provider_url: String, _contract_address: String, cid: &str) -> Self {

        let contract_code: Value = serde_json::from_str(&ipfs::dag_get(cid, &String::from("http://64.227.70.116"))).unwrap();

        // println!("{:?}", contract_code);

        let abi = serde_json::to_string(&contract_code["abi"]).unwrap();

        let contract : Contract = serde_json::from_str(abi.as_str()).unwrap();

     //   println!("{:?}", contract);

        EthContract {
            provider: _eth_provider_url,
            address: _contract_address,
            contract: contract
        }
    }

    pub fn read(&self, function_name: &String, arg: u8, elasticsearch_url: &String) ->  Result<String,Box<dyn Error>> {

        let nonce = get_nonce();

        let function: &Function = self.contract.function(function_name).unwrap();

        let token = ethabi::Token::Uint(U256::from(arg));

        let curl_args: Vec<String> = EthRequest::new(
            
            self.provider.clone(), 
            String::from("eth_call"), 
            nonce
        
        ).format_call(
    
            &self.address, 
            &format!(
            "0x{}", 
            hex_encode(
                function.encode_input(&[token]).unwrap()
            )
            )
        );
    
        let response: JsonRpcResult = request(curl_args, nonce, elasticsearch_url);
    
        let result = hex::decode(remove_zero_x(response.result.to_string())).unwrap();
        
        let tokens = function.decode_output(&result).unwrap();

      //  println!("{:?}", tokens[0].to_string());

        Ok(tokens[0].to_string())
    }
}