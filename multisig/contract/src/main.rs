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
use casper_types::{ApiError, CLType, EntryPointAccess, EntryPointType, EntryPoints, EntryPoint, Parameter, contracts::NamedKeys, URef, U512};

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    //CouldntConvertPurseArgumentIntoAccount = 0,
    CouldntGetContractPurse = 1,
    CouldntTurnContractPurseKeyIntoURef = 2,
    CouldntGetPurseBalance = 3,
    CouldntTransferFromPurseToContractPurse = 4,
    CouldntCreateBalancesDictionary = 5,
    CouldntGetBalancesDictionary = 6,
    CouldntUnwrapBalancesDictionaryKeyIntoURef = 7,
    CouldntUnwrapBalancesDictionaryKey = 8,
    InsufficientFunds = 9,
    WithdrawalFailed = 10
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn deposit() {
    let purse = runtime::get_named_arg::<URef>("purse");
    //let caller = runtime::get_named_arg::<Key>("caller").into_account().unwrap_or_revert_with(Error::CouldntConvertPurseArgumentIntoAccount);
    let caller = runtime::get_caller();

    let contract_purse = runtime::get_key("purse").unwrap_or_revert_with(Error::CouldntGetContractPurse).into_uref().unwrap_or_revert_with(Error::CouldntTurnContractPurseKeyIntoURef);
    let balance: U512 = system::get_purse_balance(purse).unwrap_or_revert_with(Error::CouldntGetPurseBalance);
    system::transfer_from_purse_to_purse(purse, contract_purse, balance, None).unwrap_or_revert_with(Error::CouldntTransferFromPurseToContractPurse);
    let balances = runtime::get_key("balances").unwrap_or_revert_with(Error::CouldntGetBalancesDictionary).into_uref().unwrap_or_revert_with(Error::CouldntUnwrapBalancesDictionaryKeyIntoURef);
    
    match storage::dictionary_get::<U512>(balances, &caller.to_string()) {
        Ok(Some(old_balance)) => {
            storage::dictionary_put(balances, &caller.to_string(), old_balance + balance);
        },
        Ok(None) => {
            storage::dictionary_put(balances, &caller.to_string(), balance);
        },
        Err(error) => {
            runtime::revert(ApiError::from(error));
        }
    }
}

#[no_mangle]
pub extern "C" fn withdraw() {
    let amount = runtime::get_named_arg::<U512>("amount");
    let caller = runtime::get_caller();
    let contract_purse = runtime::get_key("purse").unwrap_or_revert_with(Error::CouldntGetContractPurse).into_uref().unwrap_or_revert_with(Error::CouldntTurnContractPurseKeyIntoURef);

    let balances = runtime::get_key("balances").unwrap_or_revert_with(Error::CouldntGetBalancesDictionary).into_uref().unwrap_or_revert_with(Error::CouldntUnwrapBalancesDictionaryKeyIntoURef);

    match storage::dictionary_get::<U512>(balances, &caller.to_string()) {
        Ok(Some(balance)) => {
            if balance < amount {
                runtime::revert(Error::InsufficientFunds);
            } else {
                storage::dictionary_put(balances, &caller.to_string(), balance - amount);
                system::transfer_from_purse_to_account(contract_purse, caller, amount, None).unwrap_or_revert_with(Error::WithdrawalFailed);
            }
        },
        Ok(None) => {
            runtime::revert(Error::InsufficientFunds);
        },
        Err(error) => {
            runtime::revert(ApiError::from(error));
        }
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "deposit",
        vec![
            Parameter::new("purse", CLType::URef),
            //Parameter::new("caller", CLType::Key)
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            Parameter::new("amount", CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let mut named_keys = NamedKeys::new();

    let purse = system::create_purse();
    named_keys.insert(String::from("purse"), purse.into());

    if runtime::has_key("balances") {
        named_keys.insert(String::from("balances"), runtime::get_key("balances").unwrap_or_revert_with(Error::CouldntUnwrapBalancesDictionaryKey));
    } else {
        let balances = storage::new_dictionary("balances").unwrap_or_revert_with(Error::CouldntCreateBalancesDictionary);
        named_keys.insert(String::from("balances"), balances.into());
    }

    let (stored_contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("multisig_example_package".to_string()),
        Some("multisig_example_access_uref".to_string()),
    );

    runtime::put_key("multisig_example_contract", stored_contract_hash.into());
}
