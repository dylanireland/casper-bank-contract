#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use crate::alloc::string::ToString;

use casper_contract::{
    contract_api::{runtime, storage, system, account},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, CLType, EntryPointAccess, EntryPointType, U512, EntryPoints, EntryPoint, Parameter, contracts::NamedKeys, CLValue};

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    CouldntGetPurseKey = 0,
    CouldntTurnPurseKeyIntoURef = 1,
    CouldntReturnDepositPurse = 2
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn get_deposit_purse() {
    let purse = runtime::get_key("purse").unwrap_or_revert_with(Error::CouldntGetPurseKey).into_uref().unwrap_or_revert_with(Error::CouldntTurnPurseKeyIntoURef);
    runtime::ret(CLValue::from_t(purse.into_add()).unwrap_or_revert_with(Error::CouldntReturnDepositPurse));
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "get_deposit_purse",
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let purse = system::create_purse();

    let mut named_keys = NamedKeys::new();
    named_keys.insert(String::from("purse"), purse.into());

    let (stored_contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("multisig_example_package".to_string()),
        Some("multisig_example_access_uref".to_string()),
    );

    runtime::put_key("multisig_example_contract", stored_contract_hash.into());
}
