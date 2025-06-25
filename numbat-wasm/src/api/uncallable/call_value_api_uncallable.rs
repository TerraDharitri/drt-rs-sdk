use crate::{
    api::CallValueApi,
    types::{BigUint, DcdtTokenType, TokenIdentifier},
};

use super::UncallableApi;

impl CallValueApi for UncallableApi {
    fn check_not_payable(&self) {
        unreachable!()
    }

    fn rewa_value(&self) -> BigUint<Self> {
        unreachable!()
    }

    fn dcdt_value(&self) -> BigUint<Self> {
        unreachable!()
    }

    fn token(&self) -> TokenIdentifier<Self> {
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

    fn dcdt_value_by_index(&self, _index: usize) -> BigUint<Self> {
        unreachable!()
    }

    fn token_by_index(&self, _index: usize) -> TokenIdentifier<Self> {
        unreachable!()
    }

    fn dcdt_token_nonce_by_index(&self, _index: usize) -> u64 {
        unreachable!()
    }

    fn dcdt_token_type_by_index(&self, _index: usize) -> DcdtTokenType {
        unreachable!()
    }
}
