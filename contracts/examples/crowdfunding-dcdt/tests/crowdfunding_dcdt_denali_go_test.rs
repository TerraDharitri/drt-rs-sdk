#[test]
fn crowdfunding_claim_failed_go() {
    numbat_wasm_debug::denali_go("denali/crowdfunding-claim-failed.scen.json");
}

#[test]
fn crowdfunding_claim_successful_go() {
    numbat_wasm_debug::denali_go("denali/crowdfunding-claim-successful.scen.json");
}

#[test]
fn crowdfunding_claim_too_early_go() {
    numbat_wasm_debug::denali_go("denali/crowdfunding-claim-too-early.scen.json");
}

#[test]
fn crowdfunding_fund_go() {
    numbat_wasm_debug::denali_go("denali/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_fund_too_late_go() {
    numbat_wasm_debug::denali_go("denali/crowdfunding-fund-too-late.scen.json");
}

#[test]
fn crowdfunding_init_go() {
    numbat_wasm_debug::denali_go("denali/crowdfunding-init.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_failed_go() {
    numbat_wasm_debug::denali_go("denali/rewa-crowdfunding-claim-failed.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_successful_go() {
    numbat_wasm_debug::denali_go("denali/rewa-crowdfunding-claim-successful.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_too_early_go() {
    numbat_wasm_debug::denali_go("denali/rewa-crowdfunding-claim-too-early.scen.json");
}

#[test]
fn rewa_crowdfunding_fund_go() {
    numbat_wasm_debug::denali_go("denali/rewa-crowdfunding-fund.scen.json");
}

#[test]
fn rewa_crowdfunding_fund_too_late_go() {
    numbat_wasm_debug::denali_go("denali/rewa-crowdfunding-fund-too-late.scen.json");
}

#[test]
fn rewa_crowdfunding_init_go() {
    numbat_wasm_debug::denali_go("denali/rewa-crowdfunding-init.scen.json");
}
