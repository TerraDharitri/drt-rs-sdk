use numbat_wasm::*;
use numbat_wasm_debug::*;

#[allow(unused)]
fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/rewa-dcdt-swap.wasm",
        Box::new(|context| Box::new(rewa_dcdt_swap::contract_obj(context))),
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
