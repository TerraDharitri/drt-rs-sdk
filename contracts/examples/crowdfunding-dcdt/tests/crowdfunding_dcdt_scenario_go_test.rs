#[test]
fn crowdfunding_claim_failed_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-claim-failed.scen.json");
}

#[test]
fn crowdfunding_claim_successful_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-claim-successful.scen.json");
}

#[test]
fn crowdfunding_claim_too_early_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-claim-too-early.scen.json");
}

#[test]
fn crowdfunding_fund_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_fund_too_late_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-fund-too-late.scen.json");
}

#[test]
fn crowdfunding_init_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-init.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_failed_go() {
    dharitri_sc_scenario::run_go("scenarios/rewa-crowdfunding-claim-failed.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_successful_go() {
    dharitri_sc_scenario::run_go("scenarios/rewa-crowdfunding-claim-successful.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_too_early_go() {
    dharitri_sc_scenario::run_go("scenarios/rewa-crowdfunding-claim-too-early.scen.json");
}

#[test]
fn rewa_crowdfunding_fund_go() {
    dharitri_sc_scenario::run_go("scenarios/rewa-crowdfunding-fund.scen.json");
}

#[test]
fn rewa_crowdfunding_fund_too_late_go() {
    dharitri_sc_scenario::run_go("scenarios/rewa-crowdfunding-fund-too-late.scen.json");
}

#[test]
fn rewa_crowdfunding_init_go() {
    dharitri_sc_scenario::run_go("scenarios/rewa-crowdfunding-init.scen.json");
}
