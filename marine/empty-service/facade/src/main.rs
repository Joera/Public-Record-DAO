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

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
 
}

// #[marine]
// #[link(wasm_import_module = "curl_adapter")]
// extern "C" {
//     pub fn curl_request(url: Vec<String>) -> MountedBinaryResult;
// }

#[marine]
#[link(wasm_import_module = "elasticsearch_adapter")]
extern "C" {
    pub fn log(elasticsearch_url: String, msg: String) -> String;
}


#[marine]
pub fn f() -> String {

    "Koe".to_string()
}
