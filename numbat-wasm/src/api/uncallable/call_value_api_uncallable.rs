use crate::{
    api::{CallValueApi, CallValueApiImpl, Handle},
    types::DcdtTokenType,
};

use super::UncallableApi;

impl CallValueApi for UncallableApi {
    type CallValueApiImpl = UncallableApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        unreachable!()
    }
}

impl CallValueApiImpl for UncallableApi {
    fn check_not_payable(&self) {
        unreachable!()
    }

    fn rewa_value(&self) -> Handle {
        unreachable!()
    }

    fn dcdt_value(&self) -> Handle {
        unreachable!()
    }

    fn token(&self) -> Handle {
        unreachable!()
    }

    fn dcdt_token_nonce(&self) -> u64 {
        unreachable!()
    }

    fn dcdt_token_type(&self) -> DcdtTokenType {
        unreachable!()
    }

    fn dcdt_num_transfers(&self) -> usize {
        unreachable!()
    }

    fn dcdt_value_by_index(&self, _index: usize) -> Handle {
        unreachable!()
    }

    fn token_by_index(&self, _index: usize) -> Handle {
        unreachable!()
    }

    fn dcdt_token_nonce_by_index(&self, _index: usize) -> u64 {
        unreachable!()
    }

    fn dcdt_token_type_by_index(&self, _index: usize) -> DcdtTokenType {
        unreachable!()
    }
}
