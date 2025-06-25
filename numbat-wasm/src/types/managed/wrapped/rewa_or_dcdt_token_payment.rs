use crate::{
    api::ManagedTypeApi,
    types::{BigUint, RewaOrDcdtTokenIdentifier},
};

use numbat_codec::{
    numbat_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    CodecFrom, CodecFromSelf,
};

use crate as numbat_wasm; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

use super::DcdtTokenPayment;

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct RewaOrDcdtTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: RewaOrDcdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> RewaOrDcdtTokenPayment<M> {
    pub fn no_payment() -> Self {
        RewaOrDcdtTokenPayment {
            token_identifier: RewaOrDcdtTokenIdentifier::rewa(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(
        token_identifier: RewaOrDcdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        RewaOrDcdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will convert to just DCDT or terminate execution if the token is REWA.
    pub fn unwrap_dcdt(self) -> DcdtTokenPayment<M> {
        DcdtTokenPayment::new(
            self.token_identifier.unwrap_dcdt(),
            self.token_nonce,
            self.amount,
        )
    }

    pub fn into_tuple(self) -> (RewaOrDcdtTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(RewaOrDcdtTokenIdentifier<M>, u64, BigUint<M>)>
    for RewaOrDcdtTokenPayment<M>
{
    #[inline]
    fn from(value: (RewaOrDcdtTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> From<DcdtTokenPayment<M>> for RewaOrDcdtTokenPayment<M> {
    fn from(dcdt_payment: DcdtTokenPayment<M>) -> Self {
        RewaOrDcdtTokenPayment {
            token_identifier: RewaOrDcdtTokenIdentifier::dcdt(dcdt_payment.token_identifier),
            token_nonce: dcdt_payment.token_nonce,
            amount: dcdt_payment.amount,
        }
    }
}

impl<M> CodecFromSelf for RewaOrDcdtTokenPayment<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for RewaOrDcdtTokenPayment<M> where M: ManagedTypeApi {}
