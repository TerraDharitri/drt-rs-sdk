use super::AndesBigUint;

use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use alloc::string::String;
use alloc::vec::Vec;

use numbat_wasm::api::{BigIntApi, Sign};

extern "C" {
	fn bigIntNew(value: i64) -> i32;

	fn bigIntSignedByteLength(x: i32) -> i32;
	fn bigIntGetSignedBytes(reference: i32, byte_ptr: *mut u8) -> i32;
	fn bigIntSetSignedBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);

	fn bigIntAdd(dest: i32, x: i32, y: i32);
	fn bigIntSub(dest: i32, x: i32, y: i32);
	fn bigIntMul(dest: i32, x: i32, y: i32);
	fn bigIntTDiv(dest: i32, x: i32, y: i32);
	fn bigIntTMod(dest: i32, x: i32, y: i32);

	fn bigIntAbs(dest: i32, x: i32);
	fn bigIntNeg(dest: i32, x: i32);
	fn bigIntSign(x: i32) -> i32;
	fn bigIntCmp(x: i32, y: i32) -> i32;
}

pub struct AndesBigInt {
	pub handle: i32, // TODO: fix visibility
}

impl From<AndesBigUint> for AndesBigInt {
	#[inline]
	fn from(item: AndesBigUint) -> Self {
		AndesBigInt {
			handle: item.handle,
		}
	}
}

impl From<i64> for AndesBigInt {
	fn from(item: i64) -> Self {
		unsafe {
			AndesBigInt {
				handle: bigIntNew(item),
			}
		}
	}
}

impl From<i32> for AndesBigInt {
	fn from(item: i32) -> Self {
		unsafe {
			AndesBigInt {
				handle: bigIntNew(item.into()),
			}
		}
	}
}

impl AndesBigInt {
	pub fn from_i64(value: i64) -> AndesBigInt {
		unsafe {
			AndesBigInt {
				handle: bigIntNew(value),
			}
		}
	}
}

impl Clone for AndesBigInt {
	fn clone(&self) -> Self {
		unsafe {
			let clone_handle = bigIntNew(0);
			bigIntAdd(clone_handle, clone_handle, self.handle);
			AndesBigInt {
				handle: clone_handle,
			}
		}
	}
}

macro_rules! binary_operator {
	($trait:ident, $method:ident, $api_func:ident) => {
		impl $trait for AndesBigInt {
			type Output = AndesBigInt;

			fn $method(self, other: AndesBigInt) -> AndesBigInt {
				unsafe {
					let result = bigIntNew(0);
					$api_func(result, self.handle, other.handle);
					AndesBigInt { handle: result }
				}
			}
		}

		impl<'a, 'b> $trait<&'b AndesBigInt> for &'a AndesBigInt {
			type Output = AndesBigInt;

			fn $method(self, other: &AndesBigInt) -> AndesBigInt {
				unsafe {
					let result = bigIntNew(0);
					$api_func(result, self.handle, other.handle);
					AndesBigInt { handle: result }
				}
			}
		}
	};
}

binary_operator! {Add, add, bigIntAdd}
binary_operator! {Sub, sub, bigIntSub}
binary_operator! {Mul, mul, bigIntMul}
binary_operator! {Div, div, bigIntTDiv}
binary_operator! {Rem, rem, bigIntTMod}

macro_rules! binary_assign_operator {
	($trait:ident, $method:ident, $api_func:ident) => {
		impl $trait<AndesBigInt> for AndesBigInt {
			#[inline]
			fn $method(&mut self, other: Self) {
				unsafe {
					$api_func(self.handle, self.handle, other.handle);
				}
			}
		}

		impl $trait<&AndesBigInt> for AndesBigInt {
			#[inline]
			fn $method(&mut self, other: &AndesBigInt) {
				unsafe {
					$api_func(self.handle, self.handle, other.handle);
				}
			}
		}
	};
}

binary_assign_operator! {AddAssign, add_assign, bigIntAdd}
binary_assign_operator! {SubAssign, sub_assign, bigIntSub}
binary_assign_operator! {MulAssign, mul_assign, bigIntMul}
binary_assign_operator! {DivAssign, div_assign, bigIntTDiv}
binary_assign_operator! {RemAssign, rem_assign, bigIntTMod}

impl PartialEq for AndesBigInt {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		let andes_cmp = unsafe { bigIntCmp(self.handle, other.handle) };
		andes_cmp == 0
	}
}

