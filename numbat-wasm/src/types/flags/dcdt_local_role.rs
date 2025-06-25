use numbat_codec::numbat_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use super::DcdtLocalRoleFlags;
use crate as numbat_wasm;
use crate::{derive::TypeAbi, types::ManagedVecItem};

const DCDT_ROLE_NONE: &[u8] = &[];
const DCDT_ROLE_LOCAL_MINT: &[u8] = b"DCDTRoleLocalMint";
const DCDT_ROLE_LOCAL_BURN: &[u8] = b"DCDTRoleLocalBurn";
const DCDT_ROLE_NFT_CREATE: &[u8] = b"DCDTRoleNFTCreate";
const DCDT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"DCDTRoleNFTAddQuantity";
const DCDT_ROLE_NFT_BURN: &[u8] = b"DCDTRoleNFTBurn";
const DCDT_ROLE_NFT_ADD_URI: &[u8] = b"DCDTRoleNFTAddURI";
const DCDT_ROLE_NFT_UPDATE_ATTRIBUTES: &[u8] = b"DCDTRoleNFTUpdateAttributes";

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Debug, Copy,
)]
pub enum DcdtLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
    NftAddUri,
    NftUpdateAttributes,
}

impl DcdtLocalRole {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::Burn => 2,
            Self::NftCreate => 3,
            Self::NftAddQuantity => 4,
            Self::NftBurn => 5,
            Self::NftAddUri => 6,
            Self::NftUpdateAttributes => 7,
        }
    }

    pub fn as_role_name(&self) -> &'static [u8] {
        match self {
            Self::None => DCDT_ROLE_NONE,
            Self::Mint => DCDT_ROLE_LOCAL_MINT,
            Self::Burn => DCDT_ROLE_LOCAL_BURN,
            Self::NftCreate => DCDT_ROLE_NFT_CREATE,
            Self::NftAddQuantity => DCDT_ROLE_NFT_ADD_QUANTITY,
            Self::NftBurn => DCDT_ROLE_NFT_BURN,
            Self::NftAddUri => DCDT_ROLE_NFT_ADD_URI,
            Self::NftUpdateAttributes => DCDT_ROLE_NFT_UPDATE_ATTRIBUTES,
        }
    }

    pub fn to_flag(&self) -> DcdtLocalRoleFlags {
        match self {
            Self::None => DcdtLocalRoleFlags::NONE,
            Self::Mint => DcdtLocalRoleFlags::MINT,
            Self::Burn => DcdtLocalRoleFlags::BURN,
            Self::NftCreate => DcdtLocalRoleFlags::NFT_CREATE,
            Self::NftAddQuantity => DcdtLocalRoleFlags::NFT_ADD_QUANTITY,
            Self::NftBurn => DcdtLocalRoleFlags::NFT_BURN,
            Self::NftAddUri => DcdtLocalRoleFlags::NFT_ADD_URI,
            Self::NftUpdateAttributes => DcdtLocalRoleFlags::NFT_UPDATE_ATTRIBUTES,
        }
    }
}

// TODO: can be done with macros, but I didn't find a public library that does it and is no_std
// we can implement it, it's easy
const ALL_ROLES: [DcdtLocalRole; 7] = [
    DcdtLocalRole::Mint,
    DcdtLocalRole::Burn,
    DcdtLocalRole::NftCreate,
    DcdtLocalRole::NftAddQuantity,
    DcdtLocalRole::NftBurn,
    DcdtLocalRole::NftAddUri,
    DcdtLocalRole::NftUpdateAttributes,
];

impl DcdtLocalRole {
    pub fn iter_all() -> core::slice::Iter<'static, DcdtLocalRole> {
        ALL_ROLES.iter()
    }
}

impl From<u8> for DcdtLocalRole {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Mint,
            2 => Self::Burn,
            3 => Self::NftCreate,
            4 => Self::NftAddQuantity,
            5 => Self::NftBurn,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for DcdtLocalRole {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == DCDT_ROLE_LOCAL_MINT {
            Self::Mint
        } else if byte_slice == DCDT_ROLE_LOCAL_BURN {
            Self::Burn
        } else if byte_slice == DCDT_ROLE_NFT_CREATE {
            Self::NftCreate
        } else if byte_slice == DCDT_ROLE_NFT_ADD_QUANTITY {
            Self::NftAddQuantity
        } else if byte_slice == DCDT_ROLE_NFT_BURN {
            Self::NftBurn
        } else {
            Self::None
        }
    }
}

impl ManagedVecItem for DcdtLocalRole {
    const PAYLOAD_SIZE: usize = 1;
    const SKIPS_RESERIALIZATION: bool = false; // TODO: might be ok to be true, but needs testing
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        u8::from_byte_reader(reader).into()
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        <u8 as ManagedVecItem>::to_byte_writer(&self.as_u8(), writer)
    }
}
