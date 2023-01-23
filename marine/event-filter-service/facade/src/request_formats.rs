use crate::log;
pub const JSON_RPC: &'static str = "2.0";

#[derive(Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub eth_provider: String,
    pub method: String,
    pub id: u64,
}

impl Request {
    pub fn new(eth_provider: String, method: String, id: u64) -> Self {
        Request {
            jsonrpc: String::from(JSON_RPC),
            eth_provider,
            method,
            id
        }
    }

    pub fn new_filter(&self, from_block: &String, contract: &String, topics: &Vec<String>, elasticsearch_url: &String) -> Vec<String> { // Vec<String>

        let array_string = serde_json::to_string(&topics).unwrap();
        // remove [" and "]
        // let array_content: &str = &array_string[2..array_string.len() - 2];
        let array_content: &str = "";
        // ,[,null,null]
        let params = format!("{{\"jsonrpc\":\"{}\",\"method\":\"{}\", \"params\":[{{\"fromBlock\":\"{}\",\"address\":\"{}\",\"topics\":[[\"0x4d72fe0577a3a3f7da968d7b892779dde102519c25527b29cf7054f245c791b9\",\"0xb34ee265e3d4f5ec4e8b52d59b2a9be8fceca2f274ebc080d8fba797fea9391f\"],null,null]}}], \"id\":{}}}", self.jsonrpc, self.method, from_block, contract, self.id);
        
        log(elasticsearch_url, &params);

        let args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("-H"),
            String::from("Content-Type: application/json"),
            String::from("--data"),
            params,
            self.eth_provider.to_string()
        ];

        args

    }

    pub fn filter_changes(&self, filter_id: &String) -> Vec<String> {

        let params = format!("{{\"jsonrpc\":\"{}\",\"method\":\"{}\", \"params\":[\"{}\"], \"id\":{}}}", self.jsonrpc, self.method, filter_id, self.id);
        
        let args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("-H"),
            String::from("Content-Type: application/json"),
            String::from("--data"),
            params,
            self.eth_provider.to_string()
        ];

        args 
    }
}
