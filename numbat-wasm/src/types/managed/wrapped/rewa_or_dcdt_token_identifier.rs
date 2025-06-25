use crate::{
    abi::{TypeAbi, TypeName},
    api::{HandleConstraints, ManagedTypeApi},
    derive::ManagedVecItem,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    types::{ManagedBuffer, ManagedOption, ManagedRef, ManagedType, TokenIdentifier},
};
use numbat_codec::*;

use crate as numbat_wasm; // required by the ManagedVecItem derive

/// Specialized type for handling either REWA or DCDT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// # use numbat_wasm::{api::ManagedTypeApi, types::TokenIdentifier};
/// enum RewaOrDcdtTokenIdentifier<M: ManagedTypeApi> {
///     Rewa,
///     Dcdt(TokenIdentifier<M>),
/// }
/// ```
///
/// It is, however more optimized than that. Its implementation is based on `ManagedOption`.
///
/// REWA a special, invalid token identifier handle. This way we can fit it inside a single i32 in memory.
#[repr(transparent)]
#[derive(ManagedVecItem, Clone)]
pub struct RewaOrDcdtTokenIdentifier<M: ManagedTypeApi> {
    data: ManagedOption<M, TokenIdentifier<M>>,
}

impl<M: ManagedTypeApi> RewaOrDcdtTokenIdentifier<M> {
    /// This special representation is interpreted as the REWA token.
    #[allow(clippy::needless_borrow)] // clippy is wrog here, there is no other way
    pub const REWA_REPRESENTATION: &'static [u8; 4] = &b"REWA";

    /// New instance of the special REWA token representation.
    #[inline]
    pub fn rewa() -> Self {
        Self {
            data: ManagedOption::none(),
        }
    }

    /// DCDT instance, containing an DCDT token identifier.
    #[inline]
    pub fn dcdt<TI>(token_identifier: TI) -> Self
    where
        TokenIdentifier<M>: From<TI>,
    {
        Self {
            data: ManagedOption::some(TokenIdentifier::from(token_identifier)),
        }
    }

    pub fn from_opt_raw_handle(opt_handle: Option<M::ManagedBufferHandle>) -> Self {
        match opt_handle {
            Some(handle) => Self::dcdt(TokenIdentifier::from_handle(handle)),
            None => Self::rewa(),
        }
    }

    pub fn parse(data: ManagedBuffer<M>) -> Self {
        if data == Self::REWA_REPRESENTATION {
            Self::rewa()
        } else {
            Self::dcdt(TokenIdentifier::from(data))
        }
    }

    #[inline]
    pub fn is_rewa(&self) -> bool {
        self.data.is_none()
    }

    #[inline]
    pub fn is_dcdt(&self) -> bool {
        self.data.is_some()
    }

    #[inline]
    pub fn into_name(self) -> ManagedBuffer<M> {
        self.map_or_else(
            || ManagedBuffer::from(&Self::REWA_REPRESENTATION[..]),
            |token_identifier| token_identifier.into_managed_buffer(),
        )
    }

    /// Checks the DCDT token identifier for validity. REWA is considered valid, no checks needed.
    ///
    /// Will fail if it encodes an invalid DCDT token identifier.
    pub fn is_valid(&self) -> bool {
        self.map_ref_or_else(
            || true,
            |token_identifier| token_identifier.is_valid_dcdt_identifier(),
        )
    }

    pub fn map_or_else<U, D, F>(self, for_rewa: D, for_dcdt: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(TokenIdentifier<M>) -> U,
    {
        self.data.map_or_else(for_rewa, for_dcdt)
    }

    pub fn map_ref_or_else<U, D, F>(&self, for_rewa: D, for_dcdt: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&TokenIdentifier<M>) -> U,
    {
        self.data.map_ref_or_else(for_rewa, for_dcdt)
    }

    pub fn unwrap_dcdt(self) -> TokenIdentifier<M> {
        self.data.unwrap_or_sc_panic("DCDT expected")
    }

    /// Representation of the object as an `Option`.
    ///
    /// Because it does not consume `self` only a reference to the DCDT token identifier can be returned.
    pub fn as_dcdt_option(&self) -> Option<ManagedRef<'_, M, TokenIdentifier<M>>> {
        self.data.as_option()
    }

    /// Converts `self` into an `Option`. Consumes `self` in the process.
    pub fn into_dcdt_option(self) -> Option<TokenIdentifier<M>> {
        self.data.into_option()
    }
}

impl<M: ManagedTypeApi> PartialEq for RewaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<M: ManagedTypeApi> Eq for RewaOrDcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<TokenIdentifier<M>> for RewaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &TokenIdentifier<M>) -> bool {
        self.map_ref_or_else(
            || false,
            |self_dcdt_token_identifier| self_dcdt_token_identifier == other,
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for RewaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if let Some(token_identifier) = self.data.as_option() {
            token_identifier.dep_encode_or_handle_err(dest, h)
        } else {
            (&Self::REWA_REPRESENTATION[..]).dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for RewaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if let Some(token_identifier) = self.data.as_option() {
            token_identifier.top_encode_or_handle_err(output, h)
        } else {
            (&Self::REWA_REPRESENTATION[..]).top_encode_or_handle_err(output, h)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for RewaOrDcdtTokenIdentifier<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::parse(ManagedBuffer::dep_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M: ManagedTypeApi> TopDecode for RewaOrDcdtTokenIdentifier<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::parse(ManagedBuffer::top_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M> CodecFromSelf for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<TokenIdentifier<M>> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> CodecFrom<&TokenIdentifier<M>> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> CodecFrom<&str> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbi for RewaOrDcdtTokenIdentifier<M> {
    fn type_name() -> TypeName {
        "RewaOrDcdtTokenIdentifier".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for RewaOrDcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer(&ManagedBuffer::from_handle(
                token_identifier.get_handle().cast_or_signal_error::<M, _>(),
            ));
        } else {
            f.append_bytes(Self::REWA_REPRESENTATION);
        }
    }
}

const REWA_REPRESENTATION_HEX: &[u8] = b"45474C44";

impl<M: ManagedTypeApi> SCLowerHex for RewaOrDcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer_lower_hex(&ManagedBuffer::from_handle(
                token_identifier.get_handle().cast_or_signal_error::<M, _>(),
            ));
        } else {
            f.append_bytes(REWA_REPRESENTATION_HEX);
        }
    }
}

impl<M> core::fmt::Debug for RewaOrDcdtTokenIdentifier<M>
where
    M: ManagedTypeApi,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::alloc::string::ToString;
        if let Some(token_identifier) = self.data.as_option() {
            let token_id_str = token_identifier.to_string();
            f.debug_tuple("RewaOrDcdtTokenIdentifier::Dcdt")
                .field(&token_id_str)
                .finish()
        } else {
            f.write_str("RewaOrDcdtTokenIdentifier::Rewa")
        }
    }
}
