use dharitri_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-dcdt");
    blockchain.register_contract(
        "drtsc:output/crowdfunding-dcdt.drtsc.json",
        crowdfunding_dcdt::ContractBuilder,
    );

    blockchain
}

#[test]
fn generated_fund_rs() {
    world().run("scenarios/_generated_fund.scen.json");
}

#[test]
fn generated_init_rs() {
    world().run("scenarios/_generated_init.scen.json");
}

#[test]
fn generated_query_status_rs() {
    world().run("scenarios/_generated_query_status.scen.json");
}

#[test]
fn generated_sc_err_rs() {
    world().run("scenarios/_generated_sc_err.scen.json");
}

#[test]
fn crowdfunding_claim_failed_rs() {
    world().run("scenarios/crowdfunding-claim-failed.scen.json");
}

#[test]
fn crowdfunding_claim_successful_rs() {
    world().run("scenarios/crowdfunding-claim-successful.scen.json");
}

#[test]
fn crowdfunding_claim_too_early_rs() {
    world().run("scenarios/crowdfunding-claim-too-early.scen.json");
}

#[test]
fn crowdfunding_fund_rs() {
    world().run("scenarios/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_fund_too_late_rs() {
    world().run("scenarios/crowdfunding-fund-too-late.scen.json");
}

#[test]
fn crowdfunding_init_rs() {
    world().run("scenarios/crowdfunding-init.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_failed_rs() {
    world().run("scenarios/rewa-crowdfunding-claim-failed.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_successful_rs() {
    world().run("scenarios/rewa-crowdfunding-claim-successful.scen.json");
}

#[test]
fn rewa_crowdfunding_claim_too_early_rs() {
    world().run("scenarios/rewa-crowdfunding-claim-too-early.scen.json");
}

#[test]
fn rewa_crowdfunding_fund_rs() {
    world().run("scenarios/rewa-crowdfunding-fund.scen.json");
}

#[test]
fn rewa_crowdfunding_fund_too_late_rs() {
    world().run("scenarios/rewa-crowdfunding-fund-too-late.scen.json");
}

#[test]
fn rewa_crowdfunding_init_rs() {
    world().run("scenarios/rewa-crowdfunding-init.scen.json");
}
