use crate::{
    api::ManagedTypeApi,
    types::{ManagedType, ManagedVec, MultiRewaOrDcdtPayment, Payment},
};

/// Alias for a list of payments.
pub type PaymentVec<Api> = ManagedVec<Api, Payment<Api>>;

impl<M: ManagedTypeApi> PaymentVec<M> {
    /// Converts to the legacy `MultiRewaOrDcdtPayment`.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn as_multi_rewa_or_dcdt_payment(&self) -> &MultiRewaOrDcdtPayment<M> {
        unsafe { core::mem::transmute(self) }
    }

    /// Converts to the legacy `MultiRewaOrDcdtPayment`.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn into_multi_rewa_or_dcdt_payment(self) -> MultiRewaOrDcdtPayment<M> {
        unsafe { MultiRewaOrDcdtPayment::from_handle(self.forget_into_handle()) }
    }
}

impl<M: ManagedTypeApi> From<()> for PaymentVec<M> {
    #[inline]
    fn from(_value: ()) -> Self {
        PaymentVec::new()
    }
}

impl<M: ManagedTypeApi> From<Payment<M>> for PaymentVec<M> {
    #[inline]
    fn from(value: Payment<M>) -> Self {
        PaymentVec::from_single_item(value)
    }
}
