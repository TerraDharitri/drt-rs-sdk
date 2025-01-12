use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");
    blockchain.register_contract(
        "drtsc:output/payable-features.drtsc.json",
        payable_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn call_value_check_rs() {
    world().run("scenarios/call-value-check.scen.json");
}

#[test]
fn call_value_check_multi_moa_rs() {
    world().run("scenarios/call-value-check-multi-moa.scen.json");
}

#[test]
fn payable_all_transfers_rs() {
    world().run("scenarios/payable_all_transfers.scen.json");
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
fn payable_moa_1_rs() {
    world().run("scenarios/payable_moa_1.scen.json");
}

#[test]
fn payable_moa_2_rs() {
    world().run("scenarios/payable_moa_2.scen.json");
}

#[test]
fn payable_moa_3_rs() {
    world().run("scenarios/payable_moa_3.scen.json");
}

#[test]
fn payable_moa_4_rs() {
    world().run("scenarios/payable_moa_4.scen.json");
}

#[test]
fn payable_multi_array_rs() {
    world().run("scenarios/payable_multi_array.scen.json");
}

#[test]
fn payable_multi_array_moa_rs() {
    world().run("scenarios/payable_multi_array_moa.scen.json");
}

#[test]
fn payable_multiple_rs() {
    world().run("scenarios/payable_multiple.scen.json");
}

#[test]
fn payable_multiple_moa_rs() {
    world().run("scenarios/payable_multiple_moa.scen.json");
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
