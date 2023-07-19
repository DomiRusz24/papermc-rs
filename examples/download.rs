#![allow(unused_imports)]
use papermc::PaperMcClient;
use papermc::model::*;
use papermc::request::DownloadRequired;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let args = DownloadRequired {
        build: 1,
        download: "your download",
        project: "your project",
        version: "your version",
    };
    let response = client.download(args).await.unwrap();
    println!("{:#?}", response);
}