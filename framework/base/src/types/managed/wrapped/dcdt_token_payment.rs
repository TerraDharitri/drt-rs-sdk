use crate::{
    api::ManagedTypeApi,
    types::{
        BigUint, DcdtTokenPaymentMultiValue, DcdtTokenType, ManagedType, ManagedVecItem,
        TokenIdentifier,
    },
};

use crate as dharitri_sc; // needed by the codec and TypeAbi generated code
use crate::{
    codec::{
        self,
        derive::{NestedEncode, TopEncode},
        IntoMultiValue, NestedDecode, TopDecode,
    },
    derive::type_abi,
};

use super::{
    managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index,
    MoaOrDcdtTokenIdentifier, MoaOrDcdtTokenPayment, ManagedVec, ManagedVecItemPayloadBuffer,
    ManagedVecRef, MultiMoaOrDcdtPayment,
};

#[type_abi]
#[derive(TopEncode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub struct DcdtTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

/// Alias for a list of payments.
pub type MultiDcdtPayment<Api> = ManagedVec<Api, DcdtTokenPayment<Api>>;

impl<M: ManagedTypeApi> DcdtTokenPayment<M> {
    #[inline]
    pub fn new(token_identifier: TokenIdentifier<M>, token_nonce: u64, amount: BigUint<M>) -> Self {
        DcdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn token_type(&self) -> DcdtTokenType {
        if self.amount != 0 {
            if self.token_nonce == 0 {
                DcdtTokenType::Fungible
            } else if self.amount == 1u64 {
                DcdtTokenType::NonFungible
            } else {
                DcdtTokenType::SemiFungible
            }
        } else {
            DcdtTokenType::Invalid
        }
    }

    #[inline]
    pub fn into_tuple(self) -> (TokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }

    /// Zero-cost conversion that loosens the MOA restriction.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn as_moa_or_dcdt_payment(&self) -> &MoaOrDcdtTokenPayment<M> {
        unsafe { core::mem::transmute(self) }
    }

    /// Conversion that loosens the MOA restriction.
    pub fn into_multi_moa_or_dcdt_payment(self) -> MoaOrDcdtTokenPayment<M> {
        MoaOrDcdtTokenPayment {
            token_identifier: MoaOrDcdtTokenIdentifier::dcdt(self.token_identifier),
            token_nonce: self.token_nonce,
            amount: self.amount,
        }
    }
}

impl<M: ManagedTypeApi> From<(TokenIdentifier<M>, u64, BigUint<M>)> for DcdtTokenPayment<M> {
    #[inline]
    fn from(value: (TokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> TopDecode for DcdtTokenPayment<M> {
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

impl<M: ManagedTypeApi> NestedDecode for DcdtTokenPayment<M> {
    #[cfg(not(feature = "dcdt-token-payment-legacy-decode"))]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Self::regular_dep_decode_or_handle_err(input, h)
    }

    #[cfg(feature = "dcdt-token-payment-legacy-decode")]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Self::backwards_compatible_dep_decode_or_handle_err(input, h)
    }
}

impl<M: ManagedTypeApi> DcdtTokenPayment<M> {
    #[doc(hidden)]
    pub fn regular_dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        Ok(DcdtTokenPayment {
            token_identifier: TokenIdentifier::<M>::dep_decode_or_handle_err(input, h)?,
            token_nonce: <u64>::dep_decode_or_handle_err(input, h)?,
            amount: BigUint::<M>::dep_decode_or_handle_err(input, h)?,
        })
    }

    /// Deserializer version that accepts bytes encoded with an older version of the code.
    /// It uses some assumptions about the possible values of the token identifier to figure it out.
    ///
    /// More specifically:
    /// - The old encoding added an extra first byte, indicating the token type.
    /// - Token identifiers cannot be empty and cannot be very long ...
    /// - ... therefore if the bytes are shifted by 1, we should get an invalid token identifier length.
    ///
    /// Even more specifically:
    /// - having the first byte > 0 can only be explained by an older encoding
    /// - having the las byte zero can only mean 2 things:
    ///     - the token identifier is empty - but this is invalid
    ///     - we are reading an older encoding and the las token identifier length byte is the 5th instead of the 4th.
    ///
    /// **Please do not call directly in contracts, use it via the `dcdt-token-payment-legacy-decode` feature flag instead.**
    ///
    /// It is only publicly exposed for testing.
    pub fn backwards_compatible_dep_decode_or_handle_err<I, H>(
        input: &mut I,
        h: H,
    ) -> Result<Self, H::HandledErr>
    where
        I: codec::NestedDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let mut first_four_bytes = [0u8; 4];
        input.peek_into(&mut first_four_bytes[..], h)?;
        // old encoding detection, see method description for explanation
        let old_encoding = first_four_bytes[3] == 0 || first_four_bytes[0] > 0;
        if old_encoding {
            // clear legacy token type field, 1 byte
            let _ = input.read_byte(h)?;
        }
        Self::regular_dep_decode_or_handle_err(input, h)
    }
}

