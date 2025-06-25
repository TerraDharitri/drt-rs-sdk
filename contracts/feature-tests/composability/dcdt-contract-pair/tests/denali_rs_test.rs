use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:first-contract/output/first-contract.wasm",
        first_contract::ContractBuilder,
    );

    blockchain.register_contract_builder(
        "file:second-contract/output/second-contract.wasm",
        second_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() {
    numbat_wasm_debug::denali_rs("denali/init.scen.json", world());
}

#[test]
fn simple_transfer_full_rs() {
    numbat_wasm_debug::denali_rs("denali/simple_transfer_full.scen.json", world());
}

#[test]
fn simple_transfer_half_rs() {
    numbat_wasm_debug::denali_rs("denali/simple_transfer_half.scen.json", world());
}

#[test]
fn simple_transfer_full_wrong_token_rs() {
    numbat_wasm_debug::denali_rs("denali/simple_transfer_full_wrong_token.scen.json", world());
}

// TODO: implement DCDTTransfer + async call
// #[test]
// fn rejected_transfer_rs() {
// 	numbat_wasm_debug::denali_rs("denali/reject_transfer.scen.json", world());
// }
