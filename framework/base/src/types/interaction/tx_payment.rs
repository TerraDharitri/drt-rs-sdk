mod test_dcdt_transfer;
mod tx_payment_moa;
mod tx_payment_moa_or_dcdt;
mod tx_payment_moa_or_dcdt_refs;
mod tx_payment_moa_or_multi_dcdt;
mod tx_payment_moa_or_multi_dcdt_ref;
mod tx_payment_moa_value;
mod tx_payment_multi_moa_or_dcdt;
mod tx_payment_multi_dcdt;
mod tx_payment_none;
mod tx_payment_not_payable;
mod tx_payment_single_dcdt;
mod tx_payment_single_dcdt_ref;
mod tx_payment_single_dcdt_triple;

pub use test_dcdt_transfer::TestDcdtTransfer;
pub use tx_payment_moa::{Moa, MoaPayment};
pub use tx_payment_moa_value::TxMoaValue;
pub use tx_payment_multi_dcdt::TxPaymentMultiDcdt;
pub use tx_payment_not_payable::NotPayable;

use crate::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress, ManagedBuffer, MultiMoaOrDcdtPayment},
};

use super::{AnnotatedValue, FunctionCall, TxEnv, TxFrom, TxToSpecified};

/// Describes a payment that is part of a transaction.
#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used as payment (does not implement `TxPayment<{Env}>`)",
    label = "not a valid payment type",
    note = "there are multiple ways to specify the transaction payment, but `{Self}` is not one of them"
)]
pub trait TxPayment<Env>
where
    Env: TxEnv,
{
    /// Returns true if payment indicates transfer of either non-zero MOA or DCDT amounts.
    fn is_no_payment(&self, env: &Env) -> bool;

    /// Transfer-execute calls have different APIs for different payments types.
    /// This method selects between them.
    fn perform_transfer_execute(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    );

    /// Converts an DCDT call to a built-in function call, if necessary.
    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R;

    /// Payment data to be used by the testing framework. Will be refactored.
    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api>;
}

/// Marker trait that indicates that payment field contains no payment.
///
/// Implemented by `()` and `NotPayable`.
pub trait TxNoPayment<Env>: TxPayment<Env>
where
    Env: TxEnv,
{
}

/// Marks a payment object that only contains MOA or nothing at all.
pub trait TxPaymentMoaOnly<Env>: TxPayment<Env> + AnnotatedValue<Env, BigUint<Env::Api>>
where
    Env: TxEnv,
{
    #[inline]
    fn with_moa_value<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        self.with_value_ref(env, f)
    }

    fn into_moa_payment(self, env: &Env) -> BigUint<Env::Api> {
        self.into_value(env)
    }
}

#[derive(Clone)]
pub struct AnnotatedMoaPayment<Api>
where
    Api: ManagedTypeApi,
{
    pub value: BigUint<Api>,
    pub annotation: ManagedBuffer<Api>,
}

impl<Api> AnnotatedMoaPayment<Api>
where
    Api: ManagedTypeApi,
{
    pub fn new_moa(value: BigUint<Api>) -> Self {
        let annotation = value.to_display();
        AnnotatedMoaPayment { value, annotation }
    }
}

#[derive(Clone)]
pub struct FullPaymentData<Api>
where
    Api: ManagedTypeApi,
{
    pub moa: Option<AnnotatedMoaPayment<Api>>,
    pub multi_dcdt: MultiMoaOrDcdtPayment<Api>,
}

impl<Api> Default for FullPaymentData<Api>
where
    Api: ManagedTypeApi,
{
    fn default() -> Self {
        Self {
            moa: None,
            multi_dcdt: Default::default(),
        }
    }
}
