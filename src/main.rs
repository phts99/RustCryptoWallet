mod wallet_lib;
fn main() {
    let new_key: std::result::Result<(secp256k1::SecretKey, secp256k1::PublicKey), anyhow::Error> = generate_keypair();
    println!("New key {:?}", new_key);
}

pub fn generate_keypair() -> std::result::Result<(secp256k1::SecretKey, secp256k1::PublicKey), anyhow::Error> {
    return wallet_lib::create_keypair();
}