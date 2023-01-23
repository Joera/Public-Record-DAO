#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::WasmLoggerBuilder;
use marine_rs_sdk::MountedBinaryResult;
// use serde_json;
use chrono::prelude::*;

//marine_rs_sdk::module_manifest!();

fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
#[link(wasm_import_module = "curl_adapter")]
extern "C" {
    pub fn curl_request(url: Vec<String>) -> MountedBinaryResult;
}


#[marine]
pub fn log(elasticsearch_url: String, msg: String) -> String {

    let utc: DateTime<Utc> = Utc::now();

    let meta = marine_rs_sdk::get_call_parameters();

    // serde_json::to_string(&msg).unwrap()
    let data = format!("{{\"timestamp\":{:?},\"msg\":{:?},\"call_params\":{{\"init_peer_id\":{:?},\"service_id\":{:?},\"service_creator_peer_id\":{:?},\"host_id\":{:?},\"particle_id\":{:?},\"tetraplets\":{:?}}}}}", utc.to_string(), &msg, meta.init_peer_id, meta.service_id, meta.service_creator_peer_id, meta.host_id, meta.particle_id, serde_json::to_string(&meta.tetraplets).unwrap());

    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-H"),
        String::from("Content-Type: application/json"),
        String::from("--data"),
        data,
        elasticsearch_url
    ];
    
    let response  = curl_request(curl_args);
    
    String::from_utf8(response.stdout).unwrap()

}
