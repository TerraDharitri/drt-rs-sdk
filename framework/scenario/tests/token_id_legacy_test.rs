use dharitri_sc::{
    chain_core::REWA_000000_TOKEN_IDENTIFIER,
    types::{
        BoxedBytes, RewaOrDcdtTokenIdentifier, RewaOrDcdtTokenPayment, DcdtTokenIdentifier,
        DcdtTokenPayment, ManagedBuffer,
    },
};
use dharitri_sc_scenario::{
    api::StaticApi, managed_rewa_token_id, managed_test_util::check_managed_top_encode_decode,
    managed_token_id, managed_token_id_wrapped, dharitri_sc,
};

#[test]
fn test_rewa() {
    assert!(RewaOrDcdtTokenIdentifier::<StaticApi>::rewa().is_rewa());
}

#[test]
fn test_codec_top() {
    check_managed_top_encode_decode(
        RewaOrDcdtTokenIdentifier::<StaticApi>::rewa(),
        RewaOrDcdtTokenIdentifier::<StaticApi>::REWA_REPRESENTATION,
    );
}

#[test]
fn test_codec_nested() {
    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &RewaOrDcdtTokenIdentifier::<StaticApi>::REWA_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        vec![RewaOrDcdtTokenIdentifier::<StaticApi>::rewa()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dcdt_identifier() {
    // valid identifier
    assert!(DcdtTokenIdentifier::<StaticApi>::from("ALC-6258d2").is_valid_dcdt_identifier());

    // valid identifier with numbers in ticker
    assert!(DcdtTokenIdentifier::<StaticApi>::from("ALC123-6258d2").is_valid_dcdt_identifier());

    // valid ticker only numbers
    assert!(DcdtTokenIdentifier::<StaticApi>::from("12345-6258d2").is_valid_dcdt_identifier());

    // missing dash
    assert!(!DcdtTokenIdentifier::<StaticApi>::from("ALC6258d2").is_valid_dcdt_identifier());

    // wrong dash position
    assert!(!DcdtTokenIdentifier::<StaticApi>::from("AL-C6258d2").is_valid_dcdt_identifier());

    // lowercase ticker
    assert!(!DcdtTokenIdentifier::<StaticApi>::from("alc-6258d2").is_valid_dcdt_identifier());

    // uppercase random chars
    assert!(!DcdtTokenIdentifier::<StaticApi>::from("ALC-6258D2").is_valid_dcdt_identifier());

    // too many random chars
    assert!(!DcdtTokenIdentifier::<StaticApi>::from("ALC-6258d2ff").is_valid_dcdt_identifier());

    // ticker too short
    assert!(!DcdtTokenIdentifier::<StaticApi>::from("AL-6258d2").is_valid_dcdt_identifier());

    // ticker too long
    assert!(!DcdtTokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258d2").is_valid_dcdt_identifier());
}

#[test]
#[rustfmt::skip]
fn test_ticker() {
    // valid identifier
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("ALC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // valid identifier with numbers in ticker
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("ALC123-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC123"),
    );

    // valid ticker only numbers
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("12345-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("12345"),
    );

    // missing dash
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("ALC6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // wrong dash position
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("AL-C6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL-"),
    );

    // lowercase ticker
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("alc-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("alc"),
    );

    // uppercase random chars
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("ALC-6258D2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // too many random chars
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("ALC-6258d2ff").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC-6"),
    );

    // ticker too short
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("AL-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // ticker too long
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALCCCCCCCCC"),
    );
}

#[test]
fn test_is_valid_rewa_or_dcdt() {
    // rewa is always valid
    assert!(RewaOrDcdtTokenIdentifier::<StaticApi>::rewa().is_valid());

    // valid dcdt
    assert!(
        RewaOrDcdtTokenIdentifier::<StaticApi>::dcdt(DcdtTokenIdentifier::from("ALC-6258d2"))
            .is_valid()
    );

    // invalid dcdt, see above
    assert!(
        !RewaOrDcdtTokenIdentifier::<StaticApi>::dcdt(DcdtTokenIdentifier::from(
            "ALCCCCCCCCC-6258d2"
        ))
        .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from("DCDT-00000"),
        DcdtTokenIdentifier::<StaticApi>::from("DCDT-00000")
    );
    assert_ne!(
        DcdtTokenIdentifier::<StaticApi>::from("DCDT-00001"),
        DcdtTokenIdentifier::<StaticApi>::from("DCDT-00002")
    );

    assert_eq!(
        RewaOrDcdtTokenIdentifier::<StaticApi>::dcdt(DcdtTokenIdentifier::from("DCDT-00003")),
        DcdtTokenIdentifier::<StaticApi>::from("DCDT-00003")
    );
    assert_ne!(
        RewaOrDcdtTokenIdentifier::<StaticApi>::rewa(),
        DcdtTokenIdentifier::<StaticApi>::from("ANYTHING-1234")
    );
}

#[test]
#[should_panic = "StaticApi signal error: DCDT expected"]
pub fn dcdt_token_identifier_unwrap_1() {
    let _ = DcdtTokenIdentifier::<StaticApi>::from("");
}

#[test]
#[should_panic = "StaticApi signal error: DCDT expected"]
pub fn dcdt_token_identifier_unwrap_2() {
    let _ = DcdtTokenIdentifier::<StaticApi>::from("REWA");
}

#[test]
#[should_panic = "StaticApi signal error: DCDT expected"]
pub fn dcdt_token_identifier_unwrap_3() {
    let _ = DcdtTokenIdentifier::<StaticApi>::from(REWA_000000_TOKEN_IDENTIFIER);
}

#[test]
fn test_payment_eq() {
    assert_eq!(
        DcdtTokenPayment::<StaticApi>::new("PAY-00000".into(), 0, 1000u32.into()),
        DcdtTokenPayment::<StaticApi>::new("PAY-00000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        DcdtTokenPayment::<StaticApi>::new("PAY-00001".into(), 0, 1000u32.into()),
        DcdtTokenPayment::<StaticApi>::new("PAY-00002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        RewaOrDcdtTokenPayment::<StaticApi>::no_payment(),
        RewaOrDcdtTokenPayment::<StaticApi>::no_payment(),
    );
    assert_eq!(
        RewaOrDcdtTokenPayment::<StaticApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00000"),
            0,
            1000u32.into()
        ),
        RewaOrDcdtTokenPayment::<StaticApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00000"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        RewaOrDcdtTokenPayment::<StaticApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00001"),
            0,
            1000u32.into()
        ),
        RewaOrDcdtTokenPayment::<StaticApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00002"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        RewaOrDcdtTokenPayment::<StaticApi>::new(
            RewaOrDcdtTokenIdentifier::dcdt("DCDTPAY-00001"),
            0,
            1000u32.into()
        ),
        RewaOrDcdtTokenPayment::<StaticApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    assert_eq!(
        managed_rewa_token_id!(),
        RewaOrDcdtTokenIdentifier::<StaticApi>::rewa()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        DcdtTokenIdentifier::<StaticApi>::from("ALC-6258d2")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_dcdt(),
        DcdtTokenIdentifier::<StaticApi>::from("ALC-6258d2")
    )
}
