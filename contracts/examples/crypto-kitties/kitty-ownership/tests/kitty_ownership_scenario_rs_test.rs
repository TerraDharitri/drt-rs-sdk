use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/examples/crypto-kitties/kitty-ownership");
    blockchain.register_contract(
        "drtsc:../kitty-genetic-alg/output/kitty-genetic-alg.drtsc.json",
        kitty_genetic_alg::ContractBuilder,
    );
    blockchain.register_contract(
        "drtsc:output/kitty-ownership.drtsc.json",
        kitty_ownership::ContractBuilder,
    );

    blockchain
}

#[test]
fn approve_siring_rs() {
    world().run("scenarios/approve_siring.scen.json");
}

#[test]
fn breed_ok_rs() {
    world().run("scenarios/breed_ok.scen.json");
}

#[test]
fn give_birth_rs() {
    world().run("scenarios/give_birth.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn query_rs() {
    world().run("scenarios/query.scen.json");
}

#[test]
fn setup_accounts_rs() {
    world().run("scenarios/setup_accounts.scen.json");
}