use std::{collections::HashMap, env, error::Error};

use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_APP_LOG", "DEBUG");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    let log = warp::log("basic");
    let math = warp::get()
        .and(warp::path("math"))
        .and(warp::path::param::<String>())
        .and(warp::query::<HashMap<String, i32>>())
        .and(warp::path::end())
        .and_then(get_items);
    let apis = math.with(log);
    warp::serve(apis).run(([127, 0, 0, 1], 8080)).await;
    Ok(())
}

async fn get_items(
    param: String,
    number: HashMap<String, i32>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if param == String::from("add") {
        let num1 = number.get("num1").unwrap();
        let num2 = number.get("num2").unwrap();
        return Ok(format!("get {} + {} = {}", num1, num2, num1 + num2));
    } else if param == String::from("sub") {
        let num1 = number.get("num1").unwrap();
        let num2 = number.get("num2").unwrap();
        return Ok(format!("get {} - {} = {}", num1, num2, num1 - num2));
    }
    Ok(format!("review your input"))
}
