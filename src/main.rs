use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub (crate) async fn my_handler(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    println!("Hello, world!");
    Ok(json!({ "message": "Hello, world!" }))
}
