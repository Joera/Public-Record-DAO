use crate::{Subscription, ContactInfo};


pub fn get(index: u16) -> Subscription {

    let mut subscriptions: Vec<Subscription> = vec!();

    let contact1 = ContactInfo {
        url: String::from("prdaoexample2.aragonid.eth")
    };

    let sub1 = Subscription {
        id: String::from("prdao-example-rinkeby"),
        client: String::from("prdao-example-rinkeby"),
        contact: contact1,
        data_type: String::from("vote"),
        contract_type: String::from("aragon_company"),
        contract: String::from("0x00047774fb8ab777841c394a910ad0da99d13254"),
        contract_abi: String::from("prdao-example-rinkeby"),
        topics: vec!("0x4d72fe0577a3a3f7da968d7b892779dde102519c25527b29cf7054f245c791b9".to_string(),"0xb34ee265e3d4f5ec4e8b52d59b2a9be8fceca2f274ebc080d8fba797fea9391f".to_string()),
        subgraph: String::from("https://api.thegraph.com/subgraphs/name/aragon/aragon-voting-rinkeby"),
        widget_domain: String::from("widgets.public-record.eth"),
        widget_version: String::from("prdao_votes_single"),
        ipld_domain: String::from("ipld.public-record.eth"),
        ipld_tip: String::from("ipld_tip")
    };

    subscriptions.push(sub1);

    subscriptions[index as usize].clone()



}