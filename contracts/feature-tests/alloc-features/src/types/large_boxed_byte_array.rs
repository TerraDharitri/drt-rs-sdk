use dharitri_sc::types::Box;
dharitri_sc::derive_imports!();

const ARRAY_SIZE: usize = 512;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct LargeBoxedByteArray(Box<[u8; ARRAY_SIZE]>);