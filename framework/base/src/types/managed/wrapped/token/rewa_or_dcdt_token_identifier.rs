use alloc::string::{String, ToString};

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{ErrorApiImpl, ManagedTypeApi},
    codec::*,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    proxy_imports::TestTokenIdentifier,
    types::{
        DcdtTokenIdentifier, LEGACY_REWA_REPRESENTATION, ManagedBuffer, ManagedRef, ManagedType,
        TokenId,
    },
};

/// Specialized type for handling either REWA or DCDT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// # use dharitri_sc::{api::ManagedTypeApi, types::DcdtTokenIdentifier};
/// enum RewaOrDcdtTokenIdentifier<M: ManagedTypeApi> {
///     Rewa,
///     Dcdt(DcdtTokenIdentifier<M>),
/// }
/// ```
///
/// It is, however more optimized than that. Its implementation is based on `ManagedOption`.
///
/// REWA is indicated by a special, invalid token identifier handle.
/// This way we can fit it inside a single i32 in memory.
#[repr(transparent)]
#[derive(Clone)]
pub struct RewaOrDcdtTokenIdentifier<M: ManagedTypeApi> {
    pub(crate) token_id: TokenId<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for RewaOrDcdtTokenIdentifier<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        unsafe {
            RewaOrDcdtTokenIdentifier {
                token_id: TokenId::from_handle(handle),
            }
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.token_id.get_handle()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        unsafe { self.token_id.forget_into_handle() }
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::ManagedBufferHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> RewaOrDcdtTokenIdentifier<M> {
    /// This special representation is interpreted as the REWA token.
    pub const REWA_REPRESENTATION: &'static [u8; 4] = LEGACY_REWA_REPRESENTATION;

    /// New instance of the special REWA token representation.
    #[inline]
    pub fn rewa() -> Self {
        RewaOrDcdtTokenIdentifier {
            token_id: TokenId::native(),
        }
    }

    /// DCDT instance, containing an DCDT token identifier.
    #[inline]
    pub fn dcdt<TI>(token_identifier: TI) -> Self
    where
        DcdtTokenIdentifier<M>: From<TI>,
    {
        let ti_obj = DcdtTokenIdentifier::from(token_identifier);
        ti_obj.token_id.into()
    }

    pub fn parse(data: ManagedBuffer<M>) -> Self {
        if data == Self::REWA_REPRESENTATION {
            Self::rewa()
        } else {
            Self {
                token_id: data.into(),
            }
        }
    }

    #[inline]
    pub fn is_rewa(&self) -> bool {
        self.token_id.is_native()
    }

    #[inline]
    pub fn is_dcdt(&self) -> bool {
        !self.is_rewa()
    }

    /// Returns "REWA" or the token identifier.
    pub fn into_name(self) -> ManagedBuffer<M> {
        self.map_or_else(
            (),
            |()| ManagedBuffer::from(&Self::REWA_REPRESENTATION[..]),
            |(), token_identifier| token_identifier.into_managed_buffer(),
        )
    }

    /// Checks the DCDT token identifier for validity. REWA is considered valid, no checks needed.
    ///
    /// Will fail if it encodes an invalid DCDT token identifier.
    pub fn is_valid(&self) -> bool {
        self.map_ref_or_else(
            (),
            |()| true,
            |(), token_identifier| token_identifier.is_valid_dcdt_identifier(),
        )
    }

    /// Converts reference to the newer, non-legacy TokenId.
    pub fn as_token_id(&self) -> &TokenId<M> {
        // safe because of #[repr(transparent)]
        unsafe { core::mem::transmute(self) }
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.token_id.buffer
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        &self.token_id.buffer
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.token_id.to_boxed_bytes()
    }

    pub fn map_or_else<Context, D, F, R>(self, context: Context, for_rewa: D, for_dcdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, DcdtTokenIdentifier<M>) -> R,
    {
        if self.is_rewa() {
            for_rewa(context)
        } else {
            unsafe { for_dcdt(context, DcdtTokenIdentifier::dcdt_unchecked(self)) }
        }
    }

    pub fn map_ref_or_else<Context, D, F, R>(&self, context: Context, for_rewa: D, for_dcdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, &DcdtTokenIdentifier<M>) -> R,
    {
        if self.is_rewa() {
            for_rewa(context)
        } else {
            unsafe {
                let token_identifier =
                    ManagedRef::<'_, M, DcdtTokenIdentifier<M>>::wrap_handle(self.get_handle());
                for_dcdt(context, &token_identifier)
            }
        }
    }

    pub fn unwrap_dcdt(self) -> DcdtTokenIdentifier<M> {
        self.map_or_else(
            (),
            |()| {
                M::error_api_impl().signal_error(err_msg::TOKEN_IDENTIFIER_DCDT_EXPECTED.as_bytes())
            },
            |(), token_identifier| token_identifier,
        )
    }

    /// Representation of the object as an `Option`.
    ///
    /// Because it does not consume `self` only a reference to the DCDT token identifier can be returned.
    pub fn as_dcdt_option(&self) -> Option<ManagedRef<'_, M, DcdtTokenIdentifier<M>>> {
        if self.is_rewa() {
            None
        } else {
            unsafe {
                Some(ManagedRef::<'_, M, DcdtTokenIdentifier<M>>::wrap_handle(
                    self.get_handle(),
                ))
            }
        }
    }

