use crate::{
    abi::TypeAbiFrom,
    codec::{
        DecodeErrorHandler, EncodeErrorHandler, MultiValueConstLength, TopDecodeMulti,
        TopDecodeMultiInput, TopEncodeMulti, TopEncodeMultiOutput, multi_types::MultiValue3,
    },
    types::Ref,
};

use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    types::{BigUint, DcdtTokenIdentifier, DcdtTokenPayment, ManagedVecItem},
};

/// Thin wrapper around DcdtTokenPayment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DcdtTokenPaymentMultiValue<M: ManagedTypeApi> {
    obj: DcdtTokenPayment<M>,
}

#[deprecated(
    since = "0.29.3",
    note = "Alias kept for backwards compatibility. Replace with `DcdtTokenPaymentMultiValue`"
)]
pub type DcdtTokenPaymentMultiArg<M> = DcdtTokenPaymentMultiValue<M>;

impl<M: ManagedTypeApi> From<DcdtTokenPayment<M>> for DcdtTokenPaymentMultiValue<M> {
    #[inline]
    fn from(obj: DcdtTokenPayment<M>) -> Self {
        DcdtTokenPaymentMultiValue { obj }
    }
}

impl<M: ManagedTypeApi> DcdtTokenPaymentMultiValue<M> {
    pub fn into_inner(self) -> DcdtTokenPayment<M> {
        self.obj
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for DcdtTokenPaymentMultiValue<M> {
    type PAYLOAD = <DcdtTokenPayment<M> as ManagedVecItem>::PAYLOAD;
    const SKIPS_RESERIALIZATION: bool = DcdtTokenPayment::<M>::SKIPS_RESERIALIZATION;
    type Ref<'a> = Ref<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        DcdtTokenPayment::read_from_payload(payload).into()
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        unsafe { Ref::new(Self::read_from_payload(payload)) }
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.obj.save_to_payload(payload);
    }
}

impl<M> TopEncodeMulti for DcdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        output.push_single_value(&self.obj.token_identifier, h)?;
        output.push_single_value(&self.obj.token_nonce, h)?;
        output.push_single_value(&self.obj.amount, h)?;
        Ok(())
    }
}

impl<M> TopDecodeMulti for DcdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let token_identifier = DcdtTokenIdentifier::multi_decode_or_handle_err(input, h)?;
        let token_nonce = u64::multi_decode_or_handle_err(input, h)?;
        let amount = BigUint::multi_decode_or_handle_err(input, h)?;
        Ok(DcdtTokenPayment::new(token_identifier, token_nonce, amount).into())
    }
}

impl<M> MultiValueConstLength for DcdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    const MULTI_VALUE_CONST_LEN: usize = 3;
}

impl<M> TypeAbiFrom<Self> for DcdtTokenPaymentMultiValue<M> where M: ManagedTypeApi {}

impl<M> TypeAbi for DcdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        MultiValue3::<DcdtTokenIdentifier<M>, u64, BigUint<M>>::type_name()
    }

    fn type_name_rust() -> TypeName {
        "DcdtTokenPaymentMultiValue<$API>".into()
    }

    fn is_variadic() -> bool {
        true
    }
}
