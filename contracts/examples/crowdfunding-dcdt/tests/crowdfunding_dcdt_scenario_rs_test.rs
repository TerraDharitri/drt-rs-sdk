use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-dcdt");

    blockchain.register_contract(
        "file:output/crowdfunding-dcdt.wasm",
        crowdfunding_dcdt::ContractBuilder,
    );
    blockchain
}

#[test]
fn crowdfunding_claim_failed_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-claim-failed.scen.json", world());
}

#[test]
fn crowdfunding_claim_successful_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-claim-successful.scen.json", world());
}

#[test]
fn crowdfunding_claim_too_early_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-claim-too-early.scen.json", world());
}

#[test]
fn crowdfunding_fund_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-fund.scen.json", world());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-fund-too-late.scen.json", world());
}

#[test]
fn crowdfunding_init_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-init.scen.json", world());
}

#[test]
fn rewa_crowdfunding_claim_failed_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/rewa-crowdfunding-claim-failed.scen.json",
        world(),
    );
}

#[test]
fn rewa_crowdfunding_claim_successful_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/rewa-crowdfunding-claim-successful.scen.json",
        world(),
    );
}

#[test]
fn rewa_crowdfunding_claim_too_early_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/rewa-crowdfunding-claim-too-early.scen.json",
        world(),
    );
}

#[test]
fn rewa_crowdfunding_fund_rs() {
    dharitri_sc_scenario::run_rs("scenarios/rewa-crowdfunding-fund.scen.json", world());
}

#[test]
fn rewa_crowdfunding_fund_too_late_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/rewa-crowdfunding-fund-too-late.scen.json",
        world(),
    );
}

#[test]
fn rewa_crowdfunding_init_rs() {
    dharitri_sc_scenario::run_rs("scenarios/rewa-crowdfunding-init.scen.json", world());
}
