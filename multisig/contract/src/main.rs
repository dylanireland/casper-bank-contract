#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;
use alloc::vec;
use crate::alloc::string::ToString;

use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, CLType, EntryPointAccess, EntryPointType, EntryPoints, EntryPoint, Parameter, contracts::NamedKeys, URef, Key};

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    CouldntConvertPurseArgumentIntoAccount = 0,
    CouldntGetContractPurse = 1,
    CouldntTurnContractPurseKeyIntoURef = 2,
    CouldntGetPurseBalance = 3,
    CouldntTransferFromPurseToContractPurse = 4,
    CouldntCreateBalancesDictionary = 5,
    CouldntGetBalancesDictionary = 6,
    CouldntUnwrapBalancesDictionaryKeyIntoURef = 7
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn add_to_ledger() {
    let purse = runtime::get_named_arg::<URef>("purse");
    let caller = runtime::get_named_arg::<Key>("caller").into_account().unwrap_or_revert_with(Error::CouldntConvertPurseArgumentIntoAccount);

    let contract_purse = runtime::get_key("purse").unwrap_or_revert_with(Error::CouldntGetContractPurse).into_uref().unwrap_or_revert_with(Error::CouldntTurnContractPurseKeyIntoURef);
    let balance = system::get_purse_balance(purse).unwrap_or_revert_with(Error::CouldntGetPurseBalance);
    system::transfer_from_purse_to_purse(purse, contract_purse, balance, None).unwrap_or_revert_with(Error::CouldntTransferFromPurseToContractPurse);
    let balances = runtime::get_key("balances").unwrap_or_revert_with(Error::CouldntGetBalancesDictionary).into_uref().unwrap_or_revert_with(Error::CouldntUnwrapBalancesDictionaryKeyIntoURef);
    storage::dictionary_put(balances, &caller.to_formatted_string(), balance);
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "add_to_ledger",
        vec![
            Parameter::new("purse", CLType::URef),
            Parameter::new("caller", CLType::Key)
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let purse = system::create_purse();
    let balances = storage::new_dictionary("balances").unwrap_or_revert_with(Error::CouldntCreateBalancesDictionary);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(String::from("purse"), purse.into());
    named_keys.insert(String::from("balances"), balances.into());

    let (stored_contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("multisig_example_package".to_string()),
        Some("multisig_example_access_uref".to_string()),
    );

    runtime::put_key("multisig_example_contract", stored_contract_hash.into());
}
