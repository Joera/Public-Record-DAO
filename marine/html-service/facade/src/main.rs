
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
use std::collections::BTreeMap;


use prdao_shared::types::{ TemplateDataObject, ContactInfo, SubscriptionV2, DataModel, PartialConfig, HelperConfig, IpldObject, DagObject, CidObject, IpfsLink };
use prdao_shared::adapters::{ curl_request, log };
// use prdao_shared::mock; 
use prdao_shared::ipfs;
// use prdao_shared::file_system;


use serde::{ Deserialize, Serialize};
use serde_json::Value;
use handlebars::RenderError;
use libipld_pb::DagPbCodec;
use libipld_core::codec::Codec;
use libipld_core::codec::Decode;
use libipld_core::ipld::Ipld;
use blob::Blob;
use num::bigint::BigInt;



// const REMOTE_IPFS_PEER_HTTP: &str = "http://64.227.70.116";
// pub const TMP_DIR: &str = "./tmp-html/";


module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}

mod partials;
// mod file_system;
mod handlebars_renderer;
mod data_preparation;
mod helpers;

#[marine]
pub fn render(mut object: IpldObject, template_folder_cid: String, elasticsearch_url: &String, remote_ipfs_url: &String) -> IpldObject {

    let mut html = "".to_string(); 

    let mut msg: String = "download template folder for ".to_owned();
    msg.push_str(&template_folder_cid.as_str());
    log(elasticsearch_url, &msg);

    let root_dag = ipfs::dag_get(&template_folder_cid, remote_ipfs_url);
    // log(elasticsearch_url, &root_dag);
    // log(elasticsearch_url, &serde_json::to_string(&object).unwrap());

    let mut root_object: DagObject = serde_json::from_str(&root_dag).unwrap();
    // log(elasticsearch_url, &"yo".to_string());
    let data_dag = ipfs::dag_get(&object.cbor, remote_ipfs_url);    
    log(elasticsearch_url, &data_dag);
    // log(elasticsearch_url, &"yo2".to_string());
    let data = TemplateDataObject {
        data_model: object.data_model.clone(), 
        subscription: object.subscription.clone(),
        json: data_preparation::convert_into_value(&data_dag)
    };

    let html = handlebars_renderer::single(template_folder_cid, data, remote_ipfs_url, elasticsearch_url).replace("\n","").to_string();
    
 //   log(elasticsearch_url, &html);
    
    let ipfs_object = ipfs::add_file_returning_object(&html, &"index.html".to_string(), remote_ipfs_url, elasticsearch_url).unwrap();
  
  //  log(elasticsearch_url, &ipfs_object.to_string());

    let html_link = ipfs::format_link_with_name(&ipfs_object,"index.html".to_string());
    root_object.links.push(html_link);

    let stats: Value = serde_json::from_str(&ipfs::dag_stat(&object.cbor, remote_ipfs_url)).unwrap();

    root_object.links.push(IpfsLink {
        name: "data.json".to_string(),
        tsize: stats["Size"].to_string().parse::<i32>().unwrap(),
        hash: CidObject { cid: object.json.clone() }
    });

    root_object = ipfs::filter_links(root_object,"widget.handlebars".to_string());

    let new_root_cid = ipfs::dag_put(root_object, "dag-pb", remote_ipfs_url);

    // log(elasticsearch_url, &"this should be a folder with dag-pb:".to_string());
    log(elasticsearch_url, &ipfs::extract_cid(&new_root_cid));

    object.html = ipfs::extract_cid(&new_root_cid);

    object

}
