use dharitri_sc_codec::IntoMultiValue;

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{BigUint, MoaOrDcdtTokenIdentifier, MoaOrDcdtTokenPaymentMultiValue},
};

use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

use crate as dharitri_sc; // needed by the TypeAbi generated code
use crate::derive::type_abi;

use super::{
    managed_vec_item_read_from_payload_index, managed_vec_item_save_to_payload_index,
    DcdtTokenPayment, DcdtTokenPaymentRefs, ManagedVec, ManagedVecItem,
    ManagedVecItemPayloadBuffer, ManagedVecRef,
};

#[type_abi]
#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub struct MoaOrDcdtTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: MoaOrDcdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

/// Alias for a list of payments of MOA or DCDT tokens.
pub type MultiMoaOrDcdtPayment<Api> = ManagedVec<Api, MoaOrDcdtTokenPayment<Api>>;

impl<M: ManagedTypeApi> MoaOrDcdtTokenPayment<M> {
    pub fn no_payment() -> Self {
        Self::moa_payment(BigUint::zero())
    }

    pub fn new(
        token_identifier: MoaOrDcdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        MoaOrDcdtTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// A payment of token MOA-000000.
    pub fn moa_payment(amount: BigUint<M>) -> Self {
        Self::new(MoaOrDcdtTokenIdentifier::moa(), 0, amount)
    }

    /// Will convert to just DCDT or terminate execution if the token is MOA.
    pub fn unwrap_dcdt(self) -> DcdtTokenPayment<M> {
        DcdtTokenPayment::new(
            self.token_identifier.unwrap_dcdt(),
            self.token_nonce,
            self.amount,
        )
    }

    /// Equivalent to a `match { Moa | Dcdt }`.
    ///
    /// Context passed on from function to closures, to avoid ownership issues.
    /// More precisely, since only one of the two closures `for_moa` and `for_dcdt` is called,
    /// it is ok for them to have fully owned access to anything from the environment.
    /// The compiler doesn't know that only one of them can ever be called,
    /// so if we pass context to both closures, it will complain that they are moved twice.
    pub fn map_moa_or_dcdt<Context, D, F, U>(self, context: Context, for_moa: D, for_dcdt: F) -> U
    where
        D: FnOnce(Context, BigUint<M>) -> U,
        F: FnOnce(Context, DcdtTokenPayment<M>) -> U,
    {
        self.token_identifier.map_or_else(
            (context, self.amount),
            |(context, amount)| for_moa(context, amount),
            |(context, amount), token_identifier| {
                for_dcdt(
                    context,
                    DcdtTokenPayment::new(token_identifier, self.token_nonce, amount),
                )
            },
        )
    }

    /// Same as `map_moa_or_dcdt`, but only takes a reference,
    /// and consequently, the closures also only get references.
    pub fn map_ref_moa_or_dcdt<Context, D, F, U>(
        &self,
        context: Context,
        for_moa: D,
        for_dcdt: F,
    ) -> U
    where
        D: FnOnce(Context, &BigUint<M>) -> U,
        F: FnOnce(Context, DcdtTokenPaymentRefs<'_, M>) -> U,
    {
        self.token_identifier.map_ref_or_else(
            context,
            |context| for_moa(context, &self.amount),
            |context, token_identifier| {
                for_dcdt(
                    context,
                    DcdtTokenPaymentRefs::new(token_identifier, self.token_nonce, &self.amount),
                )
            },
        )
    }

    pub fn into_tuple(self) -> (MoaOrDcdtTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(MoaOrDcdtTokenIdentifier<M>, u64, BigUint<M>)>
    for MoaOrDcdtTokenPayment<M>
{
    #[inline]
    fn from(value: (MoaOrDcdtTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> From<DcdtTokenPayment<M>> for MoaOrDcdtTokenPayment<M> {
    fn from(dcdt_payment: DcdtTokenPayment<M>) -> Self {
        MoaOrDcdtTokenPayment {
            token_identifier: MoaOrDcdtTokenIdentifier::dcdt(dcdt_payment.token_identifier),
            token_nonce: dcdt_payment.token_nonce,
            amount: dcdt_payment.amount,
        }
    }
}

impl<M: ManagedTypeApi> IntoMultiValue for MoaOrDcdtTokenPayment<M> {
    type MultiValue = MoaOrDcdtTokenPaymentMultiValue<M>;

    #[inline]
    fn into_multi_value(self) -> Self::MultiValue {
        self.into()
    }
}

impl<M> TypeAbiFrom<&[u8]> for MoaOrDcdtTokenPayment<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> MoaOrDcdtTokenPayment<M> {
    pub fn as_refs(&self) -> MoaOrDcdtTokenPaymentRefs<'_, M> {
        MoaOrDcdtTokenPaymentRefs::new(&self.token_identifier, self.token_nonce, &self.amount)
    }
}

/// Similar to `MoaOrDcdtTokenPayment`, but only contains references.
pub struct MoaOrDcdtTokenPaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: &'a MoaOrDcdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: &'a BigUint<M>,
}

impl<'a, M: ManagedTypeApi> MoaOrDcdtTokenPaymentRefs<'a, M> {
    pub fn new(
        token_identifier: &'a MoaOrDcdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: &'a BigUint<M>,
    ) -> MoaOrDcdtTokenPaymentRefs<'a, M> {
        MoaOrDcdtTokenPaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn to_owned_payment(&self) -> MoaOrDcdtTokenPayment<M> {
        MoaOrDcdtTokenPayment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.amount == &0u32
    }

    pub fn map_moa_or_dcdt<Context, D, F, U>(self, context: Context, for_moa: D, for_dcdt: F) -> U
    where
        D: FnOnce(Context, &BigUint<M>) -> U,
        F: FnOnce(Context, DcdtTokenPaymentRefs<M>) -> U,
    {
        self.token_identifier.map_ref_or_else(
            context,
            |context| for_moa(context, self.amount),
            |context, token_identifier| {
                for_dcdt(
                    context,
                    DcdtTokenPaymentRefs::new(token_identifier, self.token_nonce, self.amount),
                )
            },
        )
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for MoaOrDcdtTokenPayment<M> {
    type PAYLOAD = ManagedVecItemPayloadBuffer<16>;
    const SKIPS_RESERIALIZATION: bool = false;
    type Ref<'a> = ManagedVecRef<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        let mut index = 0;
        unsafe {
            MoaOrDcdtTokenPayment {
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
