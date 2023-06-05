#![no_std]
#![no_main]

use casper_contract::{
    contract_api::runtime,
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


    let mut runtime_args = RuntimeArgs::new();
    runtime_args.insert("amount", amount).unwrap_or_revert();

    runtime::call_contract::<()>(contract_hash, "withdraw", runtime_args);
}