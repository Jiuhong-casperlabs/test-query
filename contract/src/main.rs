#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec,
};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key};

#[no_mangle]
pub fn test() {}

#[no_mangle]
pub extern "C" fn call() {
    // Create initial named keys of the contract.
    let mut named_keys: BTreeMap<String, Key> = BTreeMap::new();

    let my_dic_uref = storage::new_dictionary("my_dicname").unwrap();
    named_keys.insert("my_dicname".to_string(), my_dic_uref.into());
    named_keys.insert(
        "hello".to_string(),
        storage::new_uref("worldlalalal").into(),
    );

    let entry_points = {
        let mut entry_points = EntryPoints::new();

        let test1 = EntryPoint::new(
            "test",
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(test1);
        entry_points
    };

    let (contracthash, _version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        None,
        Some("my-uref".to_string()),
    );
    runtime::put_key("mycontracthash", contracthash.into());
}
