use serde_json::{Result, Value};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Message {
    operation: String,
    data: String,
}

pub fn parse_message(id: &str, message: &str) -> Result<Message> {
    let v: Value = serde_json::from_str(message)?;
    println!("{}: {:?}", id, v);
    Ok(Message {
        operation: String::from("op"),
        data: String::from("data"),
    })
}
