use numbat_wasm::*;
use numbat_wasm_debug::*;

fn _contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/crowdfunding-dcdt.wasm",
		Box::new(|context| Box::new(crowdfunding_dcdt::contract_obj(context))),
	);
	contract_map
}

/*
#[test]
fn crowdfunding_claim_failed() {
	parse_execute_denali(
		"denali/crowdfunding-claim-failed.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_claim_successful() {
	parse_execute_denali(
		"denali/crowdfunding-claim-successful.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_claim_too_early() {
	parse_execute_denali(
		"denali/crowdfunding-claim-too-early.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_fund_ok() {
	parse_execute_denali("denali/crowdfunding-fund.scen.json", &contract_map());
}

#[test]
fn crowdfunding_fund_too_late() {
	parse_execute_denali(
		"denali/crowdfunding-fund-too-late.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_init() {
	parse_execute_denali("denali/crowdfunding-init.scen.json", &contract_map());
}
*/
