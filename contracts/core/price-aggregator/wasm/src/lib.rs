// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           21
// Async Callback (empty):               1
// Total number of exported functions:  24

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    dharitri_sc_price_aggregator
    (
        init => init
        upgrade => upgrade
        changeAmounts => change_amounts
        addOracles => add_oracles
        removeOracles => remove_oracles
        submit => submit
        submitBatch => submit_batch
        latestRoundData => latest_round_data
        latestPriceFeed => latest_price_feed
        latestPriceFeedOptional => latest_price_feed_optional
        setSubmissionCount => set_submission_count
        getOracles => get_oracles
        setPairDecimals => set_pair_decimals
        getPairDecimals => get_pair_decimals
        submission_count => submission_count
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
        stake => stake
        unstake => unstake
        voteSlashMember => vote_slash_member
        cancelVoteSlashMember => cancel_vote_slash_member
        slashMember => slash_member
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}