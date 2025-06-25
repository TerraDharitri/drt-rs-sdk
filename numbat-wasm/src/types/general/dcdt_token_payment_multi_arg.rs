use alloc::string::String;

use crate::{
    abi::TypeAbi,
    api::{EndpointFinishApi, ManagedTypeApi},
    types::{BigUint, ManagedVecItem},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};

use super::{DcdtTokenPayment, TokenIdentifier};

/// Thin wrapper around DcdtTokenPayment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Debug)]
pub struct DcdtTokenPaymentMultiArg<M: ManagedTypeApi> {
    obj: DcdtTokenPayment<M>,
}

impl<M: ManagedTypeApi> From<DcdtTokenPayment<M>> for DcdtTokenPaymentMultiArg<M> {
    #[inline]
    fn from(obj: DcdtTokenPayment<M>) -> Self {
        DcdtTokenPaymentMultiArg { obj }
    }
}

impl<M: ManagedTypeApi> DcdtTokenPaymentMultiArg<M> {
    pub fn into_dcdt_token_payment(self) -> DcdtTokenPayment<M> {
        self.obj
    }
}

impl<M: ManagedTypeApi> ManagedVecItem<M> for DcdtTokenPaymentMultiArg<M> {
    const PAYLOAD_SIZE: usize = DcdtTokenPayment::<M>::PAYLOAD_SIZE;
    const NEEDS_RESERIALIZATION: bool = DcdtTokenPayment::<M>::NEEDS_RESERIALIZATION;

    #[inline]
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, reader: Reader) -> Self {
        DcdtTokenPayment::from_byte_reader(api, reader).into()
    }

    #[inline]
    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        self.obj.to_byte_writer(writer)
    }
}

impl<M> DynArg for DcdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let token_identifier = TokenIdentifier::dyn_load(loader, arg_id);
        let token_nonce = u64::dyn_load(loader, arg_id);
        let amount = BigUint::dyn_load(loader, arg_id);
        DcdtTokenPayment::new(token_identifier, token_nonce, amount).into()
    }
}

impl<M> EndpointResult for DcdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    type DecodeAs = DcdtTokenPaymentMultiArg<M>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        self.obj.token_identifier.finish(api.clone());
        self.obj.token_nonce.finish(api.clone());
        self.obj.amount.finish(api);
    }
}

impl<M> ContractCallArg for &DcdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        self.obj.token_identifier.push_dyn_arg(output);
        self.obj.token_nonce.push_dyn_arg(output);
        self.obj.amount.push_dyn_arg(output);
    }
}

impl<M> ContractCallArg for DcdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M> TypeAbi for DcdtTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn type_name() -> String {
        crate::types::MultiArg3::<TokenIdentifier<M>, u64, BigUint<M>>::type_name()
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}
