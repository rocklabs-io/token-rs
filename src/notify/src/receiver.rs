use ic_cdk::export::{Principal};
use ic_cdk::storage;
use ic_cdk::api;
use ic_cdk_macros::*;

#[query(name = "wants_notify")]
fn wants_notify() -> bool {
    true
}

#[query(name = "on_receive_transfer")]
fn on_receive_transfer(token: TokenMetadata, from: Principal, amount: u64) -> bool {
    // do something
    true
}
