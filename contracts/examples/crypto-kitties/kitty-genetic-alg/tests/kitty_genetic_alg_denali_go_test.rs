#[test]
fn generate_kitty_genes_go() {
    numbat_wasm_debug::denali_go("denali/generate-kitty-genes.scen.json");
}

#[test]
fn init_go() {
    numbat_wasm_debug::denali_go("denali/init.scen.json");
}
