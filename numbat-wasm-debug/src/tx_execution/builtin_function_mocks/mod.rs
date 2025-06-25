mod builtin_func_exec;
mod change_owner_mock;
mod dcdt_local_burn;
mod dcdt_local_mint;
mod dcdt_multi_transfer_mock;
mod dcdt_nft_add_quantity_mock;
mod dcdt_nft_add_uri_mock;
mod dcdt_nft_burn_mock;
mod dcdt_nft_create_mock;
mod dcdt_nft_transfer_mock;
mod dcdt_nft_update_attriutes_mock;
mod dcdt_transfer_mock;
mod set_username_mock;
mod upgrade_contract;

pub use builtin_func_exec::execute_builtin_function_or_default;
