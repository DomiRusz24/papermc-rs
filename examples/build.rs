#![allow(unused_imports)]
use papermc::PaperMcClient;
use papermc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let build = 1;
    let project = "your project";
    let version = "your version";
    let response = client.build(build, project, version).await.unwrap();
    println!("{:#?}", response);
}