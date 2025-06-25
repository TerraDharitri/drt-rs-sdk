use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/adder.wasm",
		Box::new(|context| Box::new(adder::contract_obj(context))),
	);
	contract_map
}

#[test]
fn adder_rs() {
	numbat_wasm_debug::denali_rs("denali/adder.scen.json", &contract_map());
}
