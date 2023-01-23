use crate::curl_request;
use crate::{ SubscriptionV2 };
use graphql_client::{ GraphQLQuery, Response };
use serde_json::json;

pub fn get_subgraph(id: String, sub: SubscriptionV2) -> String {


    //  let url = String::from("https://api.studio.thegraph.com/query/4789/random-electionator-2/0.0.38");
    // download to node folder 
    // with this setup i need have the file at compile time ....... 

    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "../subgraphs/eip4824/schema.graphql",    // we hebben een probleem met BigInt, BigDecFDeal en Bytes
        query_path = "../subgraphs/eip4824/query.graphql",
        response_derives = "Debug,Serialize,PartialEq"
    )]
    struct GraphView;

    let variables = graph_view::Variables {
        id: id
    };

    let request_body = GraphView::build_query(variables);
    let request_string = serde_json::to_string(&request_body).unwrap();

  //  println!("{:?}", request_string);
   
    let curl_args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("-H"),
            String::from("Content-Type: application/json"),
            String::from("--data"),
            request_string,
            sub.subgraph
        ];

    // println!("{:?}", curl_args);
    let response = curl_request(curl_args);
     
    let response = String::from_utf8(response.stdout).unwrap();
    let response_object: serde_json::Value = serde_json::from_str(&response).unwrap();
    let item_object = &response_object["data"]["proposal"];

    serde_json::to_string(&item_object).unwrap()
}