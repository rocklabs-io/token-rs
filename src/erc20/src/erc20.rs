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

// TODO: overflow/underflow check
#[update(name = "transfer")]
fn transfer(to: Principal, value: u64) -> bool {
    let from = api::caller();
    if from == to {
        return false;
    }
    let balances_read = BALANCES.read().unwrap();
    match balances_read.get(&from) {
        Some(from_balance) => {
            if *from_balance < value {
                false
            } else {
                let to_balance = balances_read.get(&to).or_else(|| Some(&0)).unwrap();
                let mut balances = BALANCES.write().unwrap();
                balances.insert(from, from_balance - value);
                balances.insert(to, *to_balance + value);
                true
            }
        },
        None => false,
    }
}

#[update(name = "transferFrom")]
fn transfer_from(from: Principal, to: Principal, value: u64) -> bool {
    let allowances_read = ALLOWANCES.read().unwrap();
    match allowances_read.get(&from) {
        Some(inner) => {
            match inner.get(&to) {
                Some(allowance) => {
                    if *allowance < value {
                        false
                    } else {
                        let balances_read = BALANCES.read().unwrap();
                        let from_balance = balances_read.get(&from).or_else(|| Some(&0)).unwrap();
                        let to_balance = balances_read.get(&to).or_else(|| Some(&0)).unwrap();
                        if *from_balance < value {
                            false
                        } else {
                            let mut balances = BALANCES.write().unwrap();
                            balances.insert(from, *from_balance - value);
                            balances.insert(to, *to_balance + value);
                            true
                        }
                    }
                },
                None => false,
            }
        },
        None => false,
    }
}


// TODO: fix compile error
#[update(name = "approve")]
fn approve(spender: Principal, value: u64) -> bool {
    let owner = api::caller();
    let allowances_read = ALLOWANCES.read().unwrap();
    match allowances_read.get(&owner) {
        Some(inner) => {
            let mut temp = inner.clone();
            temp.insert(spender, value);
            let mut allowances = ALLOWANCES.write().unwrap();
            allowances.insert(owner, temp);
        },
        None => {
            let mut inner = HashMap::new();
            inner.insert(spender, value);
            let mut allowances = ALLOWANCES.write().unwrap();
            allowances.insert(owner, inner);
        }
    }
    true
}

#[update(name = "mint")]
fn mint(to: Principal, value: u64) -> bool {
    if api::caller() != to {
        false
    } else {
        let balances_read = BALANCES.read().unwrap();
        let balance_before = balances_read.get(&to).or_else(|| Some(&0)).unwrap();
        if balance_before + value >= u64::MAX {
            false
        } else {
            let mut balances = BALANCES.write().unwrap();
            balances.insert(to, balance_before + value);
            unsafe {
                TOTALSUPPLY += value;
            }
            true
        }
    }
}

#[update(name = "burn")]
fn burn(from: Principal, value: u64) -> bool {
    if api::caller() != from || api::caller() != owner() {
        false
    } else {
        let balances_read = BALANCES.read().unwrap();
        match balances_read.get(&from) {
            Some(balance) => {
                if *balance < value {
                    false
                } else {
                    let mut balances = BALANCES.write().unwrap();
                    balances.insert(from, balance - value);
                    unsafe {
                        TOTALSUPPLY -= value;
                    }
                    true
                }
            },
            None => false,
        }
    }
}

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
