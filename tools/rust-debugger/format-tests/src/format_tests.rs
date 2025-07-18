use dharitri_sc_scenario::imports::*;

macro_rules! push {
    ($list: ident, $name:ident, $expected: expr ) => {{
        // Ensure the identifier is a valid variable name
        let _ = $name;
        $list.push((stringify!($name).to_owned(), $expected.to_owned()));
    }};
}

#[allow(unused_variables)]
// Allow redundant_clone since the variables have to be available at the breakpoint location
// they have to be cloned if used before that point
#[allow(clippy::redundant_clone)]
fn main() {
    DebugApi::dummy();

    // Used by the python script which checks the variable summaries
    let mut to_check: Vec<(String, String)> = Vec::new();

    let num_biguint: RustBigUint = 10u32.into();
    push!(to_check, num_biguint, "10");

    let num_bigint_small: RustBigInt = RustBigInt::from(-10);
    push!(to_check, num_bigint_small, "-10");

    let num_bigint_large: RustBigInt = RustBigInt::from(10).pow(30);
    push!(
        to_check,
        num_bigint_large,
        "1000000000000000000000000000000"
    );

    let num_bigint_negative: RustBigInt = RustBigInt::from(10).pow(30) * -1;
    push!(
        to_check,
        num_bigint_negative,
        "-1000000000000000000000000000000"
    );

    let biguint: BigUint<DebugApi> = num_bigint_large.to_biguint().unwrap().into();
    push!(to_check, biguint, "1000000000000000000000000000000");

    let bigint: BigInt<DebugApi> = num_bigint_negative.clone().into();
    push!(to_check, bigint, "-1000000000000000000000000000000");

    let bigfloat: BigFloat<DebugApi> = BigFloat::from_frac(-12345678, 10000);
    push!(to_check, bigfloat, "-1234.5678");

    let managed_buffer: ManagedBuffer<DebugApi> = ManagedBuffer::new_from_bytes(b"hello world");
    push!(
        to_check,
        managed_buffer,
        "\"hello world\" - (11) 0x68656c6c6f20776f726c64"
    );

    let test_sc_address: TestSCAddress = TestSCAddress::new("multi-transfer");
    push!(to_check, test_sc_address, "\"sc:multi-transfer\"");

    let test_address: TestAddress = TestAddress::new("owner-test");
    push!(to_check, test_address, "\"address:owner-test\"");

    let hex_dcdt_safe: [u8; 32] =
        hex::decode(b"00000000000000000500646364742d736166655f5f5f5f5f5f5f5f5f5f5f5f5f")
            .unwrap_or_else(|_| panic!("Unable to decode hexadecimal address"))
            .try_into()
            .unwrap_or_else(|address: Vec<u8>| {
                panic!(
                    "Invalid length: expected 32 bytes but got {}",
                    address.len()
                )
            });
    let hex_dcdt_safe_address = Address::new(hex_dcdt_safe);
    let dcdt_safe_managed_address: ManagedAddress<DebugApi> =
        ManagedAddress::from(hex_dcdt_safe_address);
    push!(to_check, dcdt_safe_managed_address, "\"dcdt-safe_____________\" - (32) 0x00000000000000000500646364742d736166655f5f5f5f5f5f5f5f5f5f5f5f5f");

    let test_token_identifier: TestTokenIdentifier = TestTokenIdentifier::new("TEST-123456");
    push!(to_check, test_token_identifier, "\"str:TEST-123456\"");

    let token_identifier: TokenIdentifier<DebugApi> = TokenIdentifier::from("MYTOK-123456");
    push!(to_check, token_identifier, "\"MYTOK-123456\"");

    let managed_address = DCDTSystemSCAddress.to_managed_address::<DebugApi>();
    push!(
        to_check,
        managed_address,
        "(32) 0x233300000000000000000000000000000002333000000000000000000002ffff"
    );

    let managed_byte_array: ManagedByteArray<DebugApi, 4> =
        ManagedByteArray::new_from_bytes(b"test");
    push!(to_check, managed_byte_array, "\"test\" - (4) 0x74657374");

    let managed_option_some_token_identifier: ManagedOption<DebugApi, TokenIdentifier<DebugApi>> =
        ManagedOption::some(token_identifier.clone());
    push!(
        to_check,
        managed_option_some_token_identifier,
        "ManagedOption::some(\"MYTOK-123456\")"
    );

    let managed_option_none: ManagedOption<DebugApi, TokenIdentifier<DebugApi>> =
        ManagedOption::none();
    push!(to_check, managed_option_none, "ManagedOption::none()");

    let payment = DcdtTokenPayment {
        token_identifier: TokenIdentifier::from("MYTOK-123456"),
        token_nonce: 42,
        amount: BigUint::from(1000u64),
    };
    push!(
        to_check,
        payment,
        "{ token_identifier: \"MYTOK-123456\", nonce: 42, amount: 1000 }"
    );

    let mut managed_vec_of_biguints: ManagedVec<DebugApi, BigUint<DebugApi>> = ManagedVec::new();
    managed_vec_of_biguints.push(BigUint::from(10u64).pow(10));
    managed_vec_of_biguints.push(BigUint::from(10u64).pow(20));
    push!(
        to_check,
        managed_vec_of_biguints,
        "(2) { [0] = 10000000000, [1] = 100000000000000000000 }"
    );

    let mut managed_vec_of_payments: ManagedVec<DebugApi, DcdtTokenPayment<DebugApi>> =
        ManagedVec::new();
    managed_vec_of_payments.push(payment.clone());
    managed_vec_of_payments.push(DcdtTokenPayment::new(
        TokenIdentifier::from("MYTOK-abcdef"),
        100,
        5000u64.into(),
    ));
    push!(to_check, managed_vec_of_payments, "(2) { [0] = { token_identifier: \"MYTOK-123456\", nonce: 42, amount: 1000 }, [1] = { token_identifier: \"MYTOK-abcdef\", nonce: 100, amount: 5000 } }");

    let rewa_or_dcdt_token_identifier_rewa: RewaOrDcdtTokenIdentifier<DebugApi> =
        RewaOrDcdtTokenIdentifier::rewa();
    push!(
        to_check,
        rewa_or_dcdt_token_identifier_rewa,
        "RewaOrDcdtTokenIdentifier::rewa()"
    );

    let rewa_or_dcdt_token_identifier_dcdt: RewaOrDcdtTokenIdentifier<DebugApi> =
        RewaOrDcdtTokenIdentifier::dcdt("MYTOK-123456");
    push!(
        to_check,
        rewa_or_dcdt_token_identifier_dcdt,
        "RewaOrDcdtTokenIdentifier::dcdt(\"MYTOK-123456\")"
    );

    // Nested type tests
    let mut managed_vec_of_addresses: ManagedVec<DebugApi, ManagedAddress<DebugApi>> =
        ManagedVec::new();
    managed_vec_of_addresses.push(managed_address.clone());
    push!(
        to_check,
        managed_vec_of_addresses,
        "(1) { [0] = (32) 0x233300000000000000000000000000000002333000000000000000000002ffff }"
    );

    let managed_option_of_vec_of_addresses: ManagedOption<
        DebugApi,
        ManagedVec<DebugApi, ManagedAddress<DebugApi>>,
    > = ManagedOption::some(managed_vec_of_addresses.clone());
    push!(to_check, managed_option_of_vec_of_addresses, "ManagedOption::some((1) { [0] = (32) 0x233300000000000000000000000000000002333000000000000000000002ffff })");

    // 5. SC wasm - heap
    let heap_address: Address = managed_address.to_address();
    push!(
        to_check,
        heap_address,
        "(32) 0x233300000000000000000000000000000002333000000000000000000002ffff"
    );

    let boxed_bytes: BoxedBytes = b"test"[..].into();
    push!(to_check, boxed_bytes, "(4) 0x74657374");

    let mut managed_vec_of_managed_buffers: ManagedVec<DebugApi, ManagedBuffer<DebugApi>> =
        ManagedVec::new();
    for value in ["ab", "abcd", "abcdefghijkl"] {
        managed_vec_of_managed_buffers.push(value.into());
    }
    push!(
        to_check,
        managed_vec_of_managed_buffers,
        "(3) { [0] = \"ab\" - (2) 0x6162, [1] = \"abcd\" - (4) 0x61626364, [2] = \"abcdefghijkl\" - (12) 0x6162636465666768696a6b6c }"
    );

    // 6. Dharitri codec - Multi-types
    let optional_value_some: OptionalValue<BigUint<DebugApi>> =
        OptionalValue::Some(BigUint::from(42u64));
    push!(to_check, optional_value_some, "OptionalValue::Some(42)");

    let optional_value_none: OptionalValue<BigUint<DebugApi>> = OptionalValue::None;
    push!(to_check, optional_value_none, "OptionalValue::None");

    // Invalid handle tests

    let invalid_handle = DebugHandle::from(-1000);
    let biguint_with_invalid_handle: BigUint<DebugApi> =
        unsafe { BigUint::from_handle(invalid_handle.clone()) };
    push!(
        to_check,
        biguint_with_invalid_handle,
        "<invalid handle: raw_handle -1000 not found in big_int_map>"
    );

    let big_float_with_invalid_handle: BigFloat<DebugApi> =
        unsafe { BigFloat::from_handle(invalid_handle.clone()) };
    push!(
        to_check,
        big_float_with_invalid_handle,
        "<invalid handle: raw_handle -1000 not found in big_float_map>"
    );

    let managed_buffer_with_invalid_handle: ManagedBuffer<DebugApi> =
        unsafe { ManagedBuffer::from_handle(invalid_handle.clone()) };
    push!(
        to_check,
        managed_buffer_with_invalid_handle,
        "<invalid handle: raw_handle -1000 not found in managed_buffer_map>"
    );

    let token_identifier_with_invalid_handle: TokenIdentifier<DebugApi> =
        unsafe { TokenIdentifier::from_handle(invalid_handle.clone()) };
    push!(
        to_check,
        token_identifier_with_invalid_handle,
        "<invalid handle: raw_handle -1000 not found in managed_buffer_map>"
    );

    let optional_value_some_with_invalid_handle: OptionalValue<BigUint<DebugApi>> =
        OptionalValue::Some(unsafe { BigUint::from_handle(invalid_handle.clone()) });
    push!(
        to_check,
        optional_value_some_with_invalid_handle,
        "OptionalValue::Some(<invalid handle: raw_handle -1000 not found in big_int_map>)"
    );

    breakpoint_marker_end_of_main();
}

fn breakpoint_marker_end_of_main() {}
