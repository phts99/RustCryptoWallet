use anyhow::Result;
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use web3::{self, Web3, transports::Http, types::{TransactionParameters, Res, H256, U256, H160}};

pub fn create_keypair() -> Result<(SecretKey, PublicKey)> {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(6);

    Ok(secp.generate_keypair(&mut rng))
}

pub fn establish_web3_connection(url: &str) -> Result<Web3<Http>> {
    let transport: Http = Http::new(url)?;
    Ok(web3::Web3::new(transport))
}

pub async fn sign_and_end(web3: Web3<Http>, tx_object: TransactionParameters, seckey: SecretKey) -> Result<H256> {
    let signed = web3.accounts().sign_transaction(tx_object, &seckey).await?;
    Ok(web3.eth().send_raw_transaction(signed.raw_transaction).await?)
}