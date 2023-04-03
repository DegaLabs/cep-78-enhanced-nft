#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;
mod converters;
mod error;

use crate::converters::u512_to_u256;
use crate::error::Error;
use contract::contract_api::{account, runtime, system};
use contract::unwrap_or_revert::UnwrapOrRevert;
use types::account::AccountHash;
use types::{
    runtime_args, ContractHash, ContractPackageHash, HashAddr, Key, RuntimeArgs, URef, U256, U512,
};

#[no_mangle]
pub extern "C" fn call() {
    let deposit_amount: U512 = runtime::get_named_arg("amount");
    // let contract_hash: ContractHash = ContractHash::from(
    //     runtime::get_named_arg::<Key>("contract_hash_key")
    //         .into_hash()
    //         .unwrap_or_revert(),
    // );

    let deposit_entry_point_name: String = runtime::get_named_arg("deposit_entry_point_name");

    let new_purse = system::create_purse();
    system::transfer_from_purse_to_purse(
        account::get_main_purse(),
        new_purse,
        deposit_amount,
        None,
    )
    .unwrap_or_revert_with(Error::ExcessiveAmount);

    let deposit_entry_point_args = retrieve_deposit_entry_point_name_args(
        deposit_entry_point_name.clone(),
        new_purse,
        u512_to_u256(deposit_amount),
    );

    if deposit_entry_point_args.is_empty() {
        runtime::revert(Error::InvalidDepositEntryPointName);
    }

    // Call the contract's entry_point to execute its original logic.
    // runtime::call_contract::<()>(
    //     contract_hash,
    //     &deposit_entry_point_name,
    //     deposit_entry_point_args,
    // );

    // let marketplace_package_input: Key = runtime::get_named_arg::<Key>("marketplace_package");

    // let marketplace_package_hash_add: HashAddr =
    //     marketplace_package_input.into_hash().unwrap_or_default();
    // let marketplace_package_hash: ContractPackageHash =
    //     ContractPackageHash::new(marketplace_package_hash_add);

    // runtime::call_versioned_contract::<()>(
    //     marketplace_package_hash,
    //     None,
    //     &deposit_entry_point_name,
    //     deposit_entry_point_args,
    // );
    let box_contract_hash: Key = runtime::get_named_arg("box_contract_hash");

    let contract_hash_addr: HashAddr = box_contract_hash.into_hash().unwrap_or_revert();
    let box_contract_hash: ContractHash = ContractHash::new(contract_hash_addr);

    let _: (String, Key, String) = runtime::call_contract(
        box_contract_hash,
        &deposit_entry_point_name,
        deposit_entry_point_args,
    );
}

fn retrieve_deposit_entry_point_name_args(
    deposit_entry_point_name: String,
    purse: URef,
    amount: U256,
) -> RuntimeArgs {
    if deposit_entry_point_name == "mint" {
        retrieve_mint_args(purse)
    // } else if deposit_entry_point_name == "bid" {
    //     retrieve_bid_args(purse, amount)
    // } else if deposit_entry_point_name == "increase_bid" {
    //     retrieve_increase_bid_args(purse, amount)
    } else {
        runtime_args! {}
    }
}

fn retrieve_mint_args(src_purse: URef) -> RuntimeArgs {
    runtime_args! {
        "token_meta_data" => runtime::get_named_arg::<String>("token_metadata"),
        "src_purse" => src_purse,
        "token_owner" => runtime::get_named_arg::<Key>("token_owner"),
    }
}

// fn retrieve_bid_args(src_purse: URef, amount: U256) -> RuntimeArgs {
//     runtime_args! {
//         "amount" => amount,
//         "src_purse" => src_purse,
//         // "sell_index" => runtime::get_named_arg::<u64>("sell_index"),
//         "nft_contract_hash" => runtime::get_named_arg::<Key>("nft_contract_hash"),
//         "token_id" => runtime::get_named_arg::<String>("token_id"),
//         "bidder" => runtime::get_named_arg::<Key>("bidder"),
//     }
// }
// fn retrieve_increase_bid_args(src_purse: URef, amount: U256) -> RuntimeArgs {
//     runtime_args! {
//         "amount" => amount,
//         "src_purse" => src_purse,
//         // "sell_index" => runtime::get_named_arg::<u64>("sell_index"),
//         "nft_contract_hash" => runtime::get_named_arg::<Key>("nft_contract_hash"),
//         "token_id" => runtime::get_named_arg::<String>("token_id"),
//         "bidder" => runtime::get_named_arg::<Key>("bidder"),
//     }
//}
