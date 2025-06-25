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

#[test]
fn forw_raw_async_accept_rewa_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/forw_raw_async_accept_rewa.scen.json",
		&contract_map(),
	);
}

// #[test]
// fn forw_raw_async_accept_dcdt_rs() {
//	numbat_wasm_debug::denali_rs("denali/forw_raw_async_accept_dcdt.scen.json", &contract_map());
// }

#[test]
fn forw_raw_async_echo_rs() {
	numbat_wasm_debug::denali_rs("denali/forw_raw_async_echo.scen.json", &contract_map());
}

#[test]
fn forw_raw_direct_rewa_rs() {
	numbat_wasm_debug::denali_rs("denali/forw_raw_direct_rewa.scen.json", &contract_map());
}

#[test]
fn forw_raw_direct_dcdt_rs() {
	numbat_wasm_debug::denali_rs("denali/forw_raw_direct_dcdt.scen.json", &contract_map());
}

// #[test]
// fn forw_raw_sync_echo_rs() {
//	numbat_wasm_debug::denali_rs("denali/forw_raw_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forw_raw_sync_rewa_rs() {
//	numbat_wasm_debug::denali_rs("denali/forw_raw_sync_rewa.scen.json", &contract_map());
// }

#[test]
fn forwarder_call_async_accept_rewa_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/forwarder_call_async_accept_rewa.scen.json",
		&contract_map(),
	);
}

// #[test]
// fn forwarder_call_async_accept_dcdt_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_async_accept_dcdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_async_accept_nft_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_async_accept_nft.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_rewa_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_rewa.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_dcdt_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_dcdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_nft_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_nft.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_rewa_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_then_read_rewa.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_dcdt_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_then_read_dcdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_nft_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_sync_accept_then_read_nft.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_rewa_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_transf_exec_accept_rewa.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_rewa_twice_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_transf_exec_accept_rewa_twice.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_dcdt_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_transf_exec_accept_dcdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_dcdt_twice_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_transf_exec_accept_dcdt_twice.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_nft_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_transf_exec_accept_nft.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_return_values_rs() {
// 	numbat_wasm_debug::denali_rs(
// 		"denali/forwarder_call_transf_exec_accept_return_values.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_sft_twice_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_call_transf_exec_accept_sft_twice.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_nft_create_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_nft_create.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_nft_transfer_async_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_nft_transfer_async.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_nft_transfer_exec_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_nft_transfer_exec.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_send_twice_rewa_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_send_twice_rewa.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_send_twice_dcdt_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_send_twice_dcdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_sync_echo_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_sync_echo_range_rs() {
//	numbat_wasm_debug::denali_rs("denali/forwarder_sync_echo_range.scen.json", &contract_map());
// }

#[test]
fn message_othershard_rs() {
	numbat_wasm_debug::denali_rs("denali/message_otherShard.scen.json", &contract_map());
}

#[test]
fn message_othershard_callback_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/message_otherShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn message_sameshard_rs() {
	numbat_wasm_debug::denali_rs("denali/message_sameShard.scen.json", &contract_map());
}

#[test]
fn message_sameshard_callback_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/message_sameShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn payment_othershard_rs() {
	numbat_wasm_debug::denali_rs("denali/payment_otherShard.scen.json", &contract_map());
}

#[test]
fn payment_othershard_callback_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/payment_otherShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn payment_sameshard_rs() {
	numbat_wasm_debug::denali_rs("denali/payment_sameShard.scen.json", &contract_map());
}

#[test]
fn payment_sameshard_callback_rs() {
	numbat_wasm_debug::denali_rs(
		"denali/payment_sameShard_callback.scen.json",
		&contract_map(),
	);
}

// #[test]
// fn recursive_caller_rewa_1_rs() {
//	numbat_wasm_debug::denali_rs("denali/recursive_caller_rewa_1.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_rewa_2_rs() {
//	numbat_wasm_debug::denali_rs("denali/recursive_caller_rewa_2.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_rewa_x_rs() {
//	numbat_wasm_debug::denali_rs("denali/recursive_caller_rewa_x.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_dcdt_1_rs() {
//	numbat_wasm_debug::denali_rs("denali/recursive_caller_dcdt_1.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_dcdt_2_rs() {
//	numbat_wasm_debug::denali_rs("denali/recursive_caller_dcdt_2.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_dcdt_x_rs() {
//	numbat_wasm_debug::denali_rs("denali/recursive_caller_dcdt_x.scen.json", &contract_map());
// }

#[test]
fn send_rewa_rs() {
	numbat_wasm_debug::denali_rs("denali/send_rewa.scen.json", &contract_map());
}

#[test]
fn send_dcdt_rs() {
	numbat_wasm_debug::denali_rs("denali/send_dcdt.scen.json", &contract_map());
}
