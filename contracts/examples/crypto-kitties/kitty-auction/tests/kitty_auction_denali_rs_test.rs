use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();

	contract_map.register_contract(
		"file:../../kitty-ownership/output/kitty-ownership.wasm",
		Box::new(|context| Box::new(kitty_ownership::contract_obj(context))),
	);
	contract_map.register_contract(
		"file:../output/kitty-auction.wasm",
		Box::new(|context| Box::new(kitty_auction::contract_obj(context))),
	);

	contract_map
}
#[test]
fn bid_first_rs() {
	numbat_wasm_debug::denali_rs("denali/bid_first.scen.json", &contract_map());
}

#[test]
fn bid_second_max_rs() {
	numbat_wasm_debug::denali_rs("denali/bid_second_max.scen.json", &contract_map());
}

#[test]
fn bid_second_ok_rs() {
	numbat_wasm_debug::denali_rs("denali/bid_second_ok.scen.json", &contract_map());
}

#[test]
fn bid_second_too_low_rs() {
	numbat_wasm_debug::denali_rs("denali/bid_second_too_low.scen.json", &contract_map());
}

#[test]
fn bid_siring_auction_rs() {
	numbat_wasm_debug::denali_rs("denali/bid_siring_auction.scen.json", &contract_map());
}

#[test]
fn create_and_auction_gen_zero_kitty_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/create_and_auction_gen_zero_kitty.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_sale_auction_not_owner_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/create_sale_auction_not_owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_sale_auction_ok_rs() {
	numbat_wasm_debug::denali_rs("denali/create_sale_auction_ok.scen.json", &contract_map());
}

#[test]
fn create_siring_auction_not_owner_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/create_siring_auction_not_owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_siring_auction_ok_rs() {
	numbat_wasm_debug::denali_rs("denali/create_siring_auction_ok.scen.json", &contract_map());
}

#[test]
fn end_auction_no_bids_rs() {
	numbat_wasm_debug::denali_rs("denali/end_auction_no_bids.scen.json", &contract_map());
}

#[test]
fn end_auction_second_bid_max_early_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/end_auction_second_bid_max_early.scen.json",
		&contract_map(),
	);
}

#[test]
fn end_auction_second_bid_ok_early_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/end_auction_second_bid_ok_early.scen.json",
		&contract_map(),
	);
}

#[test]
fn end_auction_second_bid_ok_late_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/end_auction_second_bid_ok_late.scen.json",
		&contract_map(),
	);
}

#[test]
fn end_siring_auction_rs() {
	numbat_wasm_debug::denali_rs("denali/end_siring_auction.scen.json", &contract_map());
}

#[test]
fn init_rs() {
	numbat_wasm_debug::denali_rs("denali/init.scen.json", &contract_map());
}
