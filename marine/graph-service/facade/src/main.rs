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

use prdao_shared::types::{ ContactInfo, DataModel, SubscriptionV2, IpldObject, DagObject, IpfsLink, CidObject, VoteDataObject};
use prdao_shared::adapters::{ curl_request, log } ;
use prdao_shared::ipfs;
// use prdao_shared::mock;
use hex;
use serde_json;
use base64::encode;

mod graph;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}

#[marine]
pub fn get(event_data: String, data_model: DataModel, sub: SubscriptionV2, elasticsearch_url: String, remote_ipfs_url: &String) -> IpldObject {

    log(&elasticsearch_url, &event_data);

    let h = hex::encode([event_data.parse::<u8>().unwrap()]);

    let mut vote_id : String = "appAddress:".to_owned();
    vote_id.push_str(sub.contract.as_str());
    vote_id.push_str("-vote:0x");
    vote_id.push_str(&h.as_str());   // &h[1..

    let mut msg: String = "new item for ".to_owned();
    msg.push_str(&sub.name.as_str());
    msg.push_str("with id: ");
    msg.push_str(&vote_id.as_str());
    log(&elasticsearch_url, &msg);

    let data = graph::get_subgraph(vote_id.clone(), sub.clone());

    log(&elasticsearch_url, &data);

    let browser_file_cid = ipfs::add_file(&data, &"data.json".to_string(), remote_ipfs_url, &elasticsearch_url).unwrap();

    let mut voteDataObject: VoteDataObject = serde_json::from_str(&data).unwrap();

    let cbor_cid = ipfs::dag_put_vote(voteDataObject, "dag-cbor", remote_ipfs_url).unwrap();

    let object =  IpldObject {
        item: vote_id,
        item_count: event_data,
        data_model: data_model,
        subscription: sub,
        json: browser_file_cid,    
        cbor: cbor_cid,
        html: "".to_string(),
        embed: "".to_string()
    };


    log(&elasticsearch_url, &serde_json::to_string(&object).unwrap());

    object
}


// test with two different codexes
// one as file .. that works in the browser
// one that can link in ipld structure
// #[marine]
// pub fn test(event_data: String) -> IpldObject {

//     let h = hex::encode([event_data.parse::<u8>().unwrap()]);

//     let mut vote_id : String = "appAddress:".to_owned();
//     vote_id.push_str(mock::subscription().contract.as_str());
//     vote_id.push_str("-vote:0x");
//     vote_id.push_str(&h[1..]);

//     // let id = String::from("appAddress:0x00047774fb8ab777841c394a910ad0da99d13254-vote:0xa");

//     let data = graph::get_subgraph(vote_id, mock::subscription());

//     let browser_file_cid = ipfs::add_file(&data, &"data.json".to_string(), &"http://64.227.70.116".to_string()).unwrap();

//     let mut voteDataObject: VoteDataObject = serde_json::from_str(&data).unwrap();

//     let cbor_cid = ipfs::dag_put_vote(voteDataObject, "dag-cbor", &"http://64.227.70.116".to_string());
    

//     let object =  IpldObject {
//         data_model: mock::data_model(),
//         subscription: mock::subscription(),
//         json: browser_file_cid,    
//         cbor: cbor_cid,
//         html: "".to_string(),
//         embed: "".to_string()
//     };


//     object
// }

