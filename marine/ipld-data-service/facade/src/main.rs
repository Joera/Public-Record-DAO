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
use serde::{ Deserialize, Serialize};
use serde_json::Value;

use prdao_shared::types::{ ContactInfo, SubscriptionV2, DataModel, IpldObject, DagObject, CidObject, IpfsLink, PublicMetadata, IpldDirectory, IpldDir, PublicDaoObject };
use prdao_shared::adapters::{ curl_request, log };
use prdao_shared::ipfs;


mod ipld; 

module_manifest!();

pub fn main() {
 //   WasmLoggerBuilder::new().build().ok();
 
}

#[marine]
pub fn update(tip: String, objects: Vec<IpldObject>, elasticsearch_url: &String, remote_ipfs: &String) -> String {

  let mut voteLinks: Vec<IpfsLink> = vec!();
  let mut dao_name: String = "".to_string();

  // let mut leaf2 : String;
  let mut heads: Vec<String> = vec!();
  heads.push(tip);

  for object in objects {

    let leaf0 = ipld::create_widget_leaf(&object, remote_ipfs, elasticsearch_url);

    let lineage = ipld::build_lineage(&heads.last().unwrap(), &object, remote_ipfs, elasticsearch_url);

    let reversed_lineage: Vec<String> = lineage.into_iter().rev().collect();

    let leaf1 = ipld::insert_or_replace_link(&reversed_lineage[0], &leaf0, &object.item_count, remote_ipfs, elasticsearch_url);

    // log(elasticsearch_url, &leaf1);

    let leaf2 = ipld::insert_or_replace_link(&reversed_lineage[1], &leaf1, &"votes".to_string(), remote_ipfs, elasticsearch_url);
    
    let leaf3 = ipld::insert_or_replace_link(&reversed_lineage[2], &leaf2, &object.subscription.name, remote_ipfs, elasticsearch_url);

  //  let leaf3 = ipld::insert_or_replace_link(&reversed_lineage[2], &leaf2, &ipld::trim_dao_name(&object.subscription.contact.url), remote_ipfs, elasticsearch_url);
    
    // log(elasticsearch_url, &"4".to_string());
    let head = ipld::insert_or_replace_link(&reversed_lineage[3], &leaf3, &object.data_model.slug.clone(), remote_ipfs, elasticsearch_url);
    // log(elasticsearch_url, &"5".to_string());
    // log(elasticsearch_url, &head);


    heads.push(head);
  }

log(elasticsearch_url, &heads.last().unwrap().to_string());

heads.last().unwrap().to_string()

}


