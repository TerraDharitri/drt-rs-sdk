use hex_literal::hex;
use dharitri_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::{CallTypeApi, ManagedTypeApi},
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxScEnv, TxTo, TxToSpecified},
};

/// Address of the system smart contract that manages DCDT.
const SYSTEM_SC_ADDRESS_BYTES: [u8; 32] =
    hex!("233300000000000000000000000000000002333000000000000000000002ffff");
const SYSTEM_SC_ADDRESS_BECH32: &str =
    "drt1yvesqqqqqqqqqqqqqqqqqqqqqqqqyvesqqqqqqqqqqqqqqqzlllsd5j0s2";
const SYSTEM_SC_ADDRESS_ANNOTATION: &str =
    "bech32:drt1yvesqqqqqqqqqqqqqqqqqqqqqqqqyvesqqqqqqqqqqqqqqqzlllsd5j0s2";

/// Indicates the system SC address, which is the same on any DharitrI blockchain.
pub struct DCDTSystemSCAddress;

impl DCDTSystemSCAddress {
    pub fn to_managed_address<Api>(self) -> ManagedAddress<Api>
    where
        Api: ManagedTypeApi,
    {
        ManagedAddress::from(SYSTEM_SC_ADDRESS_BYTES)
    }

    pub fn to_bech32_str(&self) -> &str {
        SYSTEM_SC_ADDRESS_BECH32
    }

    pub fn to_bech32_string(&self) -> alloc::string::String {
        SYSTEM_SC_ADDRESS_BECH32.into()
    }
}

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for DCDTSystemSCAddress
where
    Api: CallTypeApi,
{
    fn annotation(&self, _env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        ManagedBuffer::from(SYSTEM_SC_ADDRESS_ANNOTATION)
    }

    fn to_value(&self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        DCDTSystemSCAddress.to_managed_address()
    }
}

impl<Api> TxTo<TxScEnv<Api>> for DCDTSystemSCAddress where Api: CallTypeApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for DCDTSystemSCAddress where Api: CallTypeApi {}

impl TopEncode for DCDTSystemSCAddress {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        SYSTEM_SC_ADDRESS_BYTES.top_encode_or_handle_err(output, h)
    }
}

impl<M> TypeAbiFrom<DCDTSystemSCAddress> for ManagedAddress<M> where M: ManagedTypeApi {}

impl core::fmt::Display for DCDTSystemSCAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(SYSTEM_SC_ADDRESS_BECH32)
    }
}
