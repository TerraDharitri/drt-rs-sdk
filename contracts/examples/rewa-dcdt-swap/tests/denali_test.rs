use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/rewa-dcdt-swap.wasm",
		Box::new(|context| Box::new(rewa_dcdt_swap::contract_obj(context))),
	);
	contract_map
}

#[test]
fn wrap_rewa_test() {
	parse_execute_denali("denali/wrap_rewa.scen.json", &contract_map());
}

#[test]
fn wrap_then_unwrap_rewa_test() {
	parse_execute_denali("denali/unwrap_rewa.scen.json", &contract_map());
}
