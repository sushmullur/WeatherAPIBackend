use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;


// Comments are notes to self

// Using the tokia::main attribute allows us to use async/await syntax on main method
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

// This is the handler function that will be called by the lambda runtime
// pub (crate) restricts public access to within the crate
pub (crate) async fn my_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;

    let data: HashMap<String, Value> = serde_json::from_value(payload)
        .map_err(|e| Error::from(e.to_string()))?; 
    let target_city = data.get("body").unwrap().as_str().unwrap();
    let api_response = request_api(target_city.to_string()).await.unwrap();

    Ok(json!(api_response))
}

async fn request_api(city_name: String) -> Result<String, Box<dyn std::error::Error>> {
    let api_url = env::var("API_URL")?;
    let api_key = env::var("API_KEY")?;
    let id = env::var("ID")?;
    let url = format!("{}q={}&id={}&appid={}", api_url, city_name, id, api_key);
    let body = reqwest::get(&url).await?.text().await?;
    let response = parse_response(body);
    Ok(response)
}

fn parse_response(response: String) -> String {
    let map: HashMap<String, Value> = serde_json::from_str(&response).unwrap();
    let list = map.get("list").unwrap().as_array().unwrap();
    let first = list[0].as_object().unwrap();
    let json_string = serde_json::to_string_pretty(first).unwrap();
    json_string
}