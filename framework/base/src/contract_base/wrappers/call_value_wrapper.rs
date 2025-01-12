use core::marker::PhantomData;

use dharitri_core::MOA_000000_TOKEN_IDENTIFIER;

use crate::{
    api::{
        const_handles, use_raw_handle, CallValueApi, CallValueApiImpl, ErrorApi, ErrorApiImpl,
        ManagedBufferApiImpl, ManagedTypeApi, RawHandle, StaticVarApiFlags, StaticVarApiImpl,
    },
    err_msg,
    types::{
        big_num_cmp::bi_gt_zero, BigUint, ConstDecimals, MoaOrDcdtTokenIdentifier,
        MoaOrDcdtTokenPayment, MoaOrMultiDcdtPayment, DcdtTokenPayment, ManagedDecimal,
        ManagedRef, ManagedType, ManagedVec, ManagedVecItem, ManagedVecItemPayload,
        ManagedVecPayloadIterator, ManagedVecRef, TokenIdentifier,
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

    /// Cached transfers from the VM.
    fn all_dcdt_transfers_unchecked(&self) -> A::ManagedBufferHandle {
        let all_transfers_unchecked_handle: A::ManagedBufferHandle =
            use_raw_handle(const_handles::CALL_VALUE_MULTI_DCDT);
        if !A::static_var_api_impl()
            .flag_is_set_or_update(StaticVarApiFlags::CALL_VALUE_DCDT_UNCHECKED_INITIALIZED)
        {
            A::call_value_api_impl()
                .load_all_dcdt_transfers(all_transfers_unchecked_handle.clone());
        }
        all_transfers_unchecked_handle
    }

    /// Retrieves the MOA call value from the VM.
    ///
    /// Will return 0 in case of an DCDT transfer, even though MOA and DCDT transfers are now posible.
    pub fn moa_direct_non_strict(&self) -> ManagedRef<'static, A, BigUint<A>> {
        let call_value_handle: A::BigIntHandle = use_raw_handle(const_handles::CALL_VALUE_MOA);
        if !A::static_var_api_impl()
            .flag_is_set_or_update(StaticVarApiFlags::CALL_VALUE_MOA_DIRECT_INITIALIZED)
        {
            A::call_value_api_impl().load_moa_value(call_value_handle.clone());
        }
        unsafe { ManagedRef::wrap_handle(call_value_handle) }
    }

    /// Retrieves the MOA call value and crashes if anything else was transferred.
    ///
    /// Accepts both MOA sent directly, as well as MOA sent alone in a multi-transfer.
    ///
    /// Does not accept a multi-transfer with 2 or more transfers, not even 2 or more MOA transfers.
    pub fn moa(&self) -> ManagedRef<'static, A, BigUint<A>> {
        let dcdt_transfers_handle = self.all_dcdt_transfers_unchecked();
        let dcdt_transfers: ManagedRef<'static, A, ManagedVec<A, MoaOrDcdtTokenPayment<A>>> =
            unsafe { ManagedRef::wrap_handle(dcdt_transfers_handle) };
        match dcdt_transfers.len() {
            0 => self.moa_direct_non_strict(),
            1 => {
                let first = dcdt_transfers.get(0);
                if !first.token_identifier.is_moa() {
                    A::error_api_impl().signal_error(err_msg::NON_PAYABLE_FUNC_DCDT.as_bytes());
                }
                unsafe { ManagedRef::wrap_handle(first.amount.get_handle()) }
            },
            _ => A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_TRANSFERS.as_bytes()),
        }
    }

    /// Retrieves the MOA call value from the VM.
    ///
    /// Will return 0 in case of an DCDT transfer, even though MOA and DCDT transfers are now posible.
    ///
    /// ## Important!
    ///
    /// Does not cover multi-transfer scenarios properly, but left for backwards compatibility.
    ///
    /// Please use `.moa()` instead!
    ///
    /// For raw handling, `.moa_direct_non_strict()` is also acceptable.
    #[deprecated(
        since = "0.55.0",
        note = "Does not cover multi-transfer scenarios properly, but left for backwards compatibility. Please use .moa() instead!"
    )]
    pub fn moa_value(&self) -> ManagedRef<'static, A, BigUint<A>> {
        self.moa_direct_non_strict()
    }

    /// Returns the MOA call value from the VM as ManagedDecimal
    pub fn moa_decimal(&self) -> ManagedDecimal<A, ConstDecimals<18>> {
        ManagedDecimal::<A, ConstDecimals<18>>::const_decimals_from_raw(self.moa_value().clone())
    }

    /// Returns all DCDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just MOA.
    ///
    /// Will crash for MOA + DCDT multi transfers.
    pub fn all_dcdt_transfers(&self) -> ManagedRef<'static, A, ManagedVec<A, DcdtTokenPayment<A>>> {
        let multi_dcdt_handle: A::ManagedBufferHandle = self.all_dcdt_transfers_unchecked();
        let checked = A::static_var_api_impl()
            .flag_is_set_or_update(StaticVarApiFlags::CALL_VALUE_DCDT_INITIALIZED);
        if !checked && moa_000000_transfer_exists::<A>(multi_dcdt_handle.clone()) {
            A::error_api_impl().signal_error(err_msg::DCDT_UNEXPECTED_MOA.as_bytes())
        }

        unsafe { ManagedRef::wrap_handle(multi_dcdt_handle) }
    }

    /// Will return all transfers in the form of a list of MoaOrDcdtTokenPayment.
    ///
    /// Both MOA and DCDT can be returned.
    ///
    /// In case of a single MOA transfer, only one item will be returned,
    /// the MOA payment represented as an DCDT transfer (MOA-000000).
    pub fn all_transfers(
        &self,
    ) -> ManagedRef<'static, A, ManagedVec<A, MoaOrDcdtTokenPayment<A>>> {
        let all_transfers_handle: A::ManagedBufferHandle =
            use_raw_handle(const_handles::CALL_VALUE_ALL);
        if !A::static_var_api_impl()
            .flag_is_set_or_update(StaticVarApiFlags::CALL_VALUE_ALL_INITIALIZED)
        {
            let moa_single = self.moa_direct_non_strict();
            if bi_gt_zero::<A>(moa_single.get_handle()) {
                A::managed_type_impl().mb_overwrite(
                    use_raw_handle(const_handles::MBUF_MOA_000000),
                    MOA_000000_TOKEN_IDENTIFIER.as_bytes(),
                );
                A::managed_type_impl().mb_overwrite(
                    all_transfers_handle.clone(),
                    &const_handles::MOA_PAYMENT_PAYLOAD[..],
                );
            } else {
                // clone all_dcdt_transfers_unchecked -> all_transfers
                let all_transfers_unchecked_handle = self.all_dcdt_transfers_unchecked();
                A::managed_type_impl().mb_overwrite(all_transfers_handle.clone(), &[]);
                A::managed_type_impl()
                    .mb_append(all_transfers_handle.clone(), all_transfers_unchecked_handle);
            }
        }
        unsafe { ManagedRef::wrap_handle(all_transfers_handle) }
    }

    /// Verify and casts the received multi DCDT transfer in to an array.
    ///
    /// Can be used to extract all payments in one line like this:
    ///
    /// `let [payment_a, payment_b, payment_c] = self.call_value().multi_dcdt();`.
    ///
    /// Rejects MOA transfers. Switch to `multi_moa_or_dcdt` to accept mixed transfers.
    pub fn multi_dcdt<const N: usize>(&self) -> [ManagedVecRef<'static, DcdtTokenPayment<A>>; N] {
        let dcdt_transfers = self.all_dcdt_transfers();
        let array = dcdt_transfers.to_array_of_refs::<N>().unwrap_or_else(|| {
            A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_DCDT_TRANSFERS.as_bytes())
        });
        unsafe { core::mem::transmute(array) }
    }

    /// Verify and casts the received multi DCDT transfer in to an array.
    ///
    /// Can be used to extract all payments in one line like this:
    ///
    /// `let [payment_a, payment_b, payment_c] = self.call_value().multi_moa_or_dcdt();`.
    pub fn multi_moa_or_dcdt<const N: usize>(
        &self,
    ) -> [ManagedVecRef<'static, MoaOrDcdtTokenPayment<A>>; N] {
        let dcdt_transfers = self.all_transfers();
        let array = dcdt_transfers.to_array_of_refs::<N>().unwrap_or_else(|| {
            A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_TRANSFERS.as_bytes())
        });
        unsafe { core::mem::transmute(array) }
    }

    /// Expects precisely one DCDT token transfer, fungible or not.
    ///
    /// Will return the received DCDT payment.
    ///
    /// The amount cannot be 0, since that would not qualify as an DCDT transfer.
    pub fn single_dcdt(&self) -> ManagedVecRef<'static, DcdtTokenPayment<A>> {
        let dcdt_transfers = self.all_dcdt_transfers();
        if dcdt_transfers.len() != 1 {
            A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_DCDT_TRANSFERS.as_bytes())
        }
        let value = dcdt_transfers.get(0);
        unsafe { core::mem::transmute(value) }
    }

    /// Expects precisely one fungible DCDT token transfer.
    ///
    /// Returns the token ID and the amount for fungible DCDT transfers.
    ///
    /// The amount cannot be 0, since that would not qualify as an DCDT transfer.
    pub fn single_fungible_dcdt(
        &self,
    ) -> (
        ManagedRef<'static, A, TokenIdentifier<A>>,
        ManagedRef<'static, A, BigUint<A>>,
    ) {
        let payment = self.single_dcdt();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }

        unsafe {
            (
                ManagedRef::wrap_handle(payment.token_identifier.get_handle()),
                ManagedRef::wrap_handle(payment.amount.get_handle()),
            )
        }
    }

    /// Accepts and returns either an MOA payment, or a single DCDT token.
    ///
    /// Will halt execution if more than one DCDT transfer was received.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 MOA.
    pub fn moa_or_single_dcdt(&self) -> MoaOrDcdtTokenPayment<A> {
        let dcdt_transfers_handle = self.all_dcdt_transfers_unchecked();
        let dcdt_transfers: ManagedRef<'static, A, ManagedVec<A, MoaOrDcdtTokenPayment<A>>> =
            unsafe { ManagedRef::wrap_handle(dcdt_transfers_handle) };
        match dcdt_transfers.len() {
            0 => MoaOrDcdtTokenPayment {
                token_identifier: MoaOrDcdtTokenIdentifier::moa(),
                token_nonce: 0,
                amount: self.moa_direct_non_strict().clone(),
            },
            1 => dcdt_transfers.get(0).clone(),
            _ => A::error_api_impl().signal_error(err_msg::INCORRECT_NUM_DCDT_TRANSFERS.as_bytes()),
        }
    }

    /// Accepts and returns either an MOA payment, or a single fungible DCDT token.
    ///
    /// Will halt execution if more than one DCDT transfer was received, or if the received DCDT is non- or semi-fungible.
    ///
    /// Works similar to `moa_or_single_dcdt`,
    /// but checks the nonce to be 0 and returns a tuple of just token identifier and amount, for convenience.
    ///
    /// In case no transfer of value happen, it will return a payment of 0 MOA.
    pub fn moa_or_single_fungible_dcdt(&self) -> (MoaOrDcdtTokenIdentifier<A>, BigUint<A>) {
        let payment = self.moa_or_single_dcdt();
        if payment.token_nonce != 0 {
            A::error_api_impl().signal_error(err_msg::FUNGIBLE_TOKEN_EXPECTED_ERR_MSG.as_bytes());
        }

        (payment.token_identifier, payment.amount)
    }

    /// Accepts any sort of patyment, which is either:
    /// - MOA (can be zero in case of no payment whatsoever);
    /// - Multi-DCDT (one or more DCDT transfers).
    pub fn any_payment(&self) -> MoaOrMultiDcdtPayment<A> {
        let dcdt_transfers = self.all_dcdt_transfers();
        if dcdt_transfers.is_empty() {
            MoaOrMultiDcdtPayment::Moa(self.moa_direct_non_strict().clone())
        } else {
            MoaOrMultiDcdtPayment::MultiDcdt(dcdt_transfers.clone())
        }
    }
}

fn moa_000000_transfer_exists<A>(transfers_vec_handle: A::ManagedBufferHandle) -> bool
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    A::managed_type_impl().mb_overwrite(
        use_raw_handle(const_handles::MBUF_MOA_000000),
        MOA_000000_TOKEN_IDENTIFIER.as_bytes(),
    );
    unsafe {
        let mut iter: ManagedVecPayloadIterator<
            A,
            <DcdtTokenPayment<A> as ManagedVecItem>::PAYLOAD,
        > = ManagedVecPayloadIterator::new(transfers_vec_handle);

        iter.any(|payload| {
            let token_identifier_handle = RawHandle::read_from_payload(payload.slice_unchecked(0));
            A::managed_type_impl().mb_eq(
                use_raw_handle(const_handles::MBUF_MOA_000000),
                use_raw_handle(token_identifier_handle),
            )
        })
    }
}
