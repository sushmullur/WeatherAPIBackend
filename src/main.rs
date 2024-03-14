use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{Value, json};
use std::collections::HashMap;


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
    
    Ok(json!({ "city": target_city }))
}
