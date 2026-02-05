use crate::{
    api::{ManagedTypeApi, quick_signal_error},
    err_msg,
    types::{
        BigUint, RewaOrDcdtTokenPayment, RewaOrDcdtTokenPaymentMultiValue, DcdtTokenPayment,
        ManagedType, ManagedVec, MultiValueEncoded, PaymentVec,
    },
};

/// Alias for a list of payments of REWA or DCDT tokens.
pub type MultiRewaOrDcdtPayment<Api> = ManagedVec<Api, RewaOrDcdtTokenPayment<Api>>;

impl<M> MultiRewaOrDcdtPayment<M>
where
    M: ManagedTypeApi,
{
    /// The sum of all REWA-000000 transfers.
    pub fn rewa_sum(&self) -> BigUint<M> {
        let mut sum = BigUint::zero();
        for payment in self {
            if payment.token_identifier.is_rewa() {
                sum += &payment.amount;
            }
        }
        sum
    }

    /// Requires that this is a single DCDT payment, and returns it, crashes otherwise.
    pub fn to_single_dcdt(self) -> DcdtTokenPayment<M> {
        if self.len() != 1 {
            quick_signal_error::<M>(err_msg::SINGLE_DCDT_EXPECTED)
        }

        let payment = self.get(0).clone();
        payment.unwrap_dcdt()
    }

    /// Converts to a multi-value object, in this case a multi-value list of triples:
    /// `[(token identifier, payment, nonce)]`
    pub fn into_multi_value(self) -> MultiValueEncoded<M, RewaOrDcdtTokenPaymentMultiValue<M>> {
        let mut encoded = MultiValueEncoded::new();

        for payment in self {
            encoded.push(RewaOrDcdtTokenPaymentMultiValue::from(payment));
        }

        encoded
    }

    /// Converts to the newer PaymentVec (ManagedVec<Payment>).
    pub fn into_payment_vec(self) -> PaymentVec<M> {
        // safe, because it is the same layout
        unsafe { PaymentVec::from_handle(self.forget_into_handle()) }
    }
}
