use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/core/wmoa-swap");

    blockchain.register_contract(
        "drtsc:output/dharitri-sc-wmoa-swap.drtsc.json",
        dharitri_sc_wmoa_swap::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_moa_rs() {
    world().run("scenarios/unwrap_moa.scen.json");
}

#[test]
fn wrap_moa_rs() {
    world().run("scenarios/wrap_moa.scen.json");
}