impl Eq for AndesBigInt {}

impl PartialOrd for AndesBigInt {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for AndesBigInt {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		let andes_cmp = unsafe { bigIntCmp(self.handle, other.handle) };
		andes_cmp.cmp(&0)
	}
}

fn andes_cmp_i64(bi: &AndesBigInt, other: i64) -> i32 {
	unsafe {
		if other == 0 {
			bigIntSign(bi.handle)
		} else {
			bigIntCmp(bi.handle, bigIntNew(other))
		}
	}
}

impl PartialEq<i64> for AndesBigInt {
	#[inline]
	fn eq(&self, other: &i64) -> bool {
		andes_cmp_i64(&self, *other) == 0
	}
}

impl PartialOrd<i64> for AndesBigInt {
	#[inline]
	fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
		let andes_cmp = andes_cmp_i64(&self, *other);
		Some(andes_cmp.cmp(&0))
	}
}

impl Neg for AndesBigInt {
	type Output = AndesBigInt;

	fn neg(self) -> Self::Output {
		unsafe {
			let result = bigIntNew(0);
			bigIntNeg(result, self.handle);
			AndesBigInt { handle: result }
		}
	}
}

use numbat_wasm::numbat_codec::*;

impl NestedEncode for AndesBigInt {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		// TODO: vector allocation can be avoided by writing directly to dest
		self.to_signed_bytes_be().as_slice().dep_encode(dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.to_signed_bytes_be()
			.as_slice()
			.dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for AndesBigInt {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		output.set_big_int_handle_or_bytes(self.handle, || self.to_signed_bytes_be());
		Ok(())
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		_: ExitCtx,
		_: fn(ExitCtx, EncodeError) -> !,
	) {
		output.set_big_int_handle_or_bytes(self.handle, || self.to_signed_bytes_be());
	}
}

impl NestedDecode for AndesBigInt {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let size = usize::dep_decode(input)?;
		let bytes = input.read_slice(size)?;
		Ok(AndesBigInt::from_signed_bytes_be(bytes))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let size = usize::dep_decode_or_exit(input, c.clone(), exit);
		let bytes = input.read_slice_or_exit(size, c, exit);
		AndesBigInt::from_signed_bytes_be(bytes)
	}
}

impl TopDecode for AndesBigInt {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		// since can_use_handle is provided constantly,
		// the compiler is smart enough to only ever expand one of the if branches
		let (can_use_handle, handle) = input.try_get_big_int_handle();
		if can_use_handle {
			Ok(AndesBigInt { handle })
		} else {
			Ok(AndesBigInt::from_signed_bytes_be(
				&*input.into_boxed_slice_u8(),
			))
		}
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		// since can_use_handle is provided constantly,
		// the compiler is smart enough to only ever expand one of the if branches
		let (can_use_handle, handle) = input.try_get_big_int_handle();
		if can_use_handle {
			AndesBigInt { handle }
		} else {
			AndesBigInt::from_signed_bytes_be(&*input.into_boxed_slice_u8())
		}
	}
}

impl numbat_wasm::abi::TypeAbi for AndesBigInt {
	fn type_name() -> String {
		String::from("BigInt")
	}
}

impl BigIntApi for AndesBigInt {
	type BigUint = AndesBigUint;

	fn abs_uint(&self) -> AndesBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntAbs(result, self.handle);
			AndesBigUint { handle: result }
		}
	}

	fn sign(&self) -> Sign {
		unsafe {
			let s = bigIntSign(self.handle);
			match s.cmp(&0) {
				Ordering::Greater => Sign::Plus,
				Ordering::Equal => Sign::NoSign,
				Ordering::Less => Sign::Minus,
			}
		}
	}

	fn to_signed_bytes_be(&self) -> Vec<u8> {
		unsafe {
			let byte_len = bigIntSignedByteLength(self.handle);
			let mut vec = vec![0u8; byte_len as usize];
			bigIntGetSignedBytes(self.handle, vec.as_mut_ptr());
			vec
		}
	}

	fn from_signed_bytes_be(bytes: &[u8]) -> Self {
		unsafe {
			let handle = bigIntNew(0);
			bigIntSetSignedBytes(handle, bytes.as_ptr(), bytes.len() as i32);
			AndesBigInt { handle }
		}
	}
}
