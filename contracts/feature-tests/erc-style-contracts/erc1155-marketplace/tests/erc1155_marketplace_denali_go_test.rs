#[test]
fn auction_batch_go() {
    numbat_wasm_debug::denali_go("denali/auction_batch.scen.json");
}

#[test]
fn auction_single_token_rewa_go() {
    numbat_wasm_debug::denali_go("denali/auction_single_token_rewa.scen.json");
}

#[test]
fn bid_first_rewa_go() {
    numbat_wasm_debug::denali_go("denali/bid_first_rewa.scen.json");
}

#[test]
fn bid_second_rewa_go() {
    numbat_wasm_debug::denali_go("denali/bid_second_rewa.scen.json");
}

#[test]
fn bid_third_rewa_go() {
    numbat_wasm_debug::denali_go("denali/bid_third_rewa.scen.json");
}

#[test]
fn end_auction_go() {
    numbat_wasm_debug::denali_go("denali/end_auction.scen.json");
}
