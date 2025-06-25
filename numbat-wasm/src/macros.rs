// Note: Simple macros cannot be placed in numbat-wasm-derive,
// because Rust "cannot export macro_rules! macros from a `proc-macro` crate type currently".

/// Getting all imports needed for a smart contract.
#[macro_export]
macro_rules! imports {
	() => {
		use core::ops::{Add, Div, Mul, Rem, Sub};
		use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
		use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
		use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
		use numbat_wasm::api::{
			BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
			ProxyObjApi, SendApi,
		};
		use numbat_wasm::api::{ErrorApi, LogApi}; // TODO: remove at some point, they shouldn't be public
		use numbat_wasm::numbat_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
		use numbat_wasm::err_msg;
		use numbat_wasm::dcdt::*;
		use numbat_wasm::io::*;
		use numbat_wasm::non_zero_util::*;
		use numbat_wasm::storage::mappers::*;
		use numbat_wasm::types::*;
		use numbat_wasm::types::{SCResult::Err, SCResult::Ok};
		use numbat_wasm::{non_zero_usize, only_owner, require, sc_error};
		use numbat_wasm::{Box, Vec};
	};
}

/// Imports required for deriving serialization and TypeAbi.
#[macro_export]
macro_rules! derive_imports {
	() => {
		use numbat_wasm::numbat_codec;
		use numbat_wasm::numbat_codec::numbat_codec_derive::{
			NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
			TopEncodeOrDefault,
		};
		use numbat_wasm_derive::TypeAbi;
	};
}

/// Compact way of returning a static error message.
#[macro_export]
macro_rules! sc_error {
	($s:expr) => {
		numbat_wasm::types::SCResult::Err(numbat_wasm::types::SCError::from($s.as_bytes())).into()
	};
}

/// Equivalent to the `?` operator for SCResult.
#[deprecated(
	since = "0.16.0",
	note = "The `?` operator can now be used on `SCResult`, please use it instead."
)]
#[macro_export]
macro_rules! sc_try {
	($s:expr) => {
		match $s {
			numbat_wasm::types::SCResult::Ok(t) => t,
			numbat_wasm::types::SCResult::Err(e) => {
				return numbat_wasm::types::SCResult::Err(e);
			},
		}
	};
}

/// Allows us to write Solidity style `require!(<condition>, <error_msg>)` and avoid if statements.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use numbat_wasm::*;
/// # use numbat_wasm::api::BlockchainApi;
/// # use numbat_wasm::types::{*, SCResult::Ok};
/// # pub trait ExampleContract<BigInt, BigUint>: numbat_wasm::api::ContractBase
/// # where
/// #   BigInt: numbat_wasm::api::BigIntApi + 'static,
/// #   BigUint: numbat_wasm::api::BigUintApi + 'static,
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     require!(self.blockchain().get_caller() == self.blockchain().get_owner_address(), "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! require {
	($expression:expr, $error_msg:expr) => {
		if (!($expression)) {
			return sc_error!($error_msg);
		}
	};
}

/// Very compact way of not allowing anyone but the owner to call a function.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use numbat_wasm::*;
/// # use numbat_wasm::api::BlockchainApi;
/// # use numbat_wasm::types::{*, SCResult::Ok};
/// # pub trait ExampleContract<BigInt, BigUint>: numbat_wasm::api::ContractBase
/// # where
/// #   BigInt: numbat_wasm::api::BigIntApi + 'static,
/// #   BigUint: numbat_wasm::api::BigUintApi + 'static,
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     only_owner!(self, "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! only_owner {
	($trait_self: expr, $error_msg:expr) => {
		if ($trait_self.blockchain().get_caller() != $trait_self.blockchain().get_owner_address()) {
			return sc_error!($error_msg);
		}
	};
}

/// Converts usize to NonZeroUsize or returns SCError.
#[macro_export]
macro_rules! non_zero_usize {
	($input: expr, $error_msg:expr) => {
		if let Some(nz) = NonZeroUsize::new($input) {
			nz
		} else {
			return sc_error!($error_msg);
		}
	};
}
