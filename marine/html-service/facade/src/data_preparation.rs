use serde_json::Value;

pub fn convert_into_value(data: &String) -> Value {

    serde_json::from_str(&data).unwrap()

}