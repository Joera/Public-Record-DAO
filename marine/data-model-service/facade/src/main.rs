#![allow(
    non_snake_case,
    unused_variables,
    unused_imports,
    unused_parens,
    unused_mut
)]

mod temp;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use prdao_shared::adapters::log;
use prdao_shared::types::{ContactInfo, DataModel};


module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}

#[marine]
pub fn get(index: u16, elasticsearch_url: &String) -> DataModel {

    let datamodel = temp::get(index);
    let mut msg: String = "found data_model for : ".to_owned();
    msg.push_str(&datamodel.name.as_str());
    log(elasticsearch_url, &msg);
    datamodel
}
