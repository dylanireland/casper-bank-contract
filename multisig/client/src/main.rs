#![no_std]
#![no_main]

use casper_contract::{
    contract_api::{runtime, system, account},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{U512, RuntimeArgs, ContractHash, Key};

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg("amount");
    let contract_hash: ContractHash = runtime::get_named_arg::<Key>("contract_hash")
    .into_hash()
    .map(|hash| ContractHash::new(hash))
    .unwrap();

    let new_purse = system::create_purse();

    system::transfer_from_purse_to_purse(account::get_main_purse(), new_purse, amount, None).unwrap_or_revert();

    let mut runtime_args = RuntimeArgs::new();
    runtime_args.insert("purse", new_purse).unwrap_or_revert();
    runtime_args.insert("caller", runtime::get_caller()).unwrap_or_revert();

    runtime::call_contract::<()>(contract_hash, "add_to_ledger", runtime_args);

    
}