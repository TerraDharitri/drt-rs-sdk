use alloc::string::ToString;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{ErrorApi, HandleConstraints, ManagedTypeApi},
    codec::*,
    err_msg,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    types::{ManagedBuffer, ManagedRef, ManagedType, TokenId},
};

use super::RewaOrDcdtTokenIdentifier;

/// Legacy name, kept for backward compatibility.
pub type TokenIdentifier<M> = DcdtTokenIdentifier<M>;

/// Specialized type for handling DCDT token identifiers.
///
/// REWA is not allowed, this types should not be instanced for REWA.
#[repr(transparent)]
#[derive(Clone)]
pub struct DcdtTokenIdentifier<M: ErrorApi + ManagedTypeApi> {
    pub(crate) token_id: TokenId<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for DcdtTokenIdentifier<M> {
    type OwnHandle = M::ManagedBufferHandle;

    #[inline]
    unsafe fn from_handle(handle: M::ManagedBufferHandle) -> Self {
        unsafe {
            DcdtTokenIdentifier {
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

impl<M: ManagedTypeApi> DcdtTokenIdentifier<M> {
    /// Creates a new DcdtTokenIdentifier without verifying that it is not REWA-000000.
    ///
    /// ## Safety
    ///
    /// Calling it for the REWA token might lead to unexpected bugs.
    pub unsafe fn dcdt_unchecked(data: RewaOrDcdtTokenIdentifier<M>) -> Self {
        Self {
            token_id: data.into(),
        }
    }

    pub fn try_new(data: RewaOrDcdtTokenIdentifier<M>) -> Option<Self> {
        if data.is_rewa() {
            return None;
        }

        unsafe { Some(Self::dcdt_unchecked(data)) }
    }

    #[inline]
    pub fn from_dcdt_bytes<B: Into<ManagedBuffer<M>>>(bytes: B) -> Self {
        DcdtTokenIdentifier::from(bytes.into())
    }

    #[inline]
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.token_id.into_managed_buffer()
    }

    #[inline]
    pub fn as_managed_buffer(&self) -> &ManagedBuffer<M> {
        self.token_id.as_managed_buffer()
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> crate::types::heap::BoxedBytes {
        self.token_id.to_boxed_bytes()
    }

    pub fn is_valid_dcdt_identifier(&self) -> bool {
        self.token_id.is_valid()
    }

    pub fn ticker(&self) -> ManagedBuffer<M> {
        self.token_id.ticker()
    }
}

impl<M: ManagedTypeApi> From<ManagedBuffer<M>> for DcdtTokenIdentifier<M> {
    #[inline]
    fn from(buffer: ManagedBuffer<M>) -> Self {
        RewaOrDcdtTokenIdentifier::from(buffer).unwrap_dcdt()
    }
}

impl<M: ManagedTypeApi> From<TokenId<M>> for DcdtTokenIdentifier<M> {
    #[inline]
    fn from(token_id: TokenId<M>) -> Self {
        RewaOrDcdtTokenIdentifier::from(token_id).unwrap_dcdt()
    }
}

impl<M: ManagedTypeApi> From<&[u8]> for DcdtTokenIdentifier<M> {
    fn from(bytes: &[u8]) -> Self {
        RewaOrDcdtTokenIdentifier::from(bytes).unwrap_dcdt()
    }
}

impl<M: ManagedTypeApi> From<&str> for DcdtTokenIdentifier<M> {
    fn from(s: &str) -> Self {
        DcdtTokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> From<&crate::types::heap::String> for DcdtTokenIdentifier<M> {
    fn from(s: &crate::types::heap::String) -> Self {
        DcdtTokenIdentifier::from(s.as_bytes())
    }
}

impl<M: ManagedTypeApi> PartialEq for DcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.token_id == other.token_id
    }
}

impl<M: ManagedTypeApi> Eq for DcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<RewaOrDcdtTokenIdentifier<M>> for DcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &RewaOrDcdtTokenIdentifier<M>) -> bool {
        other.map_ref_or_else(
            (),
            |()| false,
            |(), dcdt_token_identifier| dcdt_token_identifier == self,
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for DcdtTokenIdentifier<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.token_id.dep_encode_or_handle_err(dest, h)
    }
}

impl<M: ManagedTypeApi> TopEncode for DcdtTokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.token_id.top_encode_or_handle_err(output, h)
    }
}

impl<M: ManagedTypeApi> NestedDecode for DcdtTokenIdentifier<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let data = RewaOrDcdtTokenIdentifier::dep_decode_or_handle_err(input, h)?;
        if let Some(ti) = DcdtTokenIdentifier::try_new(data) {
            Ok(ti)
        } else {
            Err(h.handle_error(err_msg::TOKEN_IDENTIFIER_DCDT_EXPECTED.into()))
        }
    }
}

impl<M: ManagedTypeApi> TopDecode for DcdtTokenIdentifier<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let data = RewaOrDcdtTokenIdentifier::top_decode_or_handle_err(input, h)?;
        if let Some(ti) = DcdtTokenIdentifier::try_new(data) {
            Ok(ti)
        } else {
            Err(h.handle_error(err_msg::TOKEN_IDENTIFIER_DCDT_EXPECTED.into()))
        }
    }
}

impl<M> TypeAbiFrom<&[u8]> for DcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<Vec<u8>> for DcdtTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for DcdtTokenIdentifier<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for DcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> TypeAbi for DcdtTokenIdentifier<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        // for backwards compatibility with existing tooling
        "TokenIdentifier".into()
    }

    fn type_name_rust() -> TypeName {
        "DcdtTokenIdentifier<$API>".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for DcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let cast_handle = self.get_handle().cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer(&wrap_cast);
    }
}

impl<M: ManagedTypeApi> SCLowerHex for DcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let cast_handle = self.get_handle().cast_or_signal_error::<M, _>();
        let wrap_cast = unsafe { ManagedRef::wrap_handle(cast_handle) };
        f.append_managed_buffer_lower_hex(&wrap_cast);
    }
}

impl<M: ManagedTypeApi> core::fmt::Display for DcdtTokenIdentifier<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.to_boxed_bytes();
        let s = alloc::string::String::from_utf8_lossy(bytes.as_slice());
        s.fmt(f)
    }
}

impl<M: ManagedTypeApi> core::fmt::Debug for DcdtTokenIdentifier<M> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DcdtTokenIdentifier")
            .field(&self.to_string())
            .finish()
    }
}
