#![allow(unused_imports)]
use papermc::PaperMcClient;
use papermc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let family = "your family";
    let project = "your project";
    let response = client.family(family, project).await.unwrap();
    println!("{:#?}", response);
}