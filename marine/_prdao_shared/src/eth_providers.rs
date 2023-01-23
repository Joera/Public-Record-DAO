

pub fn match_eth_provider(network: &str) -> String {

    let mut provider = "";

    match network.trim() {

        "mainnet" => { provider = "https://eth-mainnet.alchemyapi.io/v2/JvngIk96PjsTercHvN6TeJu5Za3W77J8" },
        "polygon" => { provider = "https://polygon-mainnet.g.alchemy.com/v2/QW8dFn1iLesQWMYoD2pEwaNz8HHC-y7X" },
        "rinkeby" => { provider = "https://eth-rinkeby.alchemyapi.io/v2/oDMoyeai5fTVQxfpjKQVJM3ltl1ap9z7" },
        "goerli" => { provider = "https://eth-goerli.alchemyapi.io/v2/Btuj6CK5zkBBtMDVJXM7zTa4RrC6hvCP" },
        _ => { provider = "xxx" }
    };

    String::from(provider)

}