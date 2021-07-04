use ic_cdk_macros::*;

use super::TransactionNotification;

#[query(name = "wants_notify")]
fn wants_notify() -> bool {
    true
}

#[update(name = "on_receive_transfer")]
fn on_receive_transfer(tn: TransactionNotification) -> Result<String, String>  {
    return Ok(tn.from.to_string())
}
