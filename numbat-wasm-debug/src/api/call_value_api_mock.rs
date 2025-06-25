use crate::{num_bigint, tx_mock::TxPanic, DebugApi};
use numbat_wasm::{
    api::{CallValueApi, CallValueApiImpl},
    err_msg,
    types::DcdtTokenType,
};
use num_traits::Zero;

impl DebugApi {
    fn fail_if_more_than_one_dcdt_transfer(&self) {
        if self.dcdt_num_transfers() > 1 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::TOO_MANY_DCDT_TRANSFERS.to_string(),
            });
        }
    }
}

impl CallValueApi for DebugApi {
    type CallValueApiImpl = DebugApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        DebugApi::new_from_static()
    }
}

impl CallValueApiImpl for DebugApi {
    fn check_not_payable(&self) {
        if self.input_ref().rewa_value > num_bigint::BigUint::zero() {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_REWA.to_string(),
            });
        }
        if self.dcdt_num_transfers() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_DCDT.to_string(),
            });
        }
    }

    #[inline]
    fn load_rewa_value(&self, dest: Self::BigIntHandle) {
        self.set_big_uint(dest, self.input_ref().received_rewa().clone())
    }

    #[inline]
    fn load_single_dcdt_value(&self, dest: Self::BigIntHandle) {
        self.fail_if_more_than_one_dcdt_transfer();
        if let Some(dcdt_value) = self.input_ref().received_dcdt().get(0) {
            self.set_big_uint(dest, dcdt_value.value.clone());
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DCDT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn token(&self) -> Option<Self::ManagedBufferHandle> {
        self.fail_if_more_than_one_dcdt_transfer();

        if self.dcdt_num_transfers() > 0 {
            Some(self.token_by_index(0))
        } else {
            None
        }
    }

    #[inline]
    fn dcdt_token_nonce(&self) -> u64 {
        self.fail_if_more_than_one_dcdt_transfer();
        self.dcdt_token_nonce_by_index(0)
    }

    #[inline]
    fn dcdt_token_type(&self) -> DcdtTokenType {
        self.fail_if_more_than_one_dcdt_transfer();
        self.dcdt_token_type_by_index(0)
    }

    #[inline]
    fn dcdt_num_transfers(&self) -> usize {
        self.input_ref().received_dcdt().len()
    }

    #[inline]
    fn dcdt_value_by_index(&self, index: usize) -> Self::BigIntHandle {
        if let Some(dcdt_value) = self.input_ref().received_dcdt().get(index) {
            self.insert_new_big_uint(dcdt_value.value.clone())
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DCDT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn token_by_index(&self, index: usize) -> Self::ManagedBufferHandle {
        if let Some(dcdt_value) = self.input_ref().received_dcdt().get(index) {
            self.insert_new_managed_buffer(dcdt_value.token_identifier.clone())
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DCDT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn dcdt_token_nonce_by_index(&self, index: usize) -> u64 {
        if let Some(dcdt_value) = self.input_ref().received_dcdt().get(index) {
            dcdt_value.nonce
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::DCDT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn dcdt_token_type_by_index(&self, index: usize) -> DcdtTokenType {
        if self.dcdt_token_nonce_by_index(index) == 0 {
            DcdtTokenType::Fungible
        } else {
            DcdtTokenType::NonFungible
        }
    }
}
