use super::VmApiImpl;
use numbat_wasm::{
    api::{CallValueApi, CallValueApiImpl, StaticVarApiImpl},
    types::{DcdtTokenType, ManagedType, TokenIdentifier},
};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);

    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedGetMultiDCDTCallValue(resultHandle: i32);

    fn getNumDCDTTransfers() -> i32;

    // single DCDT transfer
    fn bigIntGetDCDTCallValue(dest: i32);
    fn getDCDTTokenName(resultOffset: *const u8) -> i32;
    fn getDCDTTokenNonce() -> i64;
    fn getDCDTTokenType() -> i32;

    // DCDT by index
    fn bigIntGetDCDTCallValueByIndex(dest: i32, index: i32);
    fn getDCDTTokenNameByIndex(resultOffset: *const u8, index: i32) -> i32;
    fn getDCDTTokenNonceByIndex(index: i32) -> i64;
    fn getDCDTTokenTypeByIndex(index: i32) -> i32;
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

    fn load_rewa_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetCallValue(dest);
        }
    }

    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn load_all_dcdt_transfers(&self, dest_handle: Self::ManagedBufferHandle) {
        unsafe {
            managedGetMultiDCDTCallValue(dest_handle);
        }
    }

    fn dcdt_num_transfers(&self) -> usize {
        unsafe { getNumDCDTTransfers() as usize }
    }

    fn load_single_dcdt_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetDCDTCallValue(dest);
        }
    }

    fn token(&self) -> Option<Self::ManagedBufferHandle> {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getDCDTTokenName(name_buffer.as_mut_ptr());
            if name_len == 0 {
                None
            } else {
                Some(
                    TokenIdentifier::<Self>::from_dcdt_bytes(&name_buffer[..name_len as usize])
                        .get_raw_handle(),
                )
            }
        }
    }

    fn dcdt_token_nonce(&self) -> u64 {
        unsafe { getDCDTTokenNonce() as u64 }
    }

    fn dcdt_token_type(&self) -> DcdtTokenType {
        unsafe { (getDCDTTokenType() as u8).into() }
    }

    fn dcdt_value_by_index(&self, index: usize) -> Self::BigIntHandle {
        unsafe {
            let value_handle = self.next_handle();
            bigIntGetDCDTCallValueByIndex(value_handle, index as i32);
            value_handle
        }
    }

    fn token_by_index(&self, index: usize) -> Self::ManagedBufferHandle {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getDCDTTokenNameByIndex(name_buffer.as_mut_ptr(), index as i32);

            TokenIdentifier::<Self>::from_dcdt_bytes(&name_buffer[..name_len as usize])
                .get_raw_handle()
        }
    }

    fn dcdt_token_nonce_by_index(&self, index: usize) -> u64 {
        unsafe { getDCDTTokenNonceByIndex(index as i32) as u64 }
    }

    fn dcdt_token_type_by_index(&self, index: usize) -> DcdtTokenType {
        unsafe { (getDCDTTokenTypeByIndex(index as i32) as u8).into() }
    }
}
