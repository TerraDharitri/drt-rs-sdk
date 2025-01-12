use super::{DcdtTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    types::BigUint,
};

use crate as dharitri_sc; // needed by the TypeAbi generated code
use crate::derive::type_abi;

/// Encodes any type of payment, which either:
/// - MOA (can be zero in case of no payment whatsoever);
/// - Multi-DCDT (one or more DCDT transfers).
#[type_abi]
#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub enum MoaOrMultiDcdtPayment<M: ManagedTypeApi> {
    Moa(BigUint<M>),
    MultiDcdt(ManagedVec<M, DcdtTokenPayment<M>>),
}

impl<M: ManagedTypeApi> MoaOrMultiDcdtPayment<M> {
    pub fn is_empty(&self) -> bool {
        match self {
            MoaOrMultiDcdtPayment::Moa(moa_value) => moa_value == &0u32,
            MoaOrMultiDcdtPayment::MultiDcdt(dcdt_payments) => dcdt_payments.is_empty(),
        }
    }
}

/// The version of `MoaOrMultiDcdtPayment` that contains referrences instead of owned fields.
pub enum MoaOrMultiDcdtPaymentRefs<'a, M: ManagedTypeApi> {
    Moa(&'a BigUint<M>),
    MultiDcdt(&'a ManagedVec<M, DcdtTokenPayment<M>>),
}

impl<M: ManagedTypeApi> MoaOrMultiDcdtPayment<M> {
    pub fn as_refs(&self) -> MoaOrMultiDcdtPaymentRefs<'_, M> {
        match self {
            MoaOrMultiDcdtPayment::Moa(moa_value) => {
                MoaOrMultiDcdtPaymentRefs::Moa(moa_value)
            },
            MoaOrMultiDcdtPayment::MultiDcdt(dcdt_payments) => {
                MoaOrMultiDcdtPaymentRefs::MultiDcdt(dcdt_payments)
            },
        }
    }
}

impl<M: ManagedTypeApi> MoaOrMultiDcdtPaymentRefs<'_, M> {
    pub fn to_owned_payment(&self) -> MoaOrMultiDcdtPayment<M> {
        match self {
            MoaOrMultiDcdtPaymentRefs::Moa(moa_value) => {
                MoaOrMultiDcdtPayment::Moa((*moa_value).clone())
            },
            MoaOrMultiDcdtPaymentRefs::MultiDcdt(dcdt_payments) => {
                MoaOrMultiDcdtPayment::MultiDcdt((*dcdt_payments).clone())
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            MoaOrMultiDcdtPaymentRefs::Moa(moa_value) => *moa_value == &0u32,
            MoaOrMultiDcdtPaymentRefs::MultiDcdt(dcdt_payments) => dcdt_payments.is_empty(),
        }
    }
}
