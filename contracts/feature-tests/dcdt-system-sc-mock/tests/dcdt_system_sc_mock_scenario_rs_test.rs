use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "drtsc:output/dcdt-system-sc-mock.drtsc.json",
        dcdt_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn dcdt_system_sc_rs() {
    world().run("scenarios/dcdt_system_sc.scen.json");
}
