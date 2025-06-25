#[test]
fn claim_rewa_go() {
    dharitri_sc_scenario::run_go("scenarios/claim-rewa.scen.json");
}

#[test]
fn claim_dcdt_go() {
    dharitri_sc_scenario::run_go("scenarios/claim-dcdt.scen.json");
}

#[test]
fn claim_fees_go() {
    dharitri_sc_scenario::run_go("scenarios/claim-fees.scen.json");
}

#[test]
fn fund_rewa_and_dcdt_go() {
    dharitri_sc_scenario::run_go("scenarios/fund-rewa-and-dcdt.scen.json");
}

#[test]
fn set_accounts_go() {
    dharitri_sc_scenario::run_go("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_rewa_go() {
    dharitri_sc_scenario::run_go("scenarios/withdraw-rewa.scen.json");
}

#[test]
fn withdraw_dcdt_go() {
    dharitri_sc_scenario::run_go("scenarios/withdraw-dcdt.scen.json");
}

#[test]
fn forward_go() {
    dharitri_sc_scenario::run_go("scenarios/forward.scen.json");
}
