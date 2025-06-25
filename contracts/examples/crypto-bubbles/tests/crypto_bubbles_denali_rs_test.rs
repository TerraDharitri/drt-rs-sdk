use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/crypto-bubbles.wasm",
		Box::new(|context| Box::new(crypto_bubbles::contract_obj(context))),
	);
	contract_map
}

#[test]
fn balanceof_rs() {
	numbat_wasm_debug::denali_rs("denali/balanceOf.scen.json", &contract_map());
}

#[test]
fn create_rs() {
	numbat_wasm_debug::denali_rs("denali/create.scen.json", &contract_map());
}

#[test]
fn exceptions_rs() {
	numbat_wasm_debug::denali_rs("denali/exceptions.scen.json", &contract_map());
}

#[test]
fn joingame_rs() {
	numbat_wasm_debug::denali_rs("denali/joinGame.scen.json", &contract_map());
}

#[test]
fn rewardandsendtowallet_rs() {
	numbat_wasm_debug::denali_rs("denali/rewardAndSendToWallet.scen.json", &contract_map());
}

#[test]
fn rewardwinner_rs() {
	numbat_wasm_debug::denali_rs("denali/rewardWinner.scen.json", &contract_map());
}

#[test]
fn rewardwinner_last_rs() {
	numbat_wasm_debug::denali_rs("denali/rewardWinner_Last.scen.json", &contract_map());
}

#[test]
fn topup_ok_rs() {
	numbat_wasm_debug::denali_rs("denali/topUp_ok.scen.json", &contract_map());
}

#[test]
fn topup_withdraw_rs() {
	numbat_wasm_debug::denali_rs("denali/topUp_withdraw.scen.json", &contract_map());
}

#[test]
fn withdraw_ok_rs() {
	numbat_wasm_debug::denali_rs("denali/withdraw_Ok.scen.json", &contract_map());
}

#[test]
fn withdraw_toomuch_rs() {
	numbat_wasm_debug::denali_rs("denali/withdraw_TooMuch.scen.json", &contract_map());
}
