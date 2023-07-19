#![allow(unused_imports)]
use papermc::PaperMcClient;
use papermc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let response = client.projects().await.unwrap();
    println!("{:#?}", response);
}