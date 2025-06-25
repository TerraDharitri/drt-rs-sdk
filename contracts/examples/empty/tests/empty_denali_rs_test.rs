use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract("file:output/empty.wasm", empty::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    numbat_wasm_debug::denali_rs("denali/empty.scen.json", world());
}
