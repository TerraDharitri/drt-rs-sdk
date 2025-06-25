use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/crowdfunding-dcdt.wasm",
		Box::new(|context| Box::new(crowdfunding_dcdt::contract_obj(context))),
	);
	contract_map
}

#[test]
fn crowdfunding_claim_failed_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/crowdfunding-claim-failed.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_claim_successful_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/crowdfunding-claim-successful.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_claim_too_early_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/crowdfunding-claim-too-early.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_fund_rs() {
	numbat_wasm_debug::denali_rs("denali/crowdfunding-fund.scen.json", &contract_map());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/crowdfunding-fund-too-late.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_init_rs() {
	numbat_wasm_debug::denali_rs("denali/crowdfunding-init.scen.json", &contract_map());
}

#[test]
fn rewa_crowdfunding_claim_failed_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/rewa-crowdfunding-claim-failed.scen.json",
		&contract_map(),
	);
}

#[test]
fn rewa_crowdfunding_claim_successful_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/rewa-crowdfunding-claim-successful.scen.json",
		&contract_map(),
	);
}

#[test]
fn rewa_crowdfunding_claim_too_early_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/rewa-crowdfunding-claim-too-early.scen.json",
		&contract_map(),
	);
}

#[test]
fn rewa_crowdfunding_fund_rs() {
	numbat_wasm_debug::denali_rs("denali/rewa-crowdfunding-fund.scen.json", &contract_map());
}

#[test]
fn rewa_crowdfunding_fund_too_late_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/rewa-crowdfunding-fund-too-late.scen.json",
		&contract_map(),
	);
}

#[test]
fn rewa_crowdfunding_init_rs() {
	numbat_wasm_debug::denali_rs("denali/rewa-crowdfunding-init.scen.json", &contract_map());
}
