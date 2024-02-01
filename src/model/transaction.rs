use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TransactionSolPayload {
    pub sol_to_send: f64,
    pub to_pubkey: String,
}