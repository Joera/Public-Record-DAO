use hex::{encode as hex_encode};
use ethabi::{Token, encode, ethereum_types::H256};
use ethabi_contract::use_contract;
use ethabi_derive;
use_contract!(service_store, "store.json");

use crate::{ request, check_response_string, JsonRpcResult };
use crate::jsonrpc_helpers::{ Request };
use crate::eth_utils::{
    get_nonce, 
    remove_zero_x
  };
use crate::{curl_request };

const CONTRACT: &str = "0xC86e745d27D8bc05D2703A76fe2356b16DBc16c6";


pub fn fetchCid(name: &String, elasticsearch_url: &String) -> String {

    let eth_provider_url = "https://eth-rinkeby.alchemyapi.io/v2/oDMoyeai5fTVQxfpjKQVJM3ltl1ap9z7".to_string();
    let nonce = get_nonce();

    let curl_args: Vec<String> = Request::new(
        
        eth_provider_url, 
        String::from("eth_call"), 
        nonce
    
    ).format_call(

        &CONTRACT.to_string(), 
        &format!(
        "0x{}", 
        hex_encode(
            service_store::functions::get::encode_input(
                name
            )
        )
        )
    );

    let response: JsonRpcResult = request(curl_args, nonce, elasticsearch_url);

    let result = hex::decode(remove_zero_x(response.result.to_string())).unwrap();
    
    service_store::functions::get::decode_output(&result).unwrap()

}