use std::rc::Rc;

use denali::model::{TxCall, TxDCDT, TxExpect};

use crate::{
    tx_execution::sc_call_with_async_and_callback,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxInputDCDT},
    world_mock::BlockchainMock,
};

use super::check_tx_output;

pub fn execute(
    state: &mut Rc<BlockchainMock>,
    tx_id: &str,
    tx: &TxCall,
    expect: &Option<TxExpect>,
) {
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: tx.to.value.into(),
        rewa_value: tx.rewa_value.value.clone(),
        dcdt_values: tx_dcdt_transfers_from_denali(tx.dcdt_value.as_slice()),
        func_name: tx.function.as_bytes().to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash_dummy(tx_id),
    };
    let tx_result = sc_call_with_async_and_callback(tx_input, state, true);
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id, tx_expect, &tx_result);
    }
}

pub fn tx_dcdt_transfers_from_denali(denali_transf_dcdt: &[TxDCDT]) -> Vec<TxInputDCDT> {
    denali_transf_dcdt
        .iter()
        .map(tx_dcdt_transfer_from_denali)
        .collect()
}

pub fn tx_dcdt_transfer_from_denali(denali_transf_dcdt: &TxDCDT) -> TxInputDCDT {
    TxInputDCDT {
        token_identifier: denali_transf_dcdt.dcdt_token_identifier.value.clone(),
        nonce: denali_transf_dcdt.nonce.value,
        value: denali_transf_dcdt.dcdt_value.value.clone(),
    }
}
