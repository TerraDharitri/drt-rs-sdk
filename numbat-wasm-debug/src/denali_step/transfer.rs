use std::rc::Rc;

use numbat_wasm::types::H256;
use denali::model::TxTransfer;

use crate::{
    sc_call::tx_dcdt_transfers_from_denali, tx_execution::sc_call, tx_mock::TxInput,
    world_mock::BlockchainMock,
};

pub fn execute(state: &mut Rc<BlockchainMock>, tx_transfer: &TxTransfer) {
    let tx_input = TxInput {
        from: tx_transfer.from.value.into(),
        to: tx_transfer.to.value.into(),
        rewa_value: tx_transfer.rewa_value.value.clone(),
        dcdt_values: tx_dcdt_transfers_from_denali(tx_transfer.dcdt_value.as_slice()),
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: tx_transfer.gas_limit.value,
        gas_price: tx_transfer.gas_price.value,
        tx_hash: H256::zero(),
    };
    sc_call(tx_input, state, true).assert_ok();
}
