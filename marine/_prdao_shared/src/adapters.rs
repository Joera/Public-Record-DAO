use marine_rs_sdk::marine;
// use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
// use marine_rs_sdk::WasmLoggerBuilder;


#[marine]
#[link(wasm_import_module = "curl_adapter")]
extern "C" {
    pub fn curl_request(url: Vec<String>) -> MountedBinaryResult;
}

#[marine]
#[link(wasm_import_module = "elasticsearch_adapter")]
extern "C" {
    pub fn log(elasticsearch_url: &String, msg: &String) -> String;
}