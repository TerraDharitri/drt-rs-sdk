use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/kitty-genetic-alg.wasm",
		Box::new(|context| Box::new(kitty_genetic_alg::contract_obj(context))),
	);
	contract_map
}

#[test]
fn init() {
	parse_execute_denali("denali/init.scen.json", &contract_map());
}

#[test]
fn generate_kitty_genes_test() {
	parse_execute_denali("denali/generate-kitty-genes.scen.json", &contract_map());
}
