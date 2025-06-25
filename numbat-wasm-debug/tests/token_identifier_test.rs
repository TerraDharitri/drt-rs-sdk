use numbat_wasm::types::{
    BoxedBytes, RewaOrDcdtTokenIdentifier, RewaOrDcdtTokenPayment, DcdtTokenPayment,
    TokenIdentifier,
};
use numbat_wasm_debug::{
    check_managed_top_encode_decode, managed_rewa_token_id, managed_token_id,
    managed_token_id_wrapped, DebugApi,
};

#[test]
fn test_rewa() {
    let _ = DebugApi::dummy();
    assert!(RewaOrDcdtTokenIdentifier::<DebugApi>::rewa().is_rewa());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        RewaOrDcdtTokenIdentifier::<DebugApi>::rewa(),
        RewaOrDcdtTokenIdentifier::<DebugApi>::REWA_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &RewaOrDcdtTokenIdentifier::<DebugApi>::REWA_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api,
        vec![RewaOrDcdtTokenIdentifier::<DebugApi>::rewa()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dcdt_identifier() {
    let _ = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::<DebugApi>::from("ALC-6258d2").is_valid_dcdt_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<DebugApi>::from("ALC123-6258d2").is_valid_dcdt_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<DebugApi>::from("12345-6258d2").is_valid_dcdt_identifier());

    // missing dash
    assert!(!TokenIdentifier::<DebugApi>::from("ALC6258d2").is_valid_dcdt_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<DebugApi>::from("AL-C6258d2").is_valid_dcdt_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<DebugApi>::from("alc-6258d2").is_valid_dcdt_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<DebugApi>::from("ALC-6258D2").is_valid_dcdt_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<DebugApi>::from("ALC-6258d2ff").is_valid_dcdt_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<DebugApi>::from("AL-6258d2").is_valid_dcdt_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<DebugApi>::from("ALCCCCCCCCC-6258d2").is_valid_dcdt_identifier());
}

#[test]
fn test_is_valid_rewa_or_dcdt() {
    let _ = DebugApi::dummy();

    // rewa is always valid
    assert!(RewaOrDcdtTokenIdentifier::<DebugApi>::rewa().is_valid());

    // valid dcdt
    assert!(
        RewaOrDcdtTokenIdentifier::<DebugApi>::dcdt(TokenIdentifier::from("ALC-6258d2")).is_valid()
    );

    // invalid dcdt, see above
    assert!(
        !RewaOrDcdtTokenIdentifier::<DebugApi>::dcdt(TokenIdentifier::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    let _ = DebugApi::dummy();
    assert_eq!(
        TokenIdentifier::<DebugApi>::from("DCDT-00000"),
        TokenIdentifier::<DebugApi>::from("DCDT-00000")
    );
    assert_ne!(
        TokenIdentifier::<DebugApi>::from("DCDT-00001"),
        TokenIdentifier::<DebugApi>::from("DCDT-00002")
    );

    assert_eq!(
        RewaOrDcdtTokenIdentifier::<DebugApi>::dcdt(TokenIdentifier::from("DCDT-00003")),
        TokenIdentifier::<DebugApi>::from("DCDT-00003")
    );
    assert_ne!(
        RewaOrDcdtTokenIdentifier::<DebugApi>::rewa(),
        TokenIdentifier::<DebugApi>::from("ANYTHING-1234")
    );
    assert_ne!(
        RewaOrDcdtTokenIdentifier::<DebugApi>::rewa(),
        TokenIdentifier::<DebugApi>::from("REWA")
    );
}

#[test]
fn test_payment_eq() {
    let _ = DebugApi::dummy();
    assert_eq!(
        DcdtTokenPayment::<DebugApi>::new("PAY-00000".into(), 0, 1000u32.into()),
        DcdtTokenPayment::<DebugApi>::new("PAY-00000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        DcdtTokenPayment::<DebugApi>::new("PAY-00001".into(), 0, 1000u32.into()),
        DcdtTokenPayment::<DebugApi>::new("PAY-00002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        RewaOrDcdtTokenPayment::<DebugApi>::no_payment(),
        RewaOrDcdtTokenPayment::<DebugApi>::no_payment(),
    );
    assert_eq!(
        RewaOrDcdtTokenPayment::<DebugApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00000"),
            0,
            1000u32.into()
        ),
        RewaOrDcdtTokenPayment::<DebugApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00000"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        RewaOrDcdtTokenPayment::<DebugApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00001"),
            0,
            1000u32.into()
        ),
        RewaOrDcdtTokenPayment::<DebugApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00002"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        RewaOrDcdtTokenPayment::<DebugApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00001"),
            0,
            1000u32.into()
        ),
        RewaOrDcdtTokenPayment::<DebugApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    let _ = DebugApi::dummy();
    assert_eq!(
        managed_rewa_token_id!(),
        RewaOrDcdtTokenIdentifier::<DebugApi>::rewa()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        TokenIdentifier::<DebugApi>::from("ALC-6258d2")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_dcdt(),
        TokenIdentifier::<DebugApi>::from("ALC-6258d2")
    )
}
