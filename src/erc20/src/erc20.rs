use ic_cdk::export::{Principal};
use ic_cdk_macros::*;
use std::collections::HashMap;

static mut NAME: &str = "";
static mut SYMBOL: &str = "";
static mut DECIMALS: u64 = 8;
static mut OWNER: Principal = Principal::anonymous();
static mut TOTALSUPPLY: u64 = 0;

lazy_static! {
    static ref BALANCES: HashMap<Principal, u64> = HashMap::new();
    static ref ALLOWANCES: HashMap<Principal, HashMap<Principal, u64>> = HashMap::new();
}

#[init]
fn init() {

}

/*
#[update(name = "transfer")]
fn transfer() -> bool {

}

#[update(name = "transferFrom")]
fn transferFrom() -> bool {

}

#[update(name = "approve")]
fn approve() -> bool {

}

#[update(name = "mint")]
fn mint() -> bool {

}

#[update(name = "burn")]
fn burn() -> bool {

}

#[query(name = "balanceOf")]
fn balance_of(id: Principal) -> u64 {

}

#[query(name = "allowance")]
fn allowance(owner: Principal, spender: Principal) -> u64 {

}

#[query(name = "name")]
fn name() -> String {

}

#[query(name = "symbol")]
fn symbol() -> String {

}

#[query(name = "decimals")]
fn decimals() -> u64 {

}

#[query(name = "totalSupply")]
fn total_supply() -> u64 {

}

#[query(name = "owner")]
fn owner() -> Principal {

}

#[query(name = "controller")]
fn controller() -> Principal {

}
*/