    /// Converts `self` into an `Option`. Consumes `self` in the process.
    pub fn into_dcdt_option(self) -> Option<DcdtTokenIdentifier<M>> {
        self.map_or_else((), |()| None, |(), token_identifier| Some(token_identifier))
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for RewaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        RewaOrDcdtTokenIdentifier {
            token_id: buffer.into(),
        }
    }
}

impl<M: ManagedTypeApi> From<TokenId<M>> for RewaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn from(token_id: TokenId<M>) -> Self {
        RewaOrDcdtTokenIdentifier { token_id }
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for RewaOrDcdtTokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        RewaOrDcdtTokenIdentifier {
            token_id: TokenId::from(bytes),
        }
    }
}

impl<M: ManagedTypeApi> From<&str> for RewaOrDcdtTokenIdentifier<M> {
    fn from(s: &str) -> Self {
        RewaOrDcdtTokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> From<&String> for RewaOrDcdtTokenIdentifier<M> {
    fn from(s: &String) -> Self {
        RewaOrDcdtTokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> PartialEq for RewaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.token_id == other.token_id
    }
}

impl<M: ManagedTypeApi> Eq for RewaOrDcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<DcdtTokenIdentifier<M>> for RewaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &DcdtTokenIdentifier<M>) -> bool {
        self.map_ref_or_else(
            (),
            |()| false,
            |(), self_dcdt_token_identifier| self_dcdt_token_identifier == other,
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
        if self.is_rewa() {
            (&Self::REWA_REPRESENTATION[..]).dep_encode_or_handle_err(dest, h)
        } else {
            self.token_id.dep_encode_or_handle_err(dest, h)
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
        if self.is_rewa() {
            (&Self::REWA_REPRESENTATION[..]).top_encode_or_handle_err(output, h)
        } else {
            self.token_id.top_encode_or_handle_err(output, h)
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

impl<M> TypeAbiFrom<DcdtTokenIdentifier<M>> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&DcdtTokenIdentifier<M>> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi
{}
impl<M> TypeAbiFrom<&[u8]> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&str> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> TypeAbiFrom<TestTokenIdentifier<'_>> for RewaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi
{}
impl<M> TypeAbiFrom<&TestTokenIdentifier<'_>> for RewaOrDcdtTokenIdentifier<M> where
    M: ManagedTypeApi
{
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for RewaOrDcdtTokenIdentifier<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for RewaOrDcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> TypeAbi for RewaOrDcdtTokenIdentifier<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "RewaOrDcdtTokenIdentifier".into()
    }

    fn type_name_rust() -> TypeName {
        "RewaOrDcdtTokenIdentifier<$API>".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for RewaOrDcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_rewa() {
            f.append_bytes(Self::REWA_REPRESENTATION);
        } else {
            SCDisplay::fmt(&self.token_id, f)
        }
    }
}

const REWA_REPRESENTATION_HEX: &[u8] = b"52455741";

impl<M: ManagedTypeApi> SCLowerHex for RewaOrDcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_rewa() {
            f.append_bytes(REWA_REPRESENTATION_HEX);
        } else {
            SCLowerHex::fmt(&self.token_id, f)
        }
    }
}

impl<M> core::fmt::Debug for RewaOrDcdtTokenIdentifier<M>
where
    M: ManagedTypeApi,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.map_ref_or_else(
            f,
            |f| f.write_str("RewaOrDcdtTokenIdentifier::Rewa"),
            |f, token_identifier| {
                let token_id_str = token_identifier.to_string();
                f.debug_tuple("RewaOrDcdtTokenIdentifier::Dcdt")
                    .field(&token_id_str)
                    .finish()
            },
        )
    }
}

impl<M: ManagedTypeApi> core::fmt::Display for RewaOrDcdtTokenIdentifier<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.map_ref_or_else(
            f,
            |f| core::fmt::Display::fmt("REWA", f),
            |f, token_identifier| core::fmt::Display::fmt(token_identifier, f),
        )
    }
}
