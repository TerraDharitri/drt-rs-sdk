use dharitri_sc::types::{ManagedVecItemPayload, ManagedVecItemPayloadBuffer};

dharitri_sc::derive_imports!();

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_struct_2_test > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug,
)]
pub struct Struct2 {
    pub u_8: u8,
    pub u_16: u16,
    pub u_32: u32,
    pub u_64: u64,
    pub bool_field: bool,
    pub opt_field: Option<u8>,
    pub arr: u32,
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn struct_2_static() {
    assert_eq!(
        <Struct2 as dharitri_sc::types::ManagedVecItem>::payload_size(),
        22
    );
    assert!(!<Struct2 as dharitri_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION);
}

#[test]
fn struct_to_bytes_writer() {
    let s = Struct2 {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
        bool_field: true,
        opt_field: Some(5),
        arr: 0x61116222,
    };

    #[rustfmt::skip]
    let expected_payload = [
        /* u_8  */ 0x01,
        /* u_16 */ 0x00, 0x02,
        /* u_32 */ 0x00, 0x00, 0x00, 0x03,
        /* u_64 */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
        /* bool */ 0x01,
        /* opt  */ 0x01, 0x05,
        /* arr  */ 0x61, 0x11, 0x62, 0x22,
    ];

    let mut payload = ManagedVecItemPayloadBuffer::new_buffer();
    <Struct2 as dharitri_sc::types::ManagedVecItem>::save_to_payload(s, &mut payload);
    assert_eq!(payload.into_array(), expected_payload);
}

#[test]
fn struct_2_from_bytes_reader() {
    let expected_struct = Struct2 {
        u_8: 1u8,
        u_16: 2u16,
        u_32: 3u32,
        u_64: 4u64,
        bool_field: false,
        opt_field: Some(5),
        arr: 0x61116222,
    };

    #[rustfmt::skip]
    let payload = [
        /* u_8  */ 0x01,
        /* u_16 */ 0x00, 0x02,
        /* u_32 */ 0x00, 0x00, 0x00, 0x03,
        /* u_64 */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
        /* bool */ 0x00,
        /* opt  */ 0x01, 0x05,
        /* arr  */ 0x61, 0x11, 0x62, 0x22,
    ];

    let struct_from_bytes =
        <Struct2 as dharitri_sc::types::ManagedVecItem>::read_from_payload(&payload.into());
    assert_eq!(expected_struct, struct_from_bytes);
}
