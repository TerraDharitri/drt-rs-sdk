use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/factorial");

    blockchain.register_contract("file:output/factorial.wasm", factorial::ContractBuilder);
    blockchain
}

#[test]
fn factorial_rs() {
    numbat_wasm_debug::denali_rs("denali/factorial.scen.json", world());
}
