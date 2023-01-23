/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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

use prdao_shared::types; 
use prdao_shared::adapters::{ curl_request, log};
use prdao_shared::requests::{request, check_response_string, JsonRpcResult };
use prdao_shared::eth_utils;

mod jsonrpc_helpers;
mod services;
mod store;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}

#[marine] 
pub fn service(name: String, elasticsearch_url: String) -> types::ServiceConfig {

  services::new(name, &elasticsearch_url)
}

#[marine] 
pub fn confirm(mut name: String, service_id: String, node: String, elasticsearch_url: &String) -> bool {

    name.push_str(&" created as ");
    name.push_str(&service_id.as_str());
    name.push_str(&" on ");
    name.push_str(&node.as_str());
 //   log(elasticsearch_url, &name);

    true
}

#[marine] 
pub fn error(name: String, node: String, elasticsearch_url: &String) -> bool {

    let mut msg = "Error while creating ".to_owned();
    msg.push_str(&name.as_str());
    msg.push_str(&" on ");
    msg.push_str(&node.as_str());
    log(elasticsearch_url, &msg);

    true
}
 