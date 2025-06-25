#[test]
fn approve_siring_go() {
    numbat_wasm_debug::denali_go("denali/approve_siring.scen.json");
}

#[test]
fn breed_ok_go() {
    numbat_wasm_debug::denali_go("denali/breed_ok.scen.json");
}

#[test]
fn give_birth_go() {
    numbat_wasm_debug::denali_go("denali/give_birth.scen.json");
}

#[test]
fn init_go() {
    numbat_wasm_debug::denali_go("denali/init.scen.json");
}

#[test]
fn query_go() {
    numbat_wasm_debug::denali_go("denali/query.scen.json");
}

#[test]
fn setup_accounts_go() {
    numbat_wasm_debug::denali_go("denali/setup_accounts.scen.json");
}
