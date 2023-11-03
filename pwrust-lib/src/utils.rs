extern crate serde;
extern crate serde_json;
extern crate reqwest;

use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

static RPC_ENDPOINT: &str = "https://pwrrpc.pwrlabs.io";

#[derive(Debug, Deserialize)]
struct Response {
    data: CommonData,
    status: String,
}

#[derive(Debug, Deserialize)]
struct CommonData {
    nonce: Option<i32>,
    blocksCount: Option<i32>,
    balance: Option<i64>,
    feePerByte: Option<i32>,
    validatorsCount: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct BlockData {
    blockHash: String,
    success: bool,
    blockNumber: i32,
    blockReward: i32,
    transactionCount: i32,
    transactions: i32,
    blockSubmitter: String,
    blockSize: i32,
    timestamp: i32,
}

pub async fn nonceOfUser(address:&str) -> Result<i32, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/nonceOfUser/?userAddress=");
   endpoint.push_str(address);
   let response = reqwest::get(endpoint).await?
        .text()
        .await;

	let responseUnwrapped = response.unwrap();
	let response: Response = serde_json::from_str(&responseUnwrapped).unwrap();
	let nonce = response.data.nonce;

    Ok(nonce.unwrap())
}

pub async fn balanceOf(address:&str) -> Result<i64, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/balanceOf/?userAddress=");
   endpoint.push_str(address);
   let response = reqwest::get(endpoint).await?
        .text()
        .await;

   let responseUnwrapped = response.unwrap();
   let response: Response = serde_json::from_str(&responseUnwrapped).unwrap();
   let balance = response.data.balance;

   Ok(balance.unwrap())
}

pub async fn getBlock(blockNumber:i32) -> Result<String, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/block/?blockNumber=");
   endpoint.push_str(&blockNumber.to_string());
   reqwest::get(endpoint).await?
        .text()
        .await
}

pub async fn feePerByte() -> Result<i32, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/feePerByte/");
   let response = reqwest::get(endpoint).await?
        .text()
        .await;

   let responseUnwrapped = response.unwrap();
   let response: Response = serde_json::from_str(&responseUnwrapped).unwrap();
   let feePb = response.data.feePerByte;

   Ok(feePb.unwrap())
}

pub async fn blocksCount() -> Result<i32, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/blocksCount/");
   let response = reqwest::get(endpoint).await?
        .text()
        .await;

   let responseUnwrapped = response.unwrap();
   let response: Response = serde_json::from_str(&responseUnwrapped).unwrap();
   let blocksCnt = response.data.blocksCount;

   Ok(blocksCnt.unwrap())
}

pub async fn validatorsCount() -> Result<i32, reqwest::Error> {
   let mut endpoint = RPC_ENDPOINT.to_owned();
   endpoint.push_str("/validatorsCount/");
   let response = reqwest::get(endpoint).await?
        .text()
        .await;

   let responseUnwrapped = response.unwrap();
   let response: Response = serde_json::from_str(&responseUnwrapped).unwrap();
   let validatorsCnt = response.data.validatorsCount;

   Ok(validatorsCnt.unwrap())
}