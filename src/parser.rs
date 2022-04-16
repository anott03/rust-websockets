use serde_json::{Result, Value};
use serde::Deserialize;

#[derive(Debug)]
pub struct Message {
    operation: String,
    data: String,
}

pub fn parse_message(message: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(message)?;
    Ok(v)
}
