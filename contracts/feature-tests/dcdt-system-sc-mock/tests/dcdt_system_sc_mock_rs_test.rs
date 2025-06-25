use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/dcdt-system-sc-mock");
    blockchain.register_contract_builder(
        "file:output/dcdt-system-sc-mock.wasm",
        dcdt_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn issue_rs() {
    numbat_wasm_debug::denali_rs("denali/dcdt_system_sc.scen.json", world());
}
