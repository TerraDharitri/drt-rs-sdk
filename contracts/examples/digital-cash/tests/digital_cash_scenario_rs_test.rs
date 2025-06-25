use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
#[ignore] // verify_ed25519 not implemented
fn claim_rewa_rs() {
    dharitri_sc_scenario::run_rs("scenarios/claim-rewa.scen.json", world());
}

#[test]
#[ignore] // verify_ed25519 not implemented
fn claim_dcdt_rs() {
    dharitri_sc_scenario::run_rs("scenarios/claim-dcdt.scen.json", world());
}

#[test]
fn fund_rewa_and_dcdt_rs() {
    dharitri_sc_scenario::run_rs("scenarios/fund-rewa-and-dcdt.scen.json", world());
}

#[test]
fn set_accounts_rs() {
    dharitri_sc_scenario::run_rs("scenarios/set-accounts.scen.json", world());
}

#[test]
fn withdraw_rewa_rs() {
    dharitri_sc_scenario::run_rs("scenarios/withdraw-rewa.scen.json", world());
}

#[test]
fn withdraw_dcdt_rs() {
    dharitri_sc_scenario::run_rs("scenarios/withdraw-dcdt.scen.json", world());
}

#[test]
fn forward_rs() {
    dharitri_sc_scenario::run_rs("scenarios/forward.scen.json", world());
}
