extern crate pwrust_lib;
extern crate serde;
extern crate serde_json;

use pwrust_lib::utils::nonceOfUser;
use pwrust_lib::utils::balanceOf;
use pwrust_lib::utils::feePerByte;
use pwrust_lib::utils::blocksCount;
use pwrust_lib::utils::validatorsCount;
use pwrust_lib::utils::getBlock;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::error::Error;

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
    status: String,
}

#[derive(Debug, Deserialize)]
struct Data {
    blocksCount: i32,
}

#[tokio::main]
async fn main() {
    let address = "0x4097a04a9a8fef9ffb4a64e460193e6eb0b557c8";
    println!("Test for {}", address);

    let nonce = nonceOfUser(address).await;
    println!("Nonce: {:?}", nonce);
    
    let balance = balanceOf(address).await;
    println!("Balance: {:?}", balance);
    
    let feePb = feePerByte().await;
    println!("Fee per byte: {:?}", feePb);
    
    let blocksCnt = blocksCount().await;
    println!("Blocks count: {:?}", blocksCnt);
     
    let validatorsCnt = validatorsCount().await;
    println!("Validators count: {:?}", validatorsCnt);
    
	let blocksCntUnwrapped = blocksCnt.unwrap();
	println!("Blocks Count unwrapped: {}", blocksCntUnwrapped);
	
	let response: Response = serde_json::from_str(&blocksCntUnwrapped).unwrap();
	let blocksCountNo = response.data.blocksCount;
	println!("blocksCountno: {}", blocksCountNo);
	
	let latestBlock = getBlock(blocksCountNo - 1).await;
	println!("Latest block: {:?}", latestBlock);
}