use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, Copy, PartialEq, Eq, Debug)]
pub enum DcdtTokenType {
    Fungible = 0,
    NonFungible = 1,
    NonFungibleV2 = 2,
    SemiFungible = 3,
    MetaFungible = 4,
    DynamicNFT = 5,
    DynamicSFT = 6,
    DynamicMeta = 7,
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
            DcdtTokenType::Fungible => 0,
            DcdtTokenType::NonFungible => 1,
            DcdtTokenType::NonFungibleV2 => 2,
            DcdtTokenType::SemiFungible => 3,
            DcdtTokenType::MetaFungible => 4,
            DcdtTokenType::DynamicNFT => 5,
            DcdtTokenType::DynamicSFT => 6,
            DcdtTokenType::DynamicMeta => 7,
            DcdtTokenType::Invalid => 255,
        }
    }

    pub fn collection_from_string(token_type: &str) -> Self {
        match token_type {
            "NonFungibleDCDT" => DcdtTokenType::NonFungibleV2,
            "SemiFungibleDCDT" => DcdtTokenType::SemiFungible,
            "MetaDCDT" => DcdtTokenType::MetaFungible,
            "DynamicNonFungibleDCDT" => DcdtTokenType::DynamicNFT,
            "DynamicSemiFungibleDCDT" => DcdtTokenType::DynamicSFT,
            "DynamicMetaDCDT" => DcdtTokenType::DynamicMeta,
            _ => DcdtTokenType::Invalid,
        }
    }
}

impl From<u8> for DcdtTokenType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => DcdtTokenType::Fungible,
            1 => DcdtTokenType::NonFungible,
            2 => DcdtTokenType::NonFungibleV2,
            3 => DcdtTokenType::SemiFungible,
            4 => DcdtTokenType::MetaFungible,
            5 => DcdtTokenType::DynamicNFT,
            6 => DcdtTokenType::DynamicSFT,
            7 => DcdtTokenType::DynamicMeta,
            _ => DcdtTokenType::Invalid,
        }
    }
}
