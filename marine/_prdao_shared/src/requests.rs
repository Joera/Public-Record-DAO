
// use crate::eth_utils::{check_response_string };
use crate::adapters::log;
use crate::adapters::curl_request;

use marine_rs_sdk::marine;
use serde_json::Value;

pub(crate) type Result<T> = std::result::Result<T, T>;

pub const JSON_RPC: &'static str = "2.0";

#[marine]
#[derive(Debug)]
pub struct JsonRpcResult {
    pub jsonrpc: String,
    pub result: String,
    pub error: String,
    pub id: u64,
}

impl From<Result<String>> for JsonRpcResult {
    fn from(result: Result<String>) -> Self {
        let jsonrpc = JSON_RPC.into();
        match result {
            Ok(res) => {
                println!("{:?}", res);
                let result_obj: Value = serde_json::from_str(&res).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();
                let result: String = serde_json::from_value(result_obj["result"].clone()).unwrap();

                Self {
                    jsonrpc,
                    id,
                    result,
                    error: "".to_string(),
                }
            }
            Err(err) => {
                println!("{:?}", err);
                let result_obj: Value = serde_json::from_str(&err).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();

                Self {
                    jsonrpc,
                    id,
                    result: "".to_string(),
                    error: err,
                }
            }
        }
    }
}

pub fn check_response_string(response: String, id: &u64) -> JsonRpcResult {
    if response.len() == 0 {
        let err_msg = "{\"jsonrpc\":\"$V\",\"id\":$ID,\"error\":{\"code\":-32700,\"message\":Curl connection failed}}";
        let err_msg = err_msg.replace("$ID", &id.to_string());
        return JsonRpcResult::from(Result::from(Err(err_msg)));
    }

    match response.contains("error") {
        true => JsonRpcResult::from(Result::from(Err(response))),
        false => JsonRpcResult::from(Result::from(Ok(response))),
    }
}


pub fn request(args: Vec<String>, id: u64, elasticsearch_url: &String) -> JsonRpcResult {

    let response = curl_request(args);
    let response = String::from_utf8(response.stdout).unwrap();
    log(elasticsearch_url, &response);
    check_response_string(response, &id)
} 