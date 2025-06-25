use super::{DcdtTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        CodecFromSelf,
    },
    types::BigUint,
};

use crate as dharitri_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

/// Encodes any type of payment, which either:
/// - REWA (can be zero in case of no payment whatsoever);
/// - Multi-DCDT (one or more DCDT transfers).
#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub enum RewaOrMultiDcdtPayment<M: ManagedTypeApi> {
    Rewa(BigUint<M>),
    MultiDcdt(ManagedVec<M, DcdtTokenPayment<M>>),
}

impl<M> CodecFromSelf for RewaOrMultiDcdtPayment<M> where M: ManagedTypeApi {}
