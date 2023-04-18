#![no_std]
#![no_main]

use casper_contract::{
    contract_api::{runtime, system, account},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{U512, RuntimeArgs, URef, ContractHash, Key};

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg("amount");
    let contract_hash: ContractHash = runtime::get_named_arg::<Key>("contract_hash")
    .into_hash()
    .map(|hash| ContractHash::new(hash))
    .unwrap();
    let deposit_purse: URef = runtime::call_contract(contract_hash, "get_deposit_purse", RuntimeArgs::new());

    system::transfer_from_purse_to_purse(account::get_main_purse(), deposit_purse, amount, None).unwrap_or_revert();
}