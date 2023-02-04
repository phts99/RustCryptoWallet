use anyhow::Result;
use secp256k1::{PublicKey, SecretKey};
use web3::{self, Web3, transports::Http, types::{Address, TransactionParameters}};
use std::str::FromStr;
use tokio;

use crate::wallet_lib::{create_transaction_object, sign_and_send};

mod wallet_lib;

const URL: &str = "https://eth-goerli.g.alchemy.com/v2/r7OrFgAJkJ3km6iC2NOz7I8Ie72gofdt";

#[tokio::main]
async fn main() {
    let keypair = wallet_lib::create_keypair();
    
    let web3 = wallet_lib::establish_web3_connection(URL);

    let to_address = Address::from_str("0x08302CF8648A961c607e3e7Bd7B7Ec3230c2A6c5");

    let transaction_object = wallet_lib::create_transaction_object(to_address.unwrap(), 7);

    {
        let web3_clone: Web3<Http> = web3.unwrap();
        let transaction_object_clone: TransactionParameters = transaction_object.unwrap();
        let secret_key: SecretKey = keypair.unwrap().0;

        let result = wallet_lib::sign_and_send(web3_clone, transaction_object_clone, secret_key).await;

        println!("Result {:?}", result);
    }
}