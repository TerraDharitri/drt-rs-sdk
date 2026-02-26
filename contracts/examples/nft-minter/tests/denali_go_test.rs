#[test]
fn init_go() {
    numbat_wasm_debug::denali_go("denali/init.scen.json");
}

#[test]
fn create_nft_go() {
    numbat_wasm_debug::denali_go("denali/create_nft.scen.json");
}

#[test]
fn buy_nft_go() {
    numbat_wasm_debug::denali_go("denali/buy_nft.scen.json");
}
