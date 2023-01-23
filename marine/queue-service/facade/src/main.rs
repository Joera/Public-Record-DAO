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
// use marine_rs_sdk::WasmLoggerBuilder;

mod memory;
use prdao_shared::types::{Job, SubscriptionV2, DataModel};

module_manifest!();

pub fn main() {
 
}
 
#[marine]
pub fn add(id: String, subscription: SubscriptionV2, dataModel: DataModel) -> bool  {

    let job = Job {

        item: id,
        item_count: String::from(""),
        data_model: dataModel,
        subscription,
        json: String::from(""),    
        cbor: String::from(""),
        html: String::from(""),
        embed: String::from("")
    };

    memory::add(job);

    true
}

#[marine]
pub fn get() -> Vec<Job> {
    memory::get()
}

#[marine]
pub fn clear() -> bool {
    memory::clear();

    true
}