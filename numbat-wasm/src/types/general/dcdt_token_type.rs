use crate::abi::TypeAbi;
use alloc::string::String;
use numbat_codec::*;

const DCDT_TYPE_FUNGIBLE: &[u8] = b"FungibleDCDT";
const DCDT_TYPE_NON_FUNGIBLE: &[u8] = b"NonFungibleDCDT";
const DCDT_TYPE_SEMI_FUNGIBLE: &[u8] = b"SemiFungibleDCDT";
const DCDT_TYPE_INVALID: &[u8] = &[];

// Note: In the current implementation, SemiFungible is never returned
#[derive(Clone, PartialEq, Debug)]
pub enum DcdtTokenType {
	Fungible,
	NonFungible,
	SemiFungible,
	Invalid,
}

impl DcdtTokenType {
	pub fn as_u8(&self) -> u8 {
		match self {
			Self::Fungible => 0,
			Self::NonFungible => 1,
			Self::SemiFungible => 2,
			Self::Invalid => 3,
		}
	}

	pub fn as_type_name(&self) -> &'static [u8] {
		match self {
			Self::Fungible => DCDT_TYPE_FUNGIBLE,
			Self::NonFungible => DCDT_TYPE_NON_FUNGIBLE,
			Self::SemiFungible => DCDT_TYPE_SEMI_FUNGIBLE,
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
		} else {
			Self::Invalid
		}
	}
}

impl NestedEncode for DcdtTokenType {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_u8().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_u8().dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for DcdtTokenType {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_u8().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_u8().top_encode_or_exit(output, c, exit);
	}
}

impl NestedDecode for DcdtTokenType {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Self::from(u8::dep_decode(input)?))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Self::from(u8::dep_decode_or_exit(input, c, exit))
	}
}

impl TopDecode for DcdtTokenType {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(Self::from(u8::top_decode(input)?))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Self::from(u8::top_decode_or_exit(input, c, exit))
	}
}

impl TypeAbi for DcdtTokenType {
	fn type_name() -> String {
		"DcdtTokenType".into()
	}
}
