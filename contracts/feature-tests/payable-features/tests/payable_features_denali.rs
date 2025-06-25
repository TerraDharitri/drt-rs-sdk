use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/payable-features.wasm",
		Box::new(|context| Box::new(payable_features::contract_obj(context))),
	);
	contract_map
}

#[test]
fn payable_any_1() {
	parse_execute_denali("denali/payable_any_1.scen.json", &contract_map());
}

#[test]
fn payable_any_2() {
	parse_execute_denali("denali/payable_any_2.scen.json", &contract_map());
}

#[test]
fn payable_any_3() {
	parse_execute_denali("denali/payable_any_3.scen.json", &contract_map());
}

#[test]
fn payable_any_4() {
	parse_execute_denali("denali/payable_any_4.scen.json", &contract_map());
}

#[test]
fn payable_rewa_0() {
	parse_execute_denali("denali/payable_rewa_0.scen.json", &contract_map());
}

#[test]
fn payable_rewa_1() {
	parse_execute_denali("denali/payable_rewa_1.scen.json", &contract_map());
}

#[test]
fn payable_rewa_2() {
	parse_execute_denali("denali/payable_rewa_2.scen.json", &contract_map());
}

#[test]
fn payable_rewa_3() {
	parse_execute_denali("denali/payable_rewa_3.scen.json", &contract_map());
}

#[test]
fn payable_rewa_4() {
	parse_execute_denali("denali/payable_rewa_4.scen.json", &contract_map());
}

#[test]
fn payable_token_1() {
	parse_execute_denali("denali/payable_token_1.scen.json", &contract_map());
}

#[test]
fn payable_token_2() {
	parse_execute_denali("denali/payable_token_2.scen.json", &contract_map());
}

#[test]
fn payable_token_3() {
	parse_execute_denali("denali/payable_token_3.scen.json", &contract_map());
}

#[test]
fn payable_token_4() {
	parse_execute_denali("denali/payable_token_4.scen.json", &contract_map());
}
