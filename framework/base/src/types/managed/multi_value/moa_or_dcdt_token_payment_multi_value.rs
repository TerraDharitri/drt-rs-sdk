use crate::{
    abi::TypeAbiFrom,
    codec::{
        multi_types::MultiValue3, DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti,
        TopDecodeMultiInput, TopDecodeMultiLength, TopEncodeMulti, TopEncodeMultiOutput,
    },
    types::{MoaOrDcdtTokenIdentifier, ManagedVecRef},
};

use crate::{
    abi::{TypeAbi, TypeName},
    api::ManagedTypeApi,
    types::{BigUint, MoaOrDcdtTokenPayment, ManagedVecItem, TokenIdentifier},
};

/// Thin wrapper around MoaOrDcdtTokenPayment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MoaOrDcdtTokenPaymentMultiValue<M: ManagedTypeApi> {
    obj: MoaOrDcdtTokenPayment<M>,
}

impl<M: ManagedTypeApi> From<MoaOrDcdtTokenPayment<M>> for MoaOrDcdtTokenPaymentMultiValue<M> {
    #[inline]
    fn from(obj: MoaOrDcdtTokenPayment<M>) -> Self {
        MoaOrDcdtTokenPaymentMultiValue { obj }
    }
}

impl<M: ManagedTypeApi> MoaOrDcdtTokenPaymentMultiValue<M> {
    pub fn into_inner(self) -> MoaOrDcdtTokenPayment<M> {
        self.obj
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for MoaOrDcdtTokenPaymentMultiValue<M> {
    type PAYLOAD = <MoaOrDcdtTokenPayment<M> as ManagedVecItem>::PAYLOAD;
    const SKIPS_RESERIALIZATION: bool = MoaOrDcdtTokenPayment::<M>::SKIPS_RESERIALIZATION;
    type Ref<'a> = ManagedVecRef<'a, Self>;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        MoaOrDcdtTokenPayment::read_from_payload(payload).into()
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        ManagedVecRef::new(Self::read_from_payload(payload))
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.obj.save_to_payload(payload);
    }
}

impl<M> TopEncodeMulti for MoaOrDcdtTokenPaymentMultiValue<M>
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

impl<M> TopDecodeMulti for MoaOrDcdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let token_identifier = MoaOrDcdtTokenIdentifier::multi_decode_or_handle_err(input, h)?;
        let token_nonce = u64::multi_decode_or_handle_err(input, h)?;
        let amount = BigUint::multi_decode_or_handle_err(input, h)?;
        Ok(MoaOrDcdtTokenPayment::new(token_identifier, token_nonce, amount).into())
    }
}

impl<M> TopDecodeMultiLength for MoaOrDcdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    const LEN: usize = 3;
}

impl<M> TypeAbiFrom<Self> for MoaOrDcdtTokenPaymentMultiValue<M> where M: ManagedTypeApi {}

impl<M> TypeAbi for MoaOrDcdtTokenPaymentMultiValue<M>
where
    M: ManagedTypeApi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        MultiValue3::<TokenIdentifier<M>, u64, BigUint<M>>::type_name()
    }

    fn type_name_rust() -> TypeName {
        "MoaOrDcdtTokenPaymentMultiValue<$API>".into()
    }

    fn is_variadic() -> bool {
        true
    }
}