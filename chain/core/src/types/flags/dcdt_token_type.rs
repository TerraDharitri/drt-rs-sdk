// use dharitri_sc_derive::{type_abi, ManagedVecItem};

use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

const DCDT_TYPE_FUNGIBLE: &[u8] = b"FungibleDCDT";
const DCDT_TYPE_NON_FUNGIBLE: &[u8] = b"NonFungibleDCDT";
const DCDT_TYPE_SEMI_FUNGIBLE: &[u8] = b"SemiFungibleDCDT";
const DCDT_TYPE_META: &[u8] = b"MetaDCDT";
const DCDT_TYPE_INVALID: &[u8] = &[];

// Note: In the current implementation, SemiFungible is never returned

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub enum DcdtTokenType {
    Fungible,
    NonFungible,
    SemiFungible,
    Meta,
    Invalid,
}

impl DcdtTokenType {
    pub fn based_on_token_nonce(token_nonce: u64) -> Self {
        if token_nonce == 0 {
            DcdtTokenType::Fungible
        } else {
            DcdtTokenType::NonFungible
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Fungible => 0,
            Self::NonFungible => 1,
            Self::SemiFungible => 2,
            Self::Meta => 3,
            Self::Invalid => 4,
        }
    }

    pub fn as_type_name(&self) -> &'static [u8] {
        match self {
            Self::Fungible => DCDT_TYPE_FUNGIBLE,
            Self::NonFungible => DCDT_TYPE_NON_FUNGIBLE,
            Self::SemiFungible => DCDT_TYPE_SEMI_FUNGIBLE,
            Self::Meta => DCDT_TYPE_META,
            Self::Invalid => DCDT_TYPE_INVALID,
        }
    }
}

impl From<u8> for DcdtTokenType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Fungible,
            1 => Self::NonFungible,
            2 => Self::SemiFungible,
            3 => Self::Meta,
            _ => Self::Invalid,
        }
    }
}

impl<'a> From<&'a [u8]> for DcdtTokenType {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == DCDT_TYPE_FUNGIBLE {
            Self::Fungible
        } else if byte_slice == DCDT_TYPE_NON_FUNGIBLE {
            Self::NonFungible
        } else if byte_slice == DCDT_TYPE_SEMI_FUNGIBLE {
            Self::SemiFungible
        } else if byte_slice == DCDT_TYPE_META {
            Self::Meta
        } else {
            Self::Invalid
        }
    }
}
