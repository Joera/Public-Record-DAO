#![allow(
    non_snake_case,
    unused_variables,
    unused_imports,
    unused_parens,
    unused_mut
)]


mod temp;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use prdao_shared::adapters::log;
use prdao_shared::types::{Subscription, SubscriptionV2, ContactInfo, Job, DataModel};
use prdao_shared::eth_contract::EthContract;
use prdao_shared::ipfs;

use serde_json::Value;

mod contract;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}

#[marine]
pub fn get(index: u16, elasticsearch_url: &String) -> Subscription {

    let sub = temp::get(index);
    let mut msg: String = "found potential jobs for : ".to_owned();
    msg.push_str(&sub.id.as_str());
    log(elasticsearch_url, &msg);
    sub
}

#[marine]
pub fn getAll(eth_provider_url: String, remote_ipfs: String, elasticsearch_url: &String) -> Vec<Job> {

    // moet via Ipfs

    // let eth_provider_url = String::from("https://eth-rinkeby.alchemyapi.io/v2/oDMoyeai5fTVQxfpjKQVJM3ltl1ap9z7");
    // let remote_ipfs = String::from("http://64.227.70.116");
    // let elasticsearch_url = &String::from("http://178.128.245.19:9200/fluence/_doc/?pretty");

    let abi_cid = "bafyreie7kbhqg2pgycwsaezgy42ayohh5wu6fedtvsjbott6enmfvqzbiy";
    let contract = "0x0a7DD5cAD0D3b962A7141B754DFb5aF2B58aA0f4";

    let contract = EthContract::new(eth_provider_url, String::from(contract), &abi_cid);
    let mut i = 1;
    let mut jobs = Vec::new();
    let mut sub: SubscriptionV2;

//    println!("{:?}", contract);

    let contact1 = ContactInfo {
        url: String::from("ipld.public-record.eth")
    };

    let model1 = DataModel {
        name: String::from("Governance"),
        slug: String::from("governance"),
        domain: String::from("ipld.public-record.eth"),
        schema: String::from(""),
        query: String::from(""),
        contact: contact1   
    };



   loop {

        // log(&i.to_string(),elasticsearch_url);

        let check = contract.read(&"owners".to_string(), i, elasticsearch_url).unwrap();

        if check == "0000000000000000000000000000000000000000" { 
            break; 
        }

        let cid = contract.read(&"get".to_string(), i, elasticsearch_url).unwrap();

        if cid != "0" {

            let m = ipfs::dag_get(&cid, &remote_ipfs);

            log(elasticsearch_url, &m);
            
            sub = serde_json::from_str(&m).unwrap();

            let job = Job {
                item: String::from(""),
                item_count: String::from(""),
                data_model: model1.clone(),
                subscription: sub,
                filter: String::from(""),
                json: String::from(""),    
                cbor: String::from(""),
                html: String::from(""),
                embed: String::from("")
            };


       //     log(&serde_json::to_string(&metadata).unwrap(),elasticsearch_url);

            jobs.push(job);
        }

        i += 1;
    }

    // metadata
    jobs
}
