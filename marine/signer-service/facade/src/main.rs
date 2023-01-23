#![allow(
    non_snake_case,
    unused_variables,
    unused_imports,
    unused_parens,
    unused_mut
)]

mod keccak;
mod auth;
mod keys;
mod memory;
mod transactions;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

use marine_rs_sdk::CallParameters;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}


#[marine]
pub fn initiate() -> String {
    
    if memory::unitialized() {
        // store particle id or reference to recurring aqua script 
        keys::new()
    } else {
        String::from("this service can be initialized only once.")
    }
}

#[marine]
pub fn initiate_dev() -> Vec<String> {
    
    if memory::unitialized() {
        // store particle id or reference to recurring aqua script 
        keys::new_dev()
    } else {
        vec!(String::from("this service can be initialized only once."))
    }
}

#[marine]
pub fn from_secret(secret: String) -> Vec<String> {
    
    if memory::unitialized() {
        keys::from_secret(secret)
    } else {
        vec!(String::from("this service can be initialized only once."))
    }
}

#[marine]
pub fn init_with_peer(secret: String) -> bool {

    // so when i store in mem ... i can easily restart .. and we know it is always me. 
    // i can then also init forwarder contract  
    // to only accept 
    // fuck! it is all public 
}

#[marine]
pub fn sign_tx(tx_request_string: &str, chain_id: u64) -> String {
    // only the recurring aqua script ... how to check? particle? 
    transactions::sign(tx_request_string, chain_id)
}

#[marine] 
pub fn address() -> String {

    memory::address()
}



