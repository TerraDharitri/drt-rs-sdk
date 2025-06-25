#![feature(generic_associated_types)]

use numbat_wasm::{
    api::ManagedTypeApi,
    derive::ManagedVecItem,
    numbat_codec,
    numbat_codec::numbat_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::BigUint,
};
use numbat_wasm_debug::DebugApi;

// to test, run the following command in numbat-wasm-debug folder:
// cargo expand --test derive_managed_vec_item_biguint_test > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug,
)]
pub struct ManagedStructWithBigUint<M: ManagedTypeApi> {
    pub big_uint: numbat_wasm::types::BigUint<M>,
    pub num: u32,
}

#[test]
fn struct_with_numbers_static() {
    assert_eq!(
        <ManagedStructWithBigUint<DebugApi> as numbat_wasm::types::ManagedVecItem>::PAYLOAD_SIZE,
        8
    );
    assert!(
        !<ManagedStructWithBigUint<DebugApi> as numbat_wasm::types::ManagedVecItem>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn managed_struct_to_bytes_writer() {
    let _ = DebugApi::dummy();
    let fortytwo = 42u64;
    let s = ManagedStructWithBigUint::<DebugApi> {
        big_uint: BigUint::from(fortytwo),
        num: 0x12345,
    };
    let mut arr: [u8; 8] = [0u8;
        <ManagedStructWithBigUint<DebugApi> as numbat_wasm::types::ManagedVecItem>::PAYLOAD_SIZE];

    <ManagedStructWithBigUint<DebugApi> as numbat_wasm::types::ManagedVecItem>::to_byte_writer(
        &s,
        |bytes| {
            arr[0..<ManagedStructWithBigUint::<DebugApi> as numbat_wasm::types::ManagedVecItem>::PAYLOAD_SIZE].copy_from_slice(bytes);

            assert_eq!(arr, [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x23, 0x45]);
        },
    );
}

#[test]
fn managed_struct_from_bytes_reader() {
    let _ = DebugApi::dummy();
    let s = ManagedStructWithBigUint::<DebugApi> {
        big_uint: BigUint::from(42u64),
        num: 0x12345,
    };
    let arr: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x23, 0x45];

    let struct_from_bytes = <ManagedStructWithBigUint<DebugApi> as numbat_wasm::types::ManagedVecItem>::from_byte_reader( |bytes| {
        bytes.copy_from_slice(
                    &arr
                        [0
                            ..<ManagedStructWithBigUint::<DebugApi> as numbat_wasm::types::ManagedVecItem>::PAYLOAD_SIZE],
                );
    });
    assert_eq!(s, struct_from_bytes);
}
