#[test]
fn external_pure_go() {
    numbat_wasm_debug::denali_go("denali/external-pure.scen.json");
}

#[test]
fn external_get_go() {
    numbat_wasm_debug::denali_go("denali/external-get.scen.json");
}
