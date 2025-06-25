use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/dcdt-system-sc-mock");
    blockchain.register_contract(
        "file:output/dcdt-system-sc-mock.wasm",
        dcdt_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn issue_rs() {
    dharitri_sc_scenario::run_rs("scenarios/dcdt_system_sc.scen.json", world());
}
