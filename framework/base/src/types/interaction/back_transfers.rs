use crate::{
    api::ManagedTypeApi,
    types::{
        BigUint, RewaOrDcdtTokenPaymentMultiValue, DcdtTokenPayment, MultiRewaOrDcdtPayment,
        MultiDcdtPayment, MultiValueEncoded, PaymentVec,
    },
};

/// Holding back-transfer data, as retrieved from the VM.
#[deprecated(
    since = "0.59.0",
    note = "BackTransfers is now used instead, the legacy mechanism doesn't handle multi-transfers well"
)]
#[derive(Clone)]
pub struct BackTransfersLegacy<A>
where
    A: ManagedTypeApi,
{
    pub total_rewa_amount: BigUint<A>,
    pub dcdt_payments: MultiDcdtPayment<A>,
}

/// Holding back-transfer data, as retrieved from the VM.
///
/// It supports all transfer scenarios (REWA, DCDT, mixed).
#[derive(Clone)]
pub struct BackTransfers<A>
where
    A: ManagedTypeApi,
{
    pub payments: MultiRewaOrDcdtPayment<A>,
}

impl<A> From<MultiRewaOrDcdtPayment<A>> for BackTransfers<A>
where
    A: ManagedTypeApi,
{
    fn from(value: MultiRewaOrDcdtPayment<A>) -> Self {
        BackTransfers::new(value)
    }
}

impl<A> BackTransfers<A>
where
    A: ManagedTypeApi,
{
    pub fn new(payments: MultiRewaOrDcdtPayment<A>) -> Self {
        BackTransfers { payments }
    }

    /// The sum of all REWA-000000 back-transfers.
    pub fn rewa_sum(&self) -> BigUint<A> {
        self.payments.rewa_sum()
    }

    /// Requires that back-transfer is a single DCDT payment, and returns it, crashes otherwise.
    pub fn to_single_dcdt(self) -> DcdtTokenPayment<A> {
        self.payments.to_single_dcdt()
    }

    /// Converts back-transfer to a multi-value object, in this case a multi-value list of triples:
    /// `[(token identifier, payment, nonce)]`
    pub fn into_multi_value(self) -> MultiValueEncoded<A, RewaOrDcdtTokenPaymentMultiValue<A>> {
        self.payments.into_multi_value()
    }

    /// Converts data to the newer PaymentVec (ManagedVec<Payment>).
    pub fn into_payment_vec(self) -> PaymentVec<A> {
        self.payments.into_payment_vec()
    }
}
