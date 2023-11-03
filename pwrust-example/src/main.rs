extern crate pwrust_lib;

use pwrust_lib::utils::nonceOfUser;
use pwrust_lib::utils::balanceOf;
use pwrust_lib::utils::feePerByte;
use pwrust_lib::utils::blocksCount;
use pwrust_lib::utils::validatorsCount;
use pwrust_lib::utils::getBlock;

use std::error::Error;

#[tokio::main]
async fn main() {
    let address = "0x4097a04a9a8fef9ffb4a64e460193e6eb0b557c8";
    println!("Test for {}", address);

    let nonce = nonceOfUser(address).await;
    println!("Nonce: {:?}", nonce.unwrap());
    
    let balance = balanceOf(address).await;
    println!("Balance: {:?}", balance.unwrap());
    
    let feePb = feePerByte().await;
    println!("Fee per byte: {:?}", feePb.unwrap());
    
    let blocksCnt = blocksCount().await;
	let blocksCntUnwrapped = blocksCnt.unwrap();
    println!("Blocks count: {:?}", blocksCntUnwrapped);
    
    let validatorsCnt = validatorsCount().await;
    println!("Validators count: {:?}", validatorsCnt.unwrap());
    
	let latestBlock = getBlock(blocksCntUnwrapped - 1).await;
	println!("Latest block: {:?}", latestBlock);
}