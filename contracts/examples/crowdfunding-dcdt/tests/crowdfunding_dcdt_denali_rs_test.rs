use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-dcdt");

    blockchain.register_contract_builder(
        "file:output/crowdfunding-dcdt.wasm",
        crowdfunding_dcdt::ContractBuilder,
    );
    blockchain
}

#[test]
fn crowdfunding_claim_failed_rs() {
    numbat_wasm_debug::denali_rs("denali/crowdfunding-claim-failed.scen.json", world());
}

#[test]
fn crowdfunding_claim_successful_rs() {
    numbat_wasm_debug::denali_rs("denali/crowdfunding-claim-successful.scen.json", world());
}

#[test]
fn crowdfunding_claim_too_early_rs() {
    numbat_wasm_debug::denali_rs("denali/crowdfunding-claim-too-early.scen.json", world());
}

#[test]
fn crowdfunding_fund_rs() {
    numbat_wasm_debug::denali_rs("denali/crowdfunding-fund.scen.json", world());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
    numbat_wasm_debug::denali_rs("denali/crowdfunding-fund-too-late.scen.json", world());
}

#[test]
fn crowdfunding_init_rs() {
    numbat_wasm_debug::denali_rs("denali/crowdfunding-init.scen.json", world());
}

#[test]
fn rewa_crowdfunding_claim_failed_rs() {
    numbat_wasm_debug::denali_rs("denali/rewa-crowdfunding-claim-failed.scen.json", world());
}

#[test]
fn rewa_crowdfunding_claim_successful_rs() {
    numbat_wasm_debug::denali_rs(
        "denali/rewa-crowdfunding-claim-successful.scen.json",
        world(),
    );
}

#[test]
fn rewa_crowdfunding_claim_too_early_rs() {
    numbat_wasm_debug::denali_rs(
        "denali/rewa-crowdfunding-claim-too-early.scen.json",
        world(),
    );
}

#[test]
fn rewa_crowdfunding_fund_rs() {
    numbat_wasm_debug::denali_rs("denali/rewa-crowdfunding-fund.scen.json", world());
}

#[test]
fn rewa_crowdfunding_fund_too_late_rs() {
    numbat_wasm_debug::denali_rs("denali/rewa-crowdfunding-fund-too-late.scen.json", world());
}

#[test]
fn rewa_crowdfunding_init_rs() {
    numbat_wasm_debug::denali_rs("denali/rewa-crowdfunding-init.scen.json", world());
}
