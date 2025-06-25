use denali::TxTransfer;

use crate::BlockchainMock;

pub fn execute(state: &mut BlockchainMock, tx: &TxTransfer) {
    let sender_address = &tx.from.value.into();
    state.increase_nonce(sender_address);
    state
        .subtract_tx_payment(sender_address, &tx.value.value)
        .unwrap();
    let recipient_address = &tx.to.value.into();
    state.increase_balance(recipient_address, &tx.value.value);
    let dcdt_token_identifier = tx.dcdt_token_identifier.value.clone();
    let dcdt_value = tx.dcdt_value.value.clone();

    if !dcdt_token_identifier.is_empty() && dcdt_value > 0u32.into() {
        state.substract_dcdt_balance(sender_address, &dcdt_token_identifier[..], &dcdt_value);
        state.increase_dcdt_balance(recipient_address, &dcdt_token_identifier[..], &dcdt_value);
    }
}
