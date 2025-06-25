use crate::*;

const DCDT_TRANSFER_FUNC: &[u8] = b"DCDTTransfer";
const SET_USERNAME_FUNC: &[u8] = b"SetUserName";

pub fn try_execute_builtin_function(
	tx_input: &TxInput,
	state: &mut BlockchainMock,
) -> Option<TxResult> {
	match tx_input.func_name.as_slice() {
		DCDT_TRANSFER_FUNC => Some(execute_dcdt_transfer(tx_input, state)),
		SET_USERNAME_FUNC => Some(execute_set_username(tx_input, state)),
		_ => None,
	}
}

fn execute_dcdt_transfer(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
	let from = tx_input.from.clone();
	let to = tx_input.to.clone();
	let dcdt_token_identifier = tx_input.dcdt_token_identifier.clone();
	let dcdt_value = tx_input.dcdt_value.clone();

	state.substract_dcdt_balance(&from, &dcdt_token_identifier, &dcdt_value);
	state.increase_dcdt_balance(&to, &dcdt_token_identifier, &dcdt_value);
	TxResult::empty()
}

fn execute_set_username(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
	assert_eq!(tx_input.args.len(), 1, "SetUserName expects 1 argument");
	if state.try_set_username(&tx_input.to, tx_input.args[0].as_slice()) {
		TxResult::empty()
	} else {
		TxResult {
			result_status: 10,
			result_message: b"username already set".to_vec(),
			result_values: Vec::new(),
			result_logs: Vec::new(),
		}
	}
}
