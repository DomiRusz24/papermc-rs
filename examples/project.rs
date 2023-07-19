#![allow(unused_imports)]
use papermc::PaperMcClient;
use papermc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let project = "your project";
    let response = client.project(project).await.unwrap();
    println!("{:#?}", response);
}