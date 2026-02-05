use generic_array::typenum::U16;
use dharitri_sc_codec::IntoMultiValue;

use crate::{
    api::ManagedTypeApi,
    types::{
        BigUint, Rewa, DcdtTokenPayment, DcdtTokenPaymentRefs, ManagedVecItem,
        ManagedVecItemPayloadBuffer, NonZeroBigUint, PaymentMultiValue, PaymentRefs, Ref, TokenId,
        managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index,
    },
};

use crate as dharitri_sc; // needed by the codec and TypeAbi generated code
use crate::{
    codec::{
        self, NestedDecode, TopDecode,
        derive::{NestedEncode, TopEncode},
    },
    derive::type_abi,
};

use super::{RewaOrDcdtTokenIdentifier, RewaOrDcdtTokenPayment};

#[type_abi]
#[derive(TopEncode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub struct Payment<M: ManagedTypeApi> {
    pub token_identifier: TokenId<M>,
    pub token_nonce: u64,
    pub amount: NonZeroBigUint<M>,
}

impl<M: ManagedTypeApi> Payment<M> {
    #[inline]
    pub fn new(token_identifier: TokenId<M>, token_nonce: u64, amount: NonZeroBigUint<M>) -> Self {
        Payment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn is_fungible(&self) -> bool {
        self.token_nonce == 0
    }

    #[inline]
    pub fn into_tuple(self) -> (TokenId<M>, u64, NonZeroBigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }

    #[inline]
    pub fn as_tuple(&self) -> (&TokenId<M>, u64, &NonZeroBigUint<M>) {
        (&self.token_identifier, self.token_nonce, &self.amount)
    }

    /// Zero-cost conversion that loosens the REWA restriction.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn as_rewa_or_dcdt_payment(&self) -> &RewaOrDcdtTokenPayment<M> {
        unsafe { core::mem::transmute(self) }
    }

    /// Conversion that loosens the REWA restriction.
    pub fn into_rewa_or_dcdt_payment(self) -> RewaOrDcdtTokenPayment<M> {
        RewaOrDcdtTokenPayment {
            token_identifier: RewaOrDcdtTokenIdentifier::dcdt(self.token_identifier),
            token_nonce: self.token_nonce,
            amount: self.amount.into_big_uint(),
        }
    }

    /// Same as `map_rewa_or_dcdt`, but only takes a reference,
    /// and consequently, the closures also only get references.
    pub fn map_ref_rewa_or_dcdt<Context, D, F, U>(
        &self,
        context: Context,
        for_rewa: D,
        for_dcdt: F,
    ) -> U
    where
        D: FnOnce(Context, Rewa<&BigUint<M>>) -> U,
        F: FnOnce(Context, DcdtTokenPaymentRefs<'_, M>) -> U,
    {
        self.as_refs().map_rewa_or_dcdt(context, for_rewa, for_dcdt)
    }

    pub fn map_rewa_or_dcdt<Context, D, F, U>(self, context: Context, for_rewa: D, for_dcdt: F) -> U
    where
        D: FnOnce(Context, Rewa<BigUint<M>>) -> U,
        F: FnOnce(Context, DcdtTokenPayment<M>) -> U,
    {
        if self.token_identifier.is_native() {
            for_rewa(context, Rewa(self.amount.into_big_uint()))
        } else {
            for_dcdt(
                context,
                DcdtTokenPayment::new(
                    unsafe { self.token_identifier.into_dcdt_unchecked() },
                    self.token_nonce,
                    self.amount.into_big_uint(),
                ),
            )
        }
    }

    pub fn as_refs(&self) -> PaymentRefs<'_, M> {
        PaymentRefs::new(&self.token_identifier, self.token_nonce, &self.amount)
    }
}

impl<M> AsRef<Payment<M>> for &Payment<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn as_ref(&self) -> &Payment<M> {
        self
    }
}

impl<M: ManagedTypeApi> From<(TokenId<M>, u64, NonZeroBigUint<M>)> for Payment<M> {
    #[inline]
    fn from(value: (TokenId<M>, u64, NonZeroBigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> TopDecode for Payment<M> {
    fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::TopDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let mut nested_buffer = top_input.into_nested_buffer();
        let result = Self::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        if !codec::NestedDecodeInput::is_depleted(&nested_buffer) {
            return Err(h.handle_error(codec::DecodeError::INPUT_TOO_LONG));
        }
        Ok(result)
    }
}

impl<M: ManagedTypeApi> NestedDecode for Payment<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Ok(Payment {
            token_identifier: TokenId::<M>::dep_decode_or_handle_err(input, h)?,
            token_nonce: <u64>::dep_decode_or_handle_err(input, h)?,
            amount: NonZeroBigUint::<M>::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl<M: ManagedTypeApi> IntoMultiValue for Payment<M> {
    type MultiValue = PaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for Payment<M> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<U16>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = Ref<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            Payment {
                token_identifier: managed_vec_item_read_from_payload_index(payload, &mut index),
                token_nonce: managed_vec_item_read_from_payload_index(payload, &mut index),
                amount: managed_vec_item_read_from_payload_index(payload, &mut index),
            }
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        unsafe { Ref::new(Self::read_from_payload(payload)) }
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        let mut index = 0;

        unsafe {
            managed_vec_item_save_to_payload_index(self.token_identifier, payload, &mut index);
            managed_vec_item_save_to_payload_index(self.token_nonce, payload, &mut index);
            managed_vec_item_save_to_payload_index(self.amount, payload, &mut index);
        }
    }
}
