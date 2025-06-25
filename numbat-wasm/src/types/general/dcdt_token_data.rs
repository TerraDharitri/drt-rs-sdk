use crate::{
    abi::TypeAbi,
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress, ManagedBuffer, ManagedType, ManagedVec},
};
use alloc::string::String;
use numbat_codec::*;

use super::DcdtTokenType;

use numbat_codec::numbat_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Debug)]
pub struct DcdtTokenData<M: ManagedTypeApi> {
    pub token_type: DcdtTokenType,
    pub amount: BigUint<M>,
    pub frozen: bool,
    pub hash: ManagedBuffer<M>,
    pub name: ManagedBuffer<M>,
    pub attributes: ManagedBuffer<M>,
    pub creator: ManagedAddress<M>,
    pub royalties: BigUint<M>,
    pub uris: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> DcdtTokenData<M> {
    pub fn type_manager(&self) -> M {
        self.amount.type_manager()
    }

    pub fn decode_attributes<T: TopDecode>(&self) -> Result<T, DecodeError> {
        T::top_decode(&self.attributes)
    }
}

impl<M: ManagedTypeApi> TypeAbi for DcdtTokenData<M> {
    fn type_name() -> String {
        "DcdtTokenData".into()
    }
}
