use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

// verify_ed25519 not implemented
// #[test]
// fn claim_rewa_rs() {
//     numbat_wasm_debug::denali_rs("denali/claim-rewa.scen.json", world());
// }

// verify_ed25519 not implemented
// #[test]
// fn claim_dcdt_rs() {
//     numbat_wasm_debug::denali_rs("denali/claim-dcdt.scen.json", world());
// }

#[test]
fn fund_rewa_and_dcdt_rs() {
    numbat_wasm_debug::denali_rs("denali/fund-rewa-and-dcdt.scen.json", world());
}

#[test]
fn set_accounts_rs() {
    numbat_wasm_debug::denali_rs("denali/set-accounts.scen.json", world());
}

#[test]
fn withdraw_rewa_rs() {
    numbat_wasm_debug::denali_rs("denali/withdraw-rewa.scen.json", world());
}

#[test]
fn withdraw_dcdt_rs() {
    numbat_wasm_debug::denali_rs("denali/withdraw-dcdt.scen.json", world());
}
