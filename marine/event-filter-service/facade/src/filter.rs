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


use crate::{curl_request,log, SubscriptionV2, Job, ELASTICSEARCH_URL};
use prdao_shared::requests::{ request, JsonRpcResult};
use prdao_shared::eth_utils::{ get_nonce };
use prdao_shared::eth_providers::{match_eth_provider};

use crate::request_formats::{ Request};
use serde_json::Value;

pub fn poll(job: &Job) -> Vec<Job> {

  let eth_provider = match_eth_provider(&job.subscription.network);

  let mut response_array = vec!();
  
  let method = String::from("eth_getFilterChanges");
  let nonce = get_nonce();

  let curl_args: Vec<String> = Request::new(eth_provider, method, nonce).filter_changes(
    &String::from(job.filter.clone())
  );


  // result is an array .. so getting a JsonRpcResult with fce_results won't work here
  let response = curl_request(curl_args);
  let response = String::from_utf8(response.stdout).unwrap();
  let response: serde_json::Value = serde_json::from_str(&response).unwrap();
  
  if !response["result"].as_array().is_none() {
    for e in response["result"].as_array().unwrap() {

      // from json result to raw_log type

      // DIT WERKTE NIET >>  KREEG INVALID DATA 

      // let mut topics: Vec<Hash> = vec!();

      // for topic in e["topics"].as_array().unwrap() {
      //   let mut t: [u8; 32] = Default::default();
      //   t.copy_from_slice(&topic.to_string().as_bytes()[0..32]);
      //   let hash: Hash = t.into();
      //   topics.push(hash); // t moet H256 zijn 
      // }

  //    let data_bytes = e["data"].to_string()).as_bytes().to_vec();

      // let raw_log = RawLog {
      //     topics: topics,
      //     data: data_bytes.into() // moet Bytes zijn 
      // };

    // use_contract!(contract, "abi-aragon-voting.json");
      
    //   let event = contract::events::cast_vote::parse_log(raw_log); // .unwrap();

    //   match event {
    //     Ok(v) => {     
    //       log(elasticsearch_url, &v.vote_id.to_string());
    //     },
    //     Err(e) => {  
    //       log(elasticsearch_url, &e.to_string());
    //     }
    //   }

      let mut augmented_job = job.clone();
      augmented_job.item = extract_id(e["topics"][1].to_string());
      
      response_array.push(augmented_job);
    }
  }

  response_array
 }


 pub fn new(sub: &SubscriptionV2) -> String {

  let eth_provider = match_eth_provider(&sub.network);

  let method = String::from("eth_newFilter");
  let from_block = String::from("0x1");
  let nonce = get_nonce();

  let curl_args  = Request::new(eth_provider, method, nonce).new_filter(
      &from_block, 
      &sub.contract, 
      &sub.topics,
      &ELASTICSEARCH_URL.to_string()
    );

  let response: JsonRpcResult = request(curl_args, nonce, &ELASTICSEARCH_URL.to_string());

  // log(elasticsearch_url, &response.error.to_string());

  response.result.to_string()
 }




fn extract_id(event_data: String) -> String {

  i64::from_str_radix(&event_data.as_str()[3..67],16).unwrap().to_string()
    
}