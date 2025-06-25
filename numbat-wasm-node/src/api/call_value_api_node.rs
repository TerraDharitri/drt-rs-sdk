use super::VmApiImpl;
use numbat_wasm::{
    api::{CallValueApi, CallValueApiImpl, Handle},
    types::{DcdtTokenType, ManagedType, TokenIdentifier},
};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
    fn bigIntNew(value: i64) -> i32;
    #[cfg(not(feature = "unmanaged-ei"))]
    fn mBufferNew() -> i32;

    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);
    fn bigIntGetDCDTCallValue(dest: i32);
    fn getDCDTTokenName(resultOffset: *const u8) -> i32;
    fn getDCDTTokenNonce() -> i64;
    fn getDCDTTokenType() -> i32;

    // multi-transfer API
    fn getNumDCDTTransfers() -> i32;
    fn bigIntGetDCDTCallValueByIndex(dest: i32, index: i32);
    fn getDCDTTokenNameByIndex(resultOffset: *const u8, index: i32) -> i32;
    fn getDCDTTokenNonceByIndex(index: i32) -> i64;
    fn getDCDTTokenTypeByIndex(index: i32) -> i32;
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedGetMultiDCDTCallValue(resultHandle: i32);

    /// TODO: decide if it is worth using or not
    #[allow(dead_code)]
    fn getCallValueTokenName(callValueOffset: *const u8, resultOffset: *const u8) -> i32;
}

impl CallValueApi for VmApiImpl {
    type CallValueApiImpl = VmApiImpl;

    #[inline]
    fn call_value_api_impl() -> Self::CallValueApiImpl {
        VmApiImpl {}
    }
}

impl CallValueApiImpl for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn rewa_value(&self) -> Handle {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetCallValue(value_handle);
            value_handle
        }
    }

    fn dcdt_value(&self) -> Handle {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetDCDTCallValue(value_handle);
            value_handle
        }
    }

    fn token(&self) -> Handle {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getDCDTTokenName(name_buffer.as_mut_ptr());
            if name_len == 0 {
                TokenIdentifier::<Self>::rewa().get_raw_handle()
            } else {
                TokenIdentifier::<Self>::from_dcdt_bytes(&name_buffer[..name_len as usize])
                    .get_raw_handle()
            }
        }
    }

    fn dcdt_token_nonce(&self) -> u64 {
        unsafe { getDCDTTokenNonce() as u64 }
    }

    fn dcdt_token_type(&self) -> DcdtTokenType {
        unsafe { (getDCDTTokenType() as u8).into() }
    }

    fn dcdt_num_transfers(&self) -> usize {
        unsafe { getNumDCDTTransfers() as usize }
    }

    fn dcdt_value_by_index(&self, index: usize) -> Handle {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetDCDTCallValueByIndex(value_handle, index as i32);
            value_handle
        }
    }

    fn token_by_index(&self, index: usize) -> Handle {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getDCDTTokenNameByIndex(name_buffer.as_mut_ptr(), index as i32);
            if name_len == 0 {
                TokenIdentifier::<Self>::rewa().get_raw_handle()
            } else {
                TokenIdentifier::<Self>::from_dcdt_bytes(&name_buffer[..name_len as usize])
                    .get_raw_handle()
            }
        }
    }

    fn dcdt_token_nonce_by_index(&self, index: usize) -> u64 {
        unsafe { getDCDTTokenNonceByIndex(index as i32) as u64 }
    }

    fn dcdt_token_type_by_index(&self, index: usize) -> DcdtTokenType {
        unsafe { (getDCDTTokenTypeByIndex(index as i32) as u8).into() }
    }

    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_all_dcdt_transfers<M: numbat_wasm::api::ManagedTypeApi>(
        &self,
    ) -> numbat_wasm::types::ManagedVec<M, numbat_wasm::types::DcdtTokenPayment<M>> {
        unsafe {
            let result_handle = mBufferNew();
            managedGetMultiDCDTCallValue(result_handle);
            numbat_wasm::types::ManagedVec::from_raw_handle(result_handle)
        }
    }
}
