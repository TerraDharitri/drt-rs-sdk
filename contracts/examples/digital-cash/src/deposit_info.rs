use dharitri_sc::{
    api::ManagedTypeApi,
    types::{RewaOrDcdtTokenIdentifier, RewaOrDcdtTokenPayment, ManagedAddress},
};

dharitri_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub payment: RewaOrDcdtTokenPayment<M>,
    pub expiration_round: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, ManagedVecItem)]
pub struct FundType<M: ManagedTypeApi> {
    pub token: RewaOrDcdtTokenIdentifier<M>,
    pub nonce: u64,
}
