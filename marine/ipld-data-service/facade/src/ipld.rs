// use crate::curl_request;
// use chrono::Utc;
// use marine_rs_sdk::marine;
use serde::{ Deserialize, Serialize};
use serde_json::Value;
// use std::io::Cursor;

// use std::collections::BTreeMap;
use indexmap::IndexMap;
use crate::{ IpldObject, IpldDir, IpldDirectory, IpfsLink, PublicMetadata, CidObject,  DagObject, curl_request, log, ipfs};

pub fn create_widget_leaf(object: &IpldObject, remote_ipfs: &String, elasticsearch_url: &String) -> String {

    let public_metadata = PublicMetadata {

        organisationId: object.subscription.name.clone(),
        contract: object.subscription.contract.clone(),
     //   contact: object.subscription.contact.clone(),
        path: vec!("votes".to_string(),"governance".to_string()), // object.data_model.name
        domain: object.data_model.domain.clone()
    };

    let metadata = ipfs::dag_put_serde_value(serde_json::to_value(public_metadata).unwrap(),"dag-cbor", &remote_ipfs).unwrap();

    let metaDataLink = linkifyFile("metadata".to_string(), ipfs::extract_cid(&metadata),remote_ipfs,elasticsearch_url);
    let jsonLink = linkifyFile("json".to_string(), object.json.clone(),remote_ipfs,elasticsearch_url);
    let cborLink = linkifyFile("cbor".to_string(), object.cbor.clone(),remote_ipfs,elasticsearch_url);
    let widgetLink = linkifyDir("widget".to_string(), object.html.clone());

    let mut single_vote_object = IpldDirectory {
        Links: vec!(metaDataLink, jsonLink, cborLink, widgetLink)
    };

    let leaf = ipfs::dag_put_serde_value(serde_json::to_value(single_vote_object).unwrap(),"dag-pb", &remote_ipfs).unwrap();
    
    ipfs::extract_cid(&leaf)
}

// met object gaan we direct lijn bouwen tussen head en leaf 
pub fn build_lineage(tip: &String, object: &IpldObject, remote_ipfs: &String, elasticsearch_url: &String) -> Vec<String> { 
   
    let mut nodes: Vec<String> = vec!();
    nodes.push(tip.to_string());

    let array = vec!(object.data_model.slug.clone(), object.subscription.name.clone(),"votes".to_string());

    for selector in array {

      let links: Vec<IpfsLink> = parse_links(nodes.last().unwrap(), remote_ipfs);

      for l in links {
          if l.Name == selector {
              nodes.push(l.Hash.Cid);
          } else {
              break
          }
      }
    }

    nodes
    
}

pub fn insert_or_replace_link(dir_cid: &String, link_cid: &String, key: &String, remote_ipfs: &String, elasticsearch_url: &String) -> String {
    
    let mut dir: IpldDirectory = serde_json::from_str(&ipfs::dag_get(dir_cid,remote_ipfs)).unwrap();

    if link_exists(dir_cid, key, remote_ipfs) {
      // remove 
      let index = dir.Links.iter().position(|x| &x.Name == key).unwrap();
      dir.Links.remove(index);
    }
    // insert 
    dir.Links.push(
      linkifyDir(key.clone(),link_cid.clone())
    );
    // sort
    dir.Links.sort_by_key(|d| d.Name.clone());

    let res = ipfs::dag_put_serde_value(serde_json::to_value(dir).unwrap(),"dag-pb", &remote_ipfs).unwrap();

    ipfs::extract_cid(&res)
  }

pub fn link_exists(cid: &String, id: &String, remote_ipfs: &String) -> bool {

    let mut b: bool = false;

    let links = parse_links(cid, remote_ipfs);
    for l in links {
        if l.Name.as_str() == id {
            b = true;
        }
    }

    b
}


fn linkifyFile(Name: String, cid: String, remote_ipfs_url: &String, elasticsearch_url: &String) -> IpfsLink {

    let stats: Value = serde_json::from_str(&ipfs::dag_stat(&cid, remote_ipfs_url)).unwrap();
  
    IpfsLink {
      Name,
      Tsize: stats["Size"].to_string().parse::<i32>().unwrap(),
      Hash: CidObject { Cid: cid }
    }
}
  
pub fn linkifyDir(Name: String, cid: String) -> IpfsLink {
  
    IpfsLink {
      Name,
      Tsize: 1000,
      Hash: CidObject { Cid: cid }
    }
}

fn parse_links (cid: &String, remote_ipfs: &String) -> Vec<IpfsLink> {

    let dag = ipfs::dag_get(cid, remote_ipfs);
    let dir: Value = serde_json::from_str(&dag).unwrap();
    serde_json::from_value(dir["Links"].clone()).unwrap()

}

// pub fn trim_dao_name(url: &String) -> String {

//     let v: Vec<&str> = url.split('.').collect();
//     v[0].to_string()
// }