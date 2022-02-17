use std::fs;
use json::JsonValue;

#[allow(dead_code)]
pub fn read_json(path: &str) -> JsonValue {
    let input = fs::read(path).unwrap();
    let input = String::from_utf8(input).unwrap();
    
    json::parse(&input).unwrap()
}