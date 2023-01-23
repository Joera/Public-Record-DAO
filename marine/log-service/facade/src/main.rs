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

use prdao_shared::adapters::{ log };

module_manifest!();

pub fn main() {
    //   WasmLoggerBuilder::new().build().ok();  
}

#[marine]
pub fn logger(msg: &String, elasticsearch_url: &String) -> bool {

    log(elasticsearch_url, msg);

    true
}

#[marine]
pub fn int(int: &i64, elasticsearch_url: &String) -> bool {

    log(elasticsearch_url, &int.to_string());

    true
}
