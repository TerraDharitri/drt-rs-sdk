use numbat_wasm::types::Address;

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult, TxTokenTransfer};

pub trait BuiltinFunction {
    fn name(&self) -> &str;

    fn extract_dcdt_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDcdtTransferInfo {
        BuiltinFunctionDcdtTransferInfo::empty(tx_input)
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate);
}

/// Contains a builtin function call DCDT transfers (if any) and the real recipient of the transfer
/// (can be different from the "to" field.)
pub struct BuiltinFunctionDcdtTransferInfo {
    pub real_recipient: Address,
    pub transfers: Vec<TxTokenTransfer>,
}

impl BuiltinFunctionDcdtTransferInfo {
    pub fn empty(tx_input: &TxInput) -> Self {
        BuiltinFunctionDcdtTransferInfo {
            real_recipient: tx_input.to.clone(),
            transfers: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.transfers.is_empty()
    }
}
