use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "drtsc:output/large-storage.drtsc.json",
        large_storage::ContractBuilder,
    );
    blockchain
}

#[test]
fn large_storage_rs() {
    world().run("scenarios/large_storage.scen.json");
}
