use solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::util::basic_util;

pub fn get_balance() -> u64 {
    let pub_key = &basic_util::get_pubkey();
    let client = basic_util::get_client();

    match client.get_balance(pub_key) {
        Ok(balance) => {
            balance / LAMPORTS_PER_SOL
        },
        Err(_) => {
            println!("Error getting balance");
            0
        },
    }
}