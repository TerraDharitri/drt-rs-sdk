use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:output/rewa-dcdt-swap.wasm",
        rewa_dcdt_swap::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_rewa_rs() {
    numbat_wasm_debug::denali_rs("denali/unwrap_rewa.scen.json", world());
}

#[test]
fn wrap_rewa_rs() {
    numbat_wasm_debug::denali_rs("denali/wrap_rewa.scen.json", world());
}
