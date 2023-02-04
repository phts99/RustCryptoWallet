use anyhow::Result;
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use web3::{self, Web3, transports::Http};

pub fn create_keypair() -> Result<(SecretKey, PublicKey)> {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(6);

    Ok(secp.generate_keypair(&mut rng))
}

pub fn establish_web3_connection(url: &str) -> Result<Web3<Http>> {
    let transport: Http = Http::new(url)?;
    Ok(web3::Web3::new(transport))
}
