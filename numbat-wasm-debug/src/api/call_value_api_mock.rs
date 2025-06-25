use crate::{TxContext, TxPanic};
use numbat_wasm::{
    api::CallValueApi,
    err_msg,
    types::{BigUint, DcdtTokenType, ManagedBuffer, TokenIdentifier},
};

impl CallValueApi for TxContext {
    fn check_not_payable(&self) {
        if self.rewa_value() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_REWA.to_vec(),
            });
        }
        if self.dcdt_value() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_DCDT.to_vec(),
            });
        }
    }

    #[inline]
    fn rewa_value(&self) -> BigUint<Self> {
        self.insert_new_big_uint(self.tx_input_box.call_value.clone())
    }

    #[inline]
    fn dcdt_value(&self) -> BigUint<Self> {
        self.insert_new_big_uint(self.tx_input_box.dcdt_value.clone())
    }

    #[inline]
    fn token(&self) -> TokenIdentifier<Self> {
        ManagedBuffer::new_from_bytes(
            self.clone(),
            self.tx_input_box.dcdt_token_identifier.as_slice(),
        )
        .into()
    }

    #[inline]
    fn dcdt_token_nonce(&self) -> u64 {
        // TODO: Add DCDT nonce in mock
        0u64
    }

    #[inline]
    fn dcdt_token_type(&self) -> DcdtTokenType {
        // TODO: Add DCDT token type in mock
        DcdtTokenType::Fungible
    }

    // TODO: Mock multi-transfers

    #[inline]
    fn dcdt_num_transfers(&self) -> usize {
        0
    }

    #[inline]
    fn dcdt_value_by_index(&self, _index: usize) -> BigUint<Self> {
        self.insert_new_big_uint_zero()
    }

    #[inline]
    fn token_by_index(&self, _index: usize) -> TokenIdentifier<Self> {
        TokenIdentifier::rewa(self.clone())
    }

    #[inline]
    fn dcdt_token_nonce_by_index(&self, _index: usize) -> u64 {
        0
    }

    #[inline]
    fn dcdt_token_type_by_index(&self, _index: usize) -> DcdtTokenType {
        DcdtTokenType::Fungible
    }
}
