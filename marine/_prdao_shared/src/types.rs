#![allow(
    non_snake_case
)]

use marine_rs_sdk::marine;
use serde::{ Serialize, Deserialize };
use serde_json::Value;
pub use cid::Cid as CidGeneric;



// #[derive(Serialize, Deserialize)]
// pub type Cid = CidGeneric<multihash::U64>;

#[marine]
#[derive(Serialize, Deserialize)]
pub struct AdapterConfig {
    pub cid: String,
    pub curl_binary: bool,
    pub logger_enabled: bool,
    pub mem_page_count: u32,
    pub name: String,
}

impl AdapterConfig {
    pub fn new(name: String, cid: String, mem_page_count: u32, logger_enabled: bool, curl_binary: bool) -> AdapterConfig {         
        AdapterConfig { cid, curl_binary, logger_enabled, mem_page_count, name }
    }
}

#[marine]
#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    pub adapters: Vec<AdapterConfig>,
    pub cid: String,
    pub facade_name: String,
    pub name: String,
   
}

#[marine]
#[derive(Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub url: String
}

#[marine]
#[derive(Clone,Serialize, Deserialize)]
pub struct DataModel {
    pub name: String,
    pub slug: String,
    pub schema: String,
    pub domain: String,
    pub query: String,
    pub contact: ContactInfo
}

#[marine]
#[derive(Clone,Serialize, Deserialize)]
pub struct Subscription {

    pub id: String,
    pub client: String,
    pub contact: ContactInfo,
    pub data_type: String,
    pub contract_type: String,
    pub contract: String,
    pub contract_abi: String,
    pub topics: Vec<String>,
    pub subgraph: String,
    pub widget_domain: String,
    pub widget_version: String,
    pub ipld_domain: String,
    pub ipld_tip: String
}

#[marine]
#[derive(Clone,Serialize, Deserialize)]
pub struct Abi {

    pub version: u8,
    pub codec: String,
    pub multihash: Vec<u8>,
    pub multibaseName: String
}


#[marine]
#[derive(Clone,Serialize, Deserialize, Debug)]
pub struct SubscriptionV2 {

  //  pub abi: CidObject,
    pub name: String,
    pub topics: Vec<String>,
    pub network: String,
    pub contract: String,
    pub subgraph: String,
    pub services: String
}

pub struct PartialConfig {
    pub name: String,
    pub cid: String
}

pub struct HelperConfig {
    pub name: String,
    pub cid: String
}


#[marine]
#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    pub item: String,
    pub item_count: String,
    pub data_model: DataModel,
    pub subscription: SubscriptionV2,
    pub filter: String,
    pub json: String,    
    pub cbor: String,
    pub html: String,
    pub embed: String
}


#[derive(Serialize, Deserialize)]
pub struct TemplateDataObject {
    pub data_model: DataModel,
    pub subscription: SubscriptionV2,
    pub json: Value
}



#[derive(Clone,Serialize, Deserialize,Debug)]
pub struct CidObject  {
    #[serde(rename = "/")]
    pub Cid: String
}



#[derive(Clone,Serialize, Deserialize,Debug)]
pub struct IpfsLink {
    pub Name: String,
    pub Tsize: i32,
    pub Hash: CidObject
}

impl IpfsLink {
    pub fn update(mut link: IpfsLink, cid: &String) {
        link.Hash.Cid = cid.to_owned();
      }
}

// #[derive(Serialize, Deserialize,Debug)]
// pub struct IpfsDirectLink {
//     pub name: String,
//     pub size: i32,
//     pub cid: String
// }

#[derive(Serialize, Deserialize,Debug)]
pub struct IpfsDataObject {
    pub bytes: String
}

#[derive(Serialize, Deserialize,Debug)]
pub struct IpfsDataLink {
    #[serde(rename = "/")]
    pub data: IpfsDataObject
}

#[derive(Serialize, Deserialize,Debug)]
pub struct DagObject {
  //  #[serde(skip_serializing_if = "is_null")]
    pub Data: IpfsDataLink,
    pub Links: Vec<IpfsLink>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Voter {
    pub address: String,
    pub id: String
}

#[derive(Serialize, Deserialize,Debug)]
pub struct CastVote {
    pub createdAt: String,
    pub id: String,
    pub stake: String,
    pub supports: bool,
    pub voter: Voter 
}

#[derive(Serialize, Deserialize,Debug)]
pub struct VoteDataObject {

    pub appAddress: String,
    pub castVotes: Vec<CastVote>,
    pub creator: String,
    pub executed: bool,
    pub executedAt: String,
    pub id: String,
    pub metadata: String,
    pub minAcceptQuorum: String,
    pub nay: String,
    pub orgAddress: String,
    pub script: String,
    pub snapshotBlock: String,
    pub startDate: String,
    pub supportRequiredPct: String,
    pub voteNum: String,
    pub votingPower: String,
    pub yea: String
}

#[marine]
#[derive(Clone,Serialize, Deserialize)]
pub struct PublicMetadata {
    pub organisationId: String,
    pub contract: String,
 //   pub contact: ContactInfo,
    pub path: Vec<String>,
    pub domain: String

}


#[derive(Clone, Serialize, Deserialize)]
pub struct IpldDirectory {
    pub Links: Vec<IpfsLink> 
}

// impl IpldDirectory {
//     fn update_link(dir: &mut IpldDirectory, key: &String, value: &String) {

//         let index = &mut dir.links.into_iter().position(|s| s.name.as_str() == key).unwrap();
  

//         IpfsLink::update(mut dir.links[*index as usize], value);

//         // remove , insert , sort

//         // for mut link in dir.links {
//         //     if link.name.as_str() == key {
//         //       IpfsLink::update(&mut link, value);
//         //     }
//         // }
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct IpldDir {
    pub data: String,
    pub links: Vec<IpfsLink> 
}


#[derive(Serialize, Deserialize)]
pub struct PublicDaoObject {
    pub name: String,
    pub id: String,   
    pub contact: ContactInfo,  
    pub votes: Vec<IpfsLink>

}