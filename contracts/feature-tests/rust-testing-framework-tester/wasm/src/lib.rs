// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           26
// Async Callback:                       1
// Total number of exported functions:  28

#![no_std]

dharitri_sc_wasm_adapter::allocator!(static64k);
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    rust_testing_framework_tester
    (
        init => init
        sum => sum
        sum_sc_result => sum_sc_result
        get_caller_legacy => get_caller_legacy
        get_moa_balance => get_moa_balance
        get_dcdt_balance => get_dcdt_balance
        receive_moa => receive_moa
        recieve_moa_half => recieve_moa_half
        receive_dcdt => receive_dcdt
        reject_payment => reject_payment
        receive_dcdt_half => receive_dcdt_half
        receive_multi_dcdt => receive_multi_dcdt
        send_nft => send_nft
        mint_dcdt => mint_dcdt
        burn_dcdt => burn_dcdt
        create_nft => create_nft
        get_block_epoch => get_block_epoch
        get_block_nonce => get_block_nonce
        get_block_timestamp => get_block_timestamp
        get_random_buffer_once => get_random_buffer_once
        get_random_buffer_twice => get_random_buffer_twice
        call_other_contract_execute_on_dest => call_other_contract_execute_on_dest
        call_other_contract_add_async_call => call_other_contract_add_async_call
        getTotalValue => get_total_value
        execute_on_dest_add_value => execute_on_dest_add_value
        addValue => add
        panic => panic
    )
}

dharitri_sc_wasm_adapter::async_callback! { rust_testing_framework_tester }
