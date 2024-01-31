use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;

pub fn get_pubkey() -> Pubkey {
    Pubkey::from_str(
        env::var("MY_PUB_KEY")
            .expect("Error finding the public key")
            .as_str(),
    )
    .expect("Error getting the public key")
}

#[allow(dead_code)]
pub fn get_client() -> RpcClient {
    let rpc_url = String::from("https://api.devnet.solana.com"); // JSON RPC URL for devnet
    RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed())
}
