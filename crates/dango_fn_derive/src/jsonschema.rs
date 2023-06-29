use serde_json::{json, Value};

pub fn string(description: String) -> Value {
    json! {
        {
            "type": "string",
            "description": description
        }
    }
}
