extern crate reqwest;

use std::error::Error;

static RPC_ENDPOINT: &str = "https://pwrrpc.pwrlabs.io";

pub async fn nonceOfUser(address:&str) -> Result<String, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/nonceOfUser/?userAddress=");
   endpoint.push_str(address);
   reqwest::get(endpoint).await?
        .text()
        .await
}

pub async fn balanceOf(address:&str) -> Result<String, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/balanceOf/?userAddress=");
   endpoint.push_str(address);
   reqwest::get(endpoint).await?
        .text()
        .await
}

pub async fn getBlock(blockNumber:i32) -> Result<String, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/block/?blockNumber=");
   endpoint.push_str(&blockNumber.to_string());
   reqwest::get(endpoint).await?
        .text()
        .await
}

pub async fn feePerByte() -> Result<String, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/feePerByte/");
   reqwest::get(endpoint).await?
        .text()
        .await
}

pub async fn blocksCount() -> Result<String, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/blocksCount/");
   reqwest::get(endpoint).await?
        .text()
        .await
}

pub async fn validatorsCount() -> Result<String, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/validatorsCount/");
   reqwest::get(endpoint).await?
        .text()
        .await
}