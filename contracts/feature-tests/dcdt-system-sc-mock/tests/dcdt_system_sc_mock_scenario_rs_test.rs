use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
#[ignore = "builtin SC not implemented"]
fn dcdt_system_sc_rs() {
    world().run("scenarios/dcdt_system_sc.scen.json");
}
