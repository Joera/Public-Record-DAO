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

use prdao_shared::adapters::{ curl_request, log};
use prdao_shared::types::{ SubscriptionV2, Job };
use prdao_shared::constants::{ELASTICSEARCH_URL};
// use prdao_shared::marine::log;

mod filter;
mod request_formats;




module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}

#[marine] 
pub fn poll(mut job: Job) -> Vec<Job> {

    let results = filter::poll(&job);
    
    let mut msg: String = results.iter().count().to_string(); 
    msg.push_str(" results for ");
    msg.push_str(job.subscription.name.as_str());
    msg.push_str(" with filter: ");
    msg.push_str(&job.filter.as_str());
    log(&ELASTICSEARCH_URL.to_string(), &msg);

    results 
}


#[marine] 
pub fn new(mut job: Job) -> Job {

    job.filter = filter::new(&job.subscription);

    let mut msg: String = "new_filter: ".to_owned();
    msg.push_str(&job.filter.as_str());
    msg.push_str(" for ");
    msg.push_str(job.subscription.name.as_str());
    log(&ELASTICSEARCH_URL.to_string(), &msg);

    job
}