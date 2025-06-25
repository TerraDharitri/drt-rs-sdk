use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../async-alice/output/async-alice.wasm",
		Box::new(|context| Box::new(async_alice::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../async-bob/output/async-bob.wasm",
		Box::new(|context| Box::new(async_bob::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../forwarder/output/forwarder.wasm",
		Box::new(|context| Box::new(forwarder::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../forwarder-raw/output/forwarder-raw.wasm",
		Box::new(|context| Box::new(forwarder_raw::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../vault/output/vault.wasm",
		Box::new(|context| Box::new(vault::contract_obj(context))),
	);

	contract_map
}

// #[test]
// fn forwarder_async_accept_rewa() {
// 	parse_execute_denali(
// 		"denali/forwarder_async_accept_rewa.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_async_accept_dcdt() {
// 	parse_execute_denali(
// 		"denali/forwarder_async_accept_dcdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_async_accept_rewa() {
// 	parse_execute_denali(
// 		"denali/forwarder_raw_async_accept_rewa.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_async_accept_dcdt() {
// 	parse_execute_denali(
// 		"denali/forwarder_raw_async_accept_dcdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_async_echo() {
// 	parse_execute_denali("denali/forwarder_raw_async_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_raw_direct_rewa() {
// 	parse_execute_denali(
// 		"denali/forwarder_raw_direct_rewa.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_direct_dcdt() {
// 	parse_execute_denali(
// 		"denali/forwarder_raw_direct_dcdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_sync_echo() {
// 	parse_execute_denali("denali/forwarder_raw_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_raw_sync_rewa() {
// 	parse_execute_denali("denali/forwarder_raw_sync_rewa.scen.json", &contract_map());
// }

// TODO: successive asyncs currently not supported
// #[test]
// fn forwarder_send_twice_rewa() {
// 	parse_execute_denali(
// 		"denali/forwarder_send_twice_rewa.scen.json",
// 		&contract_map(),
// 	);
// }

// TODO: successive asyncs currently not supported
// #[test]
// fn forwarder_send_twice_dcdt() {
// 	parse_execute_denali(
// 		"denali/forwarder_send_twice_dcdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_accept_rewa() {
// 	parse_execute_denali(
// 		"denali/forwarder_sync_accept_rewa.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_accept_dcdt() {
// 	parse_execute_denali(
// 		"denali/forwarder_sync_accept_dcdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_echo() {
// 	parse_execute_denali("denali/forwarder_sync_echo.scen.json", &contract_map());
// }

#[test]
fn message_othershard() {
	parse_execute_denali("denali/message_otherShard.scen.json", &contract_map());
}

#[test]
fn message_othershard_callback() {
	parse_execute_denali(
		"denali/message_otherShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn message_sameshard() {
	parse_execute_denali("denali/message_sameShard.scen.json", &contract_map());
}

#[test]
fn message_sameshard_callback() {
	parse_execute_denali(
		"denali/message_sameShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn payment_othershard() {
	parse_execute_denali("denali/payment_otherShard.scen.json", &contract_map());
}

#[test]
fn payment_othershard_callback() {
	parse_execute_denali(
		"denali/payment_otherShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn payment_sameshard() {
	parse_execute_denali("denali/payment_sameShard.scen.json", &contract_map());
}

#[test]
fn payment_sameshard_callback() {
	parse_execute_denali(
		"denali/payment_sameShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn send_rewa() {
	parse_execute_denali("denali/send_rewa.scen.json", &contract_map());
}

#[test]
fn send_dcdt() {
	parse_execute_denali("denali/send_dcdt.scen.json", &contract_map());
}
