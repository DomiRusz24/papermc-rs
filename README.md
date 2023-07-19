<div id="top"></div>

<p align="center">
    <a href="https://github.com/domirusz24/papermc-rs/stargazers">
        <img src="https://img.shields.io/github/stars/domirusz24/papermc-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/domirusz24/papermc-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/domirusz24/papermc-rs/ci?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/papermc">
    <img src="https://img.shields.io/crates/d/papermc?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/papermc">
    <img src="https://img.shields.io/crates/v/papermc?style=flat-square" alt="Crates.io" />
</a>

</p>

PaperMC client, generated from the OpenAPI spec.

# Usage

```rust
use papermc::PaperMcClient;
use papermc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let response = client.projects().await.unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
papermc = "0.1.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/papermc)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*