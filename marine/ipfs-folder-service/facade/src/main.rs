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
use prdao_shared::types::{ ContactInfo, Subscription, DataModel, IpldObject };

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
pub fn process(html: String, object: IpldObject, data_tip: String, remote_ipfs_url: String, elasticsearch_url: String) -> IpldObject {

    // get current html folder cid from data_tip 
    // ipfs dag get bafyreiearzijdw2zgf525jguzwuiwpzc32myltivgt7vpkjnkgei46ceue/templates/governance/protocols/random-electionator-2/items/\"144\"

    // what if new ?

    // download default folder in temp    // combine template and download folder ??
    // replce html 
    // upload folder
    // return new cid;
    cid
}
