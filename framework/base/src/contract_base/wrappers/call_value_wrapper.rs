use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, use_raw_handle, CallValueApi, CallValueApiImpl, ErrorApi, ErrorApiImpl,
        ManagedTypeApi, StaticVarApiImpl,
    },
    err_msg,
    types::{
        BigUint, RewaOrDcdtTokenIdentifier, RewaOrDcdtTokenPayment, DcdtTokenPayment, ManagedType,
        ManagedVec, TokenIdentifier,
    },
};

#[derive(Default)]
pub struct CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    _phantom: PhantomData<A>,
}

impl<A> CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub fn new() -> Self {
        CallValueWrapper {
            _phantom: PhantomData,
        }
    }

    /// Retrieves the REWA call value from the VM.
    /// Will return 0 in case of an DCDT transfer (cannot have both REWA and DCDT transfer simultaneously).
    pub fn rewa_value(&self) -> BigUint<A> {
        let mut call_value_handle = A::static_var_api_impl().get_call_value_rewa_handle();
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = use_raw_handle(const_handles::CALL_VALUE_REWA);
            A::static_var_api_impl().set_call_value_rewa_handle(call_value_handle.clone());
            A::call_value_api_impl().load_rewa_value(call_value_handle.clone());
        }
        BigUint::from_handle(call_value_handle) // unsafe, TODO: replace with ManagedRef<...>
    }

    /// Returns all DCDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just REWA.
    /// Fully managed underlying types, very efficient.
    pub fn all_dcdt_transfers(&self) -> ManagedVec<A, DcdtTokenPayment<A>> {
        let mut call_value_handle = A::static_var_api_impl().get_call_value_multi_dcdt_handle();
        if call_value_handle == const_handles::UNINITIALIZED_HANDLE {
            call_value_handle = use_raw_handle(const_handles::CALL_VALUE_MULTI_DCDT);
            A::static_var_api_impl().set_call_value_multi_dcdt_handle(call_value_handle.clone());
            A::call_value_api_impl().load_all_dcdt_transfers(call_value_handle.clone());
        }
        ManagedVec::from_handle(call_value_handle) // unsafe, TODO: replace with ManagedRef<...>
    }

    /// Verify and casts the received multi DCDT transfer in to an array.
    ///
    /// Can be used to extract all payments in one line like this:
    ///
    /// `let [payment_a, payment_b, payment_c] = self.call_value().multi_dcdt();`.
    pub fn multi_dcdt<const N: usize>(&self) -> [DcdtTokenPayment<A>; N] {
        self.all_dcdt_transfers()
            .to_array_of_refs::<N>()
            .unwrap_or_else(|| {
                A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_DCDT_TRANSFERS.as_bytes())
            })
    }

    /// Expects precisely one DCDT token transfer, fungible or not.
    ///
    /// Will return the received DCDT payment.
    ///
    /// The amount cannot be 0, since that would not qualify as an DCDT transfer.
    pub fn single_dcdt(&self) -> DcdtTokenPayment<A> {
        let [payments] = self.multi_dcdt();
        payments
    }

    /// Expects precisely one fungible DCDT token transfer.
    ///
    /// Returns the token ID and the amount for fungible DCDT transfers.
    ///
    /// The amount cannot be 0, since that would not qualify as an DCDT transfer.
    pub fn single_fungible_dcdt(&self) -> (TokenIdentifier<A>, BigUint<A>) {
        let payment = self.single_dcdt();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }
        (payment.token_identifier, payment.amount)
    }

    /// Retrieves the DCDT call value from the VM.
    /// Will return 0 in case of an REWA transfer (cannot have both REWA and DCDT transfer simultaneously).
    pub fn dcdt_value(&self) -> BigUint<A> {
        let call_value_single_dcdt: A::BigIntHandle =
            use_raw_handle(const_handles::CALL_VALUE_SINGLE_DCDT);
        A::call_value_api_impl().load_single_dcdt_value(call_value_single_dcdt.clone());
        BigUint::from_handle(call_value_single_dcdt)
    }

    /// Accepts and returns either an REWA payment, or a single DCDT token.
    ///
    /// Will halt execution if more than one DCDT transfer was received.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 REWA.
    pub fn rewa_or_single_dcdt(&self) -> RewaOrDcdtTokenPayment<A> {
        let dcdt_transfers = self.all_dcdt_transfers();
        match dcdt_transfers.len() {
            0 => RewaOrDcdtTokenPayment {
                token_identifier: RewaOrDcdtTokenIdentifier::rewa(),
                token_nonce: 0,
                amount: self.rewa_value(),
            },
            1 => dcdt_transfers.get(0).into(),
            _ => A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_DCDT_TRANSFERS.as_bytes()),
        }
    }

    /// Accepts and returns either an REWA payment, or a single fungible DCDT token.
    ///
    /// Will halt execution if more than one DCDT transfer was received, or if the received DCDT is non- or semi-fungible.
    ///
    /// Works similar to `rewa_or_single_dcdt`,
    /// but checks the nonce to be 0 and returns a tuple of just token identifier and amount, for convenience.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 REWA.
    pub fn rewa_or_single_fungible_dcdt(&self) -> (RewaOrDcdtTokenIdentifier<A>, BigUint<A>) {
        let payment = self.rewa_or_single_dcdt();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }

        (payment.token_identifier, payment.amount)
    }
}
