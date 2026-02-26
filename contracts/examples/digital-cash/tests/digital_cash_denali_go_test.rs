#[test]
fn claim_rewa_go() {
    numbat_wasm_debug::denali_go("denali/claim-rewa.scen.json");
}

#[test]
fn claim_dcdt_go() {
    numbat_wasm_debug::denali_go("denali/claim-dcdt.scen.json");
}

#[test]
fn fund_rewa_and_dcdt_go() {
    numbat_wasm_debug::denali_go("denali/fund-rewa-and-dcdt.scen.json");
}

#[test]
fn set_accounts_go() {
    numbat_wasm_debug::denali_go("denali/set-accounts.scen.json");
}

#[test]
fn withdraw_rewa_go() {
    numbat_wasm_debug::denali_go("denali/withdraw-rewa.scen.json");
}

#[test]
fn withdraw_dcdt_go() {
    numbat_wasm_debug::denali_go("denali/withdraw-dcdt.scen.json");
}
