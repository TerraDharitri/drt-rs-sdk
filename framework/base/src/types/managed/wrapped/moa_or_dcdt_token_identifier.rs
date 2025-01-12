use alloc::string::ToString;
use dharitri_core::MOA_000000_TOKEN_IDENTIFIER;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{
        const_handles, use_raw_handle, ErrorApiImpl, HandleConstraints, ManagedBufferApiImpl,
        ManagedTypeApi,
    },
    codec::*,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    proxy_imports::TestTokenIdentifier,
    types::{ManagedBuffer, ManagedRef, ManagedType, TokenIdentifier},
};

/// Specialized type for handling either MOA or DCDT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// # use dharitri_sc::{api::ManagedTypeApi, types::TokenIdentifier};
/// enum MoaOrDcdtTokenIdentifier<M: ManagedTypeApi> {
///     Moa,
///     Dcdt(TokenIdentifier<M>),
/// }
/// ```
///
/// It is, however more optimized than that. Its implementation is based on `ManagedOption`.
///
/// MOA is indicated by a special, invalid token identifier handle.
/// This way we can fit it inside a single i32 in memory.
#[repr(transparent)]
#[derive(Clone)]
pub struct MoaOrDcdtTokenIdentifier<M: ManagedTypeApi> {
    pub(crate) buffer: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for MoaOrDcdtTokenIdentifier<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        MoaOrDcdtTokenIdentifier {
            buffer: ManagedBuffer::from_handle(handle),
        }
    }

    fn get_handle(&self) -> M::ManagedBufferHandle {
        self.buffer.get_handle()
    }

    unsafe fn forget_into_handle(self) -> Self::OwnHandle {
        self.buffer.forget_into_handle()
    }

    fn transmute_from_handle_ref(handle_ref: &M::ManagedBufferHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }

    fn transmute_from_handle_ref_mut(handle_ref: &mut M::ManagedBufferHandle) -> &mut Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> MoaOrDcdtTokenIdentifier<M> {
    /// This special representation is interpreted as the MOA token.
    pub const MOA_REPRESENTATION: &'static [u8; 3] = b"MOA";

    /// New instance of the special MOA token representation.
    #[inline]
    pub fn moa() -> Self {
        MoaOrDcdtTokenIdentifier {
            buffer: ManagedBuffer::from(MOA_000000_TOKEN_IDENTIFIER),
        }
    }

    /// DCDT instance, containing an DCDT token identifier.
    #[inline]
    pub fn dcdt<TI>(token_identifier: TI) -> Self
    where
        TokenIdentifier<M>: From<TI>,
    {
        let ti_obj = TokenIdentifier::from(token_identifier);
        ti_obj.data
    }

    pub fn parse(data: ManagedBuffer<M>) -> Self {
        if data == Self::MOA_REPRESENTATION {
            Self::moa()
        } else {
            Self { buffer: data }
        }
    }

    #[inline]
    pub fn is_moa(&self) -> bool {
        M::managed_type_impl().mb_overwrite(
            use_raw_handle(const_handles::MBUF_MOA_000000),
            MOA_000000_TOKEN_IDENTIFIER.as_bytes(),
        );
        M::managed_type_impl().mb_eq(
            use_raw_handle(const_handles::MBUF_MOA_000000),
            self.buffer.handle.clone(),
        )
    }

    #[inline]
    pub fn is_dcdt(&self) -> bool {
        !self.is_moa()
    }

    /// Returns "MOA" or the token identifier.
    pub fn into_name(self) -> ManagedBuffer<M> {
        self.map_or_else(
            (),
            |()| ManagedBuffer::from(&Self::MOA_REPRESENTATION[..]),
            |(), token_identifier| token_identifier.into_managed_buffer(),
        )
    }

    /// Checks the DCDT token identifier for validity. MOA is considered valid, no checks needed.
    ///
    /// Will fail if it encodes an invalid DCDT token identifier.
    pub fn is_valid(&self) -> bool {
        self.map_ref_or_else(
            (),
            |()| true,
            |(), token_identifier| token_identifier.is_valid_dcdt_identifier(),
        )
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.buffer
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        &self.buffer
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.buffer.to_boxed_bytes()
    }

    pub fn map_or_else<Context, D, F, R>(self, context: Context, for_moa: D, for_dcdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, TokenIdentifier<M>) -> R,
    {
        if self.is_moa() {
            for_moa(context)
        } else {
            unsafe { for_dcdt(context, TokenIdentifier::dcdt_unchecked(self)) }
        }
    }

    pub fn map_ref_or_else<Context, D, F, R>(&self, context: Context, for_moa: D, for_dcdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, &TokenIdentifier<M>) -> R,
    {
        if self.is_moa() {
            for_moa(context)
        } else {
            unsafe {
                let token_identifier =
                    ManagedRef::<'_, M, TokenIdentifier<M>>::wrap_handle(self.get_handle());
                for_dcdt(context, &token_identifier)
            }
        }
    }

    pub fn unwrap_dcdt(self) -> TokenIdentifier<M> {
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
    pub fn as_dcdt_option(&self) -> Option<ManagedRef<'_, M, TokenIdentifier<M>>> {
        if self.is_moa() {
            None
        } else {
            unsafe {
                Some(ManagedRef::<'_, M, TokenIdentifier<M>>::wrap_handle(
                    self.get_handle(),
                ))
            }
        }
    }

    /// Converts `self` into an `Option`. Consumes `self` in the process.
    pub fn into_dcdt_option(self) -> Option<TokenIdentifier<M>> {
        self.map_or_else((), |()| None, |(), token_identifier| Some(token_identifier))
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        MoaOrDcdtTokenIdentifier { buffer }
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for MoaOrDcdtTokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        MoaOrDcdtTokenIdentifier {
            buffer: ManagedBuffer::new_from_bytes(bytes),
        }
    }
}

