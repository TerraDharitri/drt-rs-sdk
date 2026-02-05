use dharitri_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");
    blockchain.register_contract(
        "drtsc:output/payable-features.drtsc.json",
        payable_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn payable_all_1_rs() {
    world().run("scenarios/payable_all_1.scen.json");
}

#[test]
fn payable_all_2_rs() {
    world().run("scenarios/payable_all_2.scen.json");
}

#[test]
fn payable_all_3_rs() {
    world().run("scenarios/payable_all_3.scen.json");
}

#[test]
fn payable_all_transfers_1_rs() {
    world().run("scenarios/payable_all_transfers_1.scen.json");
}

#[test]
fn payable_all_transfers_2_rs() {
    world().run("scenarios/payable_all_transfers_2.scen.json");
}

#[test]
fn payable_any_1_rs() {
    world().run("scenarios/payable_any_1.scen.json");
}

#[test]
fn payable_any_2_rs() {
    world().run("scenarios/payable_any_2.scen.json");
}

#[test]
fn payable_any_3_rs() {
    world().run("scenarios/payable_any_3.scen.json");
}

#[test]
fn payable_any_4_rs() {
    world().run("scenarios/payable_any_4.scen.json");
}

#[test]
fn payable_any_5_rs() {
    world().run("scenarios/payable_any_5.scen.json");
}

#[test]
fn payable_array_rs() {
    world().run("scenarios/payable_array.scen.json");
}

#[test]
fn payable_array_rewa_or_dcdt_rs() {
    world().run("scenarios/payable_array_rewa_or_dcdt.scen.json");
}

#[test]
fn payable_array_dcdt_rs() {
    world().run("scenarios/payable_array_dcdt.scen.json");
}

#[test]
fn payable_rewa_1_rs() {
    world().run("scenarios/payable_rewa_1.scen.json");
}

#[test]
fn payable_rewa_2_rs() {
    world().run("scenarios/payable_rewa_2.scen.json");
}

#[test]
fn payable_rewa_3_rs() {
    world().run("scenarios/payable_rewa_3.scen.json");
}

#[test]
fn payable_rewa_4_rs() {
    world().run("scenarios/payable_rewa_4.scen.json");
}

#[test]
fn payable_rewa_5_rs() {
    world().run("scenarios/payable_rewa_5.scen.json");
}

#[test]
fn payable_legacy_rewa_dcdt_rs() {
    world().run("scenarios/payable_legacy_rewa_dcdt.scen.json");
}

#[test]
fn payable_multiple_rs() {
    world().run("scenarios/payable_multiple.scen.json");
}

#[test]
fn payable_multiple_rewa_rs() {
    world().run("scenarios/payable_multiple_rewa.scen.json");
}

#[test]
fn payable_token_1_rs() {
    world().run("scenarios/payable_token_1.scen.json");
}

#[test]
fn payable_token_2_rs() {
    world().run("scenarios/payable_token_2.scen.json");
}

#[test]
fn payable_token_3_rs() {
    world().run("scenarios/payable_token_3.scen.json");
}

#[test]
fn payable_token_4_rs() {
    world().run("scenarios/payable_token_4.scen.json");
}

#[test]
fn payable_token_5_rs() {
    world().run("scenarios/payable_token_5.scen.json");
}
