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

    let mut line = rustyline::DefaultEditor::new().unwrap();

    loop {
        let input = match line.readline(" > ") {
            Ok(line) => line,
            _ => break,
        };

        let res = client.run_prompt(&input, &functions).unwrap();
        let fn_call = &res["function_call"];

        println!("{:?}", fn_call);

        let fn_name = fn_call["name"].as_str().unwrap();
        let fn_args = fn_call["arguments"].as_str().unwrap();
        let fn_args: Value = serde_json::from_str(fn_args).unwrap();

        if fn_name == "weather" {
            client.response("weather", "30 degrees Celsius");
        }

        let res = client.completions(&functions).unwrap();
        println!("complete {:?}", res);
    }
}
