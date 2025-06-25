#[test]
fn auction_batch_go() {
    dharitri_sc_scenario::run_go("scenarios/auction_batch.scen.json");
}

#[test]
fn auction_single_token_rewa_go() {
    dharitri_sc_scenario::run_go("scenarios/auction_single_token_rewa.scen.json");
}

#[test]
fn bid_first_rewa_go() {
    dharitri_sc_scenario::run_go("scenarios/bid_first_rewa.scen.json");
}

#[test]
fn bid_second_rewa_go() {
    dharitri_sc_scenario::run_go("scenarios/bid_second_rewa.scen.json");
}

#[test]
fn bid_third_rewa_go() {
    dharitri_sc_scenario::run_go("scenarios/bid_third_rewa.scen.json");
}

#[test]
fn end_auction_go() {
    dharitri_sc_scenario::run_go("scenarios/end_auction.scen.json");
}
