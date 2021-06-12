use ic_cdk::export::{Principal};
use ic_cdk::api;
use ic_cdk_macros::*;
use std::collections::HashMap;
use std::sync::RwLock;

static mut NAME: &str = "";
static mut SYMBOL: &str = "";
static mut DECIMALS: u64 = 8;
static mut OWNER: Principal = Principal::anonymous();
static mut TOTALSUPPLY: u64 = 0;

lazy_static! {
    static ref BALANCES: RwLock<HashMap<Principal, u64>> = RwLock::new(HashMap::new());
    static ref ALLOWANCES: RwLock<HashMap<Principal, HashMap<Principal, u64>>> = RwLock::new(HashMap::new());
}

#[init]
fn init(name: String, symbol: String, decimals: u64, total_supply: u64) {
    unsafe {
        NAME = Box::leak(name.into_boxed_str());
        SYMBOL = Box::leak(symbol.into_boxed_str());
        DECIMALS = decimals;
        TOTALSUPPLY = total_supply;
        OWNER = api::caller();
        let mut balances = BALANCES.write().unwrap();
        balances.insert(OWNER, TOTALSUPPLY);
    }
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
*/

#[query(name = "balanceOf")]
fn balance_of(id: Principal) -> u64 {
    let balances = BALANCES.read().unwrap();
    match balances.get(&id) {
        Some(balance) => *balance,
        None => 0,
    }
}

#[query(name = "allowance")]
fn allowance(owner: Principal, spender: Principal) -> u64 {
    let allowances = ALLOWANCES.read().unwrap();
    match allowances.get(&owner) {
        Some(inner) => {
            match inner.get(&spender) {
                Some(value) => *value,
                None => 0,
            }
        },
        None => 0,
    }
}

#[query(name = "name")]
fn name() -> String {
    unsafe {
        NAME.to_string()
    }
}

#[query(name = "symbol")]
fn symbol() -> String {
    unsafe {
        SYMBOL.to_string()
    }
}

#[query(name = "decimals")]
fn decimals() -> u64 {
    unsafe {
        DECIMALS
    }
}

#[query(name = "totalSupply")]
fn total_supply() -> u64 {
    unsafe {
        TOTALSUPPLY
    }
}

#[query(name = "owner")]
fn owner() -> Principal {
    unsafe {
        OWNER
    }
}

#[query(name = "controller")]
fn controller() -> Principal {
    // TODO: get token canister controller
    Principal::anonymous()
}
