use candid::CandidType;
use ic_types::Principal;
use serde::{Deserialize, Serialize};

use std::hash::Hash;

pub mod receiver;
pub mod token;

/// Struct sent by the ledger canister when it notifies a recipient of a payment
#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug)]
pub struct TransactionNotification {
    pub from: Principal,
    pub to: Principal,
    pub amount: u64,
}