impl<M: ManagedTypeApi> PartialEq for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<M: ManagedTypeApi> Eq for MoaOrDcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<TokenIdentifier<M>> for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &TokenIdentifier<M>) -> bool {
        self.map_ref_or_else(
            (),
            |()| false,
            |(), self_dcdt_token_identifier| self_dcdt_token_identifier == other,
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if self.is_moa() {
            (&Self::MOA_REPRESENTATION[..]).dep_encode_or_handle_err(dest, h)
        } else {
            self.buffer.dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if self.is_moa() {
            (&Self::MOA_REPRESENTATION[..]).top_encode_or_handle_err(output, h)
        } else {
            self.buffer.top_encode_or_handle_err(output, h)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for MoaOrDcdtTokenIdentifier<M> {
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

impl<M: ManagedTypeApi> TopDecode for MoaOrDcdtTokenIdentifier<M> {
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

impl<M> TypeAbiFrom<TokenIdentifier<M>> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&TokenIdentifier<M>> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&[u8]> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&str> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> TypeAbiFrom<TestTokenIdentifier<'_>> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi
{}
impl<M> TypeAbiFrom<&TestTokenIdentifier<'_>> for MoaOrDcdtTokenIdentifier<M> where
    M: ManagedTypeApi
{
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for MoaOrDcdtTokenIdentifier<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for MoaOrDcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> TypeAbi for MoaOrDcdtTokenIdentifier<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "MoaOrDcdtTokenIdentifier".into()
    }

    fn type_name_rust() -> TypeName {
        "MoaOrDcdtTokenIdentifier<$API>".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for MoaOrDcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_moa() {
            f.append_bytes(Self::MOA_REPRESENTATION);
        } else {
            let cast_handle = self.buffer.get_handle().cast_or_signal_error::<M, _>();
            let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
            f.append_managed_buffer(&wrap_cast);
        }
    }
}

const MOA_REPRESENTATION_HEX: &[u8] = b"45474C44";

impl<M: ManagedTypeApi> SCLowerHex for MoaOrDcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if self.is_moa() {
            f.append_bytes(MOA_REPRESENTATION_HEX);
        } else {
            let cast_handle = self.buffer.get_handle().cast_or_signal_error::<M, _>();
            let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
            f.append_managed_buffer_lower_hex(&wrap_cast);
        }
    }
}

impl<M> core::fmt::Debug for MoaOrDcdtTokenIdentifier<M>
where
    M: ManagedTypeApi,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.map_ref_or_else(
            f,
            |f| f.write_str("MoaOrDcdtTokenIdentifier::Moa"),
            |f, token_identifier| {
                let token_id_str = token_identifier.to_string();
                f.debug_tuple("MoaOrDcdtTokenIdentifier::Dcdt")
                    .field(&token_id_str)
                    .finish()
            },
        )
    }
}
