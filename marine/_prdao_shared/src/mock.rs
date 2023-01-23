use crate::types::{ContactInfo,DataModel, Subscription};

pub fn data () -> &'static str {

    let data:  &'static str = "{
    
        \"data_model\":{\"name\":\"DAO Votes\",\"slug\":\"votes\",\"schema\":\"\",\"domain\":\"ipld.public-record.eth\",\"query\":\"\",\"contact\":{\"url\":\"ipld.public-record.eth\"},\"widget_template\":\"QmSv5s9FXKyqMFMiAcE9zNHS25YDDowBN7dViX5n4kGwzf\"},
        
        \"subscription\":{\"id\":\"prdao-example-rinkeby\",\"client\":\"prdao-example-rinkeby\",\"contact\":{\"url\":\"prdaoexample2.aragonid.eth\"},\"data_type\":\"vote\",\"contract_type\":\"aragon_company\",\"contract\":\"0x00047774fb8ab777841c394a910ad0da99d13254\",\"contract_abi\":\"prdao-example-rinkeby\",\"topics\":[\"0x4d72fe0577a3a3f7da968d7b892779dde102519c25527b29cf7054f245c791b9\",\"0xb34ee265e3d4f5ec4e8b52d59b2a9be8fceca2f274ebc080d8fba797fea9391f\"],\"subgraph\":\"https://api.thegraph.com/subgraphs/name/aragon/aragon-voting-rinkeby\"},
        
        \"json\":{
            \"appAddress\":\"0x00047774fb8ab777841c394a910ad0da99d13254\",
           
            \"castVotes\":[
                {\"createdAt\":\"1641767000\",\"id\":\"0xa-voter:0xe986d774de323749f82ca13471c8c580316c05fb\",\"stake\":\"10000000000000000000\",\"supports\":false,\"voter\":{\"address\":\"0xe986d774de323749f82ca13471c8c580316c05fb\",\"id\":\"0x00047774fb8ab777841c394a910ad0da99d13254-voter-0xe986d774de323749f82ca13471c8c580316c05fb\"}},
                {\"createdAt\":\"1641767000\",\"id\":\"0xa-voter:0xe986d774de323749f82ca13471c8c580316c05fb\",\"stake\":\"10000000000000000000\",\"supports\":true,\"voter\":{\"address\":\"0xe986d774de323749f82ca13471c8c580316c05fb\",\"id\":\"0x00047774fb8ab777841c394a910ad0da99d13254-voter-0xe986d774de323749f82ca13471c8c580316c05fb\"}},
                {\"createdAt\":\"1641767000\",\"id\":\"0xa-voter:0xe986d774de323749f82ca13471c8c580316c05fb\",\"stake\":\"11000000000000000000\",\"supports\":true,\"voter\":{\"address\":\"0xe986d774de323749f82ca13471c8c580316c05fb\",\"id\":\"0x00047774fb8ab777841c394a910ad0da99d13254-voter-0xe986d774de323749f82ca13471c8c580316c05fb\"}},
                {\"createdAt\":\"1641767000\",\"id\":\"0xa-voter:0xe986d774de323749f82ca13471c8c580316c05fb\",\"stake\":\"21000000000000000000\",\"supports\":false,\"voter\":{\"address\":\"0xe986d774de323749f82ca13471c8c580316c05fb\",\"id\":\"0x00047774fb8ab777841c394a910ad0da99d13254-voter-0xe986d774de323749f82ca13471c8c580316c05fb\"}},
                {\"createdAt\":\"1641767000\",\"id\":\"0xa-voter:0xe986d774de323749f82ca13471c8c580316c05fb\",\"stake\":\"11000000000000000000\",\"supports\":false,\"voter\":{\"address\":\"0xe986d774de323749f82ca13471c8c580316c05fb\",\"id\":\"0x00047774fb8ab777841c394a910ad0da99d13254-voter-0xe986d774de323749f82ca13471c8c580316c05fb\"}}  
            ],
                
            \"creator\":\"0x290d57814ddf2948722d0eb138020e07a8571d11\",
            \"executed\":false,
            \"executedAt\":\"0\",
            \"id\":\"appAddress:0x00047774fb8ab777841c394a910ad0da99d13254-vote:0xa\",
            \"metadata\":\"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Lorem ipsum dolor sit amet, consectetur adipiscing elit?\",
            \"minAcceptQuorum\":\"150000000000000000\",
            \"nay\":\"42000000000000000000\",
            \"orgAddress\":\"0x0f1c5eebf086a031008269be0b291d27a11e7641\",
            \"script\":\"0x00000001\",
            \"snapshotBlock\":\"9963257\",
            \"startDate\":\"1641761599\",
            \"supportRequiredPct\":\"500000000000000000\",
            \"voteNum\":\"53\",
            \"votingPower\":\"100000000000000000000\",
            \"yea\":\"21000000000000000000\"},
        
        \"html\":\"\",
        \"embed\":\"\"
    }";

    data

}

pub fn contact_info() -> ContactInfo {

    ContactInfo {
        url: String::from("ipld.public-record.eth")
    }
}

pub fn data_model() -> DataModel {

    DataModel {
        name: String::from("Public Record Dao Votes"),
        slug: String::from("prdao_votes"),
        schema: String::from(""),
        domain: String::from("ipld.public-record.eth"),
        query: String::from(""),
        contact: contact_info()
    }
}

pub fn subscription() -> Subscription {

    Subscription {
        id: String::from("prdao-example-rinkeby"),
        client: String::from("prdao-example-rinkeby"),
        contact: contact_info(),
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
    }
}


