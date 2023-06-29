use std::{env, time::Duration};

use dango_client::ClientBuilder;
use dotenv::dotenv;
use serde_json::{json, Value};

fn main() {
    dotenv().ok();
    env_logger::init();

    let api_key = env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");

    let requester = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .unwrap();

    let mut client = ClientBuilder::default()
        .requester(requester)
        .api_key(api_key)
        .model("gpt-3.5-turbo-0613".into())
        .build()
        .unwrap();

    let functions: Vec<Value> = vec![json!({
        "name": "weather",
        "description": "get the weather of a given city",
        "parameters": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "the city to get the weather of"
                }
            }
        }
    })];

    client.run_prompt("", functions).unwrap();
}
