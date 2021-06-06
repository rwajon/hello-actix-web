use serde_json::Value;

pub fn parse(json_string: &String) -> Value {
    let response = serde_json::from_str(&json_string).unwrap();

    response
}

pub fn stringify(json_data: &Value) -> String {
    let response = serde_json::to_string(&json_data).unwrap();

    response
}