impl<M: ManagedTypeApi> IntoMultiValue for DcdtTokenPayment<M> {
    type MultiValue = DcdtTokenPaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for DcdtTokenPayment<M> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<16>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = ManagedVecRef<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            DcdtTokenPayment {
                token_identifier: managed_vec_item_read_from_payload_index(payload, &mut index),
                token_nonce: managed_vec_item_read_from_payload_index(payload, &mut index),
                amount: managed_vec_item_read_from_payload_index(payload, &mut index),
            }
        }
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        ManagedVecRef::new(Self::read_from_payload(payload))
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

/// The version of `DcdtTokenPayment` that contains referrences instead of owned fields.
pub struct DcdtTokenPaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: &'a TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: &'a BigUint<M>,
}

impl<M: ManagedTypeApi> DcdtTokenPayment<M> {
    pub fn as_refs(&self) -> DcdtTokenPaymentRefs<'_, M> {
        DcdtTokenPaymentRefs::new(&self.token_identifier, self.token_nonce, &self.amount)
    }
}

impl<'a, M: ManagedTypeApi> DcdtTokenPaymentRefs<'a, M> {
    #[inline]
    pub fn new(
        token_identifier: &'a TokenIdentifier<M>,
        token_nonce: u64,
        amount: &'a BigUint<M>,
    ) -> Self {
        DcdtTokenPaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will clone the referenced values.
    pub fn to_owned_payment(&self) -> DcdtTokenPayment<M> {
        DcdtTokenPayment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }
}

impl<M: ManagedTypeApi> MultiDcdtPayment<M> {
    /// Zero-cost conversion that loosens the MOA restriction.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn as_multi_moa_or_dcdt_payment(&self) -> &MultiMoaOrDcdtPayment<M> {
        unsafe { core::mem::transmute(self) }
    }

    /// Zero-cost conversion that loosens the MOA restriction.
    ///
    /// It is always safe to do, since the 2 types are guaranteed to have the same layout.
    pub fn into_multi_moa_or_dcdt_payment(self) -> MultiMoaOrDcdtPayment<M> {
        unsafe { MultiMoaOrDcdtPayment::from_handle(self.forget_into_handle()) }
    }
}

impl<M: ManagedTypeApi> From<()> for MultiDcdtPayment<M> {
    #[inline]
    fn from(_value: ()) -> Self {
        MultiDcdtPayment::new()
    }
}

impl<M: ManagedTypeApi> From<DcdtTokenPayment<M>> for MultiDcdtPayment<M> {
    #[inline]
    fn from(value: DcdtTokenPayment<M>) -> Self {
        MultiDcdtPayment::from_single_item(value)
    }
}

impl<M: ManagedTypeApi> From<(TokenIdentifier<M>, u64, BigUint<M>)> for MultiDcdtPayment<M> {
    #[inline]
    fn from(value: (TokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        MultiDcdtPayment::from_single_item(value.into())
    }
}