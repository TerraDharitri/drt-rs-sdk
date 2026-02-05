use dharitri_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn payable_all_1_go() {
    world().run("scenarios/payable_all_1.scen.json");
}

#[test]
fn payable_all_2_go() {
    world().run("scenarios/payable_all_2.scen.json");
}

#[test]
fn payable_all_3_go() {
    world().run("scenarios/payable_all_3.scen.json");
}

#[test]
fn payable_all_transfers_1_go() {
    world().run("scenarios/payable_all_transfers_1.scen.json");
}

#[test]
fn payable_all_transfers_2_go() {
    world().run("scenarios/payable_all_transfers_2.scen.json");
}

#[test]
fn payable_any_1_go() {
    world().run("scenarios/payable_any_1.scen.json");
}

#[test]
fn payable_any_2_go() {
    world().run("scenarios/payable_any_2.scen.json");
}

#[test]
fn payable_any_3_go() {
    world().run("scenarios/payable_any_3.scen.json");
}

#[test]
fn payable_any_4_go() {
    world().run("scenarios/payable_any_4.scen.json");
}

#[test]
fn payable_any_5_go() {
    world().run("scenarios/payable_any_5.scen.json");
}

#[test]
fn payable_array_go() {
    world().run("scenarios/payable_array.scen.json");
}

#[test]
fn payable_array_rewa_or_dcdt_go() {
    world().run("scenarios/payable_array_rewa_or_dcdt.scen.json");
}

#[test]
fn payable_array_dcdt_go() {
    world().run("scenarios/payable_array_dcdt.scen.json");
}

#[test]
fn payable_rewa_1_go() {
    world().run("scenarios/payable_rewa_1.scen.json");
}

#[test]
fn payable_rewa_2_go() {
    world().run("scenarios/payable_rewa_2.scen.json");
}

#[test]
fn payable_rewa_3_go() {
    world().run("scenarios/payable_rewa_3.scen.json");
}

#[test]
fn payable_rewa_4_go() {
    world().run("scenarios/payable_rewa_4.scen.json");
}

#[test]
fn payable_rewa_5_go() {
    world().run("scenarios/payable_rewa_5.scen.json");
}

#[test]
fn payable_legacy_rewa_dcdt_go() {
    world().run("scenarios/payable_legacy_rewa_dcdt.scen.json");
}

#[test]
fn payable_multiple_go() {
    world().run("scenarios/payable_multiple.scen.json");
}

#[test]
fn payable_multiple_rewa_go() {
    world().run("scenarios/payable_multiple_rewa.scen.json");
}

#[test]
fn payable_token_1_go() {
    world().run("scenarios/payable_token_1.scen.json");
}

#[test]
fn payable_token_2_go() {
    world().run("scenarios/payable_token_2.scen.json");
}

#[test]
fn payable_token_3_go() {
    world().run("scenarios/payable_token_3.scen.json");
}

#[test]
fn payable_token_4_go() {
    world().run("scenarios/payable_token_4.scen.json");
}

#[test]
fn payable_token_5_go() {
    world().run("scenarios/payable_token_5.scen.json");
}
