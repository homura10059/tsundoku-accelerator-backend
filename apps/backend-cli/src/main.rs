use domains::wish_lists;
use dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let target = args.get(1).unwrap();
    println!("target: {:?}", target);

    match target.as_str() {
        "update_all_wish_list" => wish_lists::update_all_wish_list().await.unwrap(),
        _ => println!("not matched"),
    }
}
