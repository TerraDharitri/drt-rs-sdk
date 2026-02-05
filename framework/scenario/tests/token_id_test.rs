use dharitri_sc::{
    chain_core::REWA_000000_TOKEN_IDENTIFIER,
    types::{
        BoxedBytes, RewaOrDcdtTokenIdentifier, RewaOrDcdtTokenPayment, DcdtTokenIdentifier,
        DcdtTokenPayment, ManagedBuffer, TokenId,
    },
};
use dharitri_sc_scenario::{
    api::StaticApi,
    managed_test_util::{check_managed_top_decode, check_managed_top_encode_decode},
    dharitri_sc, token_id,
};

#[test]
fn test_rewa() {
    assert!(RewaOrDcdtTokenIdentifier::<StaticApi>::rewa().is_rewa());
    assert!(TokenId::<StaticApi>::native().is_native());
    assert!(TokenId::<StaticApi>::native().as_legacy().is_rewa());
}

#[test]
fn test_codec_top() {
    check_managed_top_encode_decode(
        TokenId::<StaticApi>::from(REWA_000000_TOKEN_IDENTIFIER),
        REWA_000000_TOKEN_IDENTIFIER.as_bytes(),
    );
}

#[test]
fn test_decode_top_backwards_compatibility() {
    for rewa_token_id_str in ["", "REWA", REWA_000000_TOKEN_IDENTIFIER] {
        let deserialized: TokenId<StaticApi> =
            check_managed_top_decode(rewa_token_id_str.as_bytes());
        assert!(deserialized.is_native());
        assert!(deserialized.as_legacy().is_rewa());
    }
}

#[test]
fn test_codec_nested() {
    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, REWA_000000_TOKEN_IDENTIFIER.len() as u8],
        REWA_000000_TOKEN_IDENTIFIER.as_bytes(),
    ]);
    check_managed_top_encode_decode(
        vec![TokenId::<StaticApi>::from(REWA_000000_TOKEN_IDENTIFIER)],
        expected.as_slice(),
    );
}

#[test]
fn test_decode_nested_backwards_compatibility() {
    for rewa_token_id_str in ["", "REWA", REWA_000000_TOKEN_IDENTIFIER] {
        let encoded_vec = BoxedBytes::from_concat(&[
            &[0, 0, 0, rewa_token_id_str.len() as u8],
            rewa_token_id_str.as_bytes(),
        ]);
        let deserialized: Vec<TokenId<StaticApi>> =
            check_managed_top_decode(encoded_vec.as_slice());
        assert_eq!(deserialized.len(), 1);
        assert!(deserialized[0].is_native());
        assert!(deserialized[0].as_legacy().is_rewa());
    }
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dcdt_identifier() {
    // valid identifier
    assert!(TokenId::<StaticApi>::from("ALC-6258d2").is_valid_dcdt_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenId::<StaticApi>::from("ALC123-6258d2").is_valid_dcdt_identifier());

    // valid ticker only numbers
    assert!(TokenId::<StaticApi>::from("12345-6258d2").is_valid_dcdt_identifier());

    // missing dash
    assert!(!TokenId::<StaticApi>::from("ALC6258d2").is_valid_dcdt_identifier());

    // wrong dash position
    assert!(!TokenId::<StaticApi>::from("AL-C6258d2").is_valid_dcdt_identifier());

    // lowercase ticker
    assert!(!TokenId::<StaticApi>::from("alc-6258d2").is_valid_dcdt_identifier());

    // uppercase random chars
    assert!(!TokenId::<StaticApi>::from("ALC-6258D2").is_valid_dcdt_identifier());

    // too many random chars
    assert!(!TokenId::<StaticApi>::from("ALC-6258d2ff").is_valid_dcdt_identifier());

    // ticker too short
    assert!(!TokenId::<StaticApi>::from("AL-6258d2").is_valid_dcdt_identifier());

    // ticker too long
    assert!(!TokenId::<StaticApi>::from("ALCCCCCCCCC-6258d2").is_valid_dcdt_identifier());
}

#[test]
#[rustfmt::skip]
fn test_ticker() {
    // valid identifier
    assert_eq!(
        TokenId::<StaticApi>::from("ALC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // valid identifier with numbers in ticker
    assert_eq!(
        TokenId::<StaticApi>::from("ALC123-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC123"),
    );

    // valid ticker only numbers
    assert_eq!(
        TokenId::<StaticApi>::from("12345-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("12345"),
    );

    // missing dash
    assert_eq!(
        TokenId::<StaticApi>::from("ALC6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // wrong dash position
    assert_eq!(
        TokenId::<StaticApi>::from("AL-C6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL-"),
    );

    // lowercase ticker
    assert_eq!(
        TokenId::<StaticApi>::from("alc-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("alc"),
    );

    // uppercase random chars
    assert_eq!(
        TokenId::<StaticApi>::from("ALC-6258D2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // too many random chars
    assert_eq!(
        TokenId::<StaticApi>::from("ALC-6258d2ff").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC-6"),
    );

    // ticker too short
    assert_eq!(
        TokenId::<StaticApi>::from("AL-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // ticker too long
    assert_eq!(
        TokenId::<StaticApi>::from("ALCCCCCCCCC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALCCCCCCCCC"),
    );
}

#[test]
fn test_is_valid_rewa_or_dcdt() {
    // rewa is always valid
    assert!(RewaOrDcdtTokenIdentifier::<StaticApi>::rewa().is_valid());

    // valid dcdt
    assert!(RewaOrDcdtTokenIdentifier::<StaticApi>::dcdt(TokenId::from("ALC-6258d2")).is_valid());

    // invalid dcdt, see above
    assert!(
        !RewaOrDcdtTokenIdentifier::<StaticApi>::dcdt(TokenId::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_id_backwards_compatibility() {
    for rewa_token_id_str in ["", "REWA", "REWA-000000"] {
        let token_id =
            TokenId::<StaticApi>::new(ManagedBuffer::<StaticApi>::from(rewa_token_id_str));
        assert!(token_id.is_native());
        assert!(token_id.as_legacy().is_rewa());
        let token_id = TokenId::<StaticApi>::new_backwards_compatible(
            ManagedBuffer::<StaticApi>::from(rewa_token_id_str),
        );
        assert!(token_id.is_native());
        assert!(token_id.as_legacy().is_rewa());
        let token_id =
            TokenId::<StaticApi>::from(ManagedBuffer::<StaticApi>::from(rewa_token_id_str));
        assert!(token_id.is_native());
        assert!(token_id.as_legacy().is_rewa());
        let token_id = TokenId::<StaticApi>::from(rewa_token_id_str);
        assert!(token_id.is_native());
        assert!(token_id.as_legacy().is_rewa());
    }
}

#[test]
fn test_rewa_or_dcdt_token_identifier_backwards_compatibility() {
    for rewa_token_id_str in ["", "REWA", "REWA-000000"] {
        let old_token_id = RewaOrDcdtTokenIdentifier::<StaticApi>::parse(
            ManagedBuffer::<StaticApi>::from(rewa_token_id_str),
        );
        assert!(old_token_id.is_rewa());
        let old_token_id = RewaOrDcdtTokenIdentifier::<StaticApi>::from(
            ManagedBuffer::<StaticApi>::from(rewa_token_id_str),
        );
        assert!(old_token_id.is_rewa());
        let old_token_id = RewaOrDcdtTokenIdentifier::<StaticApi>::from(rewa_token_id_str);
        assert!(old_token_id.is_rewa());
    }
}

#[test]
fn test_token_id_eq() {
    assert_eq!(
        TokenId::<StaticApi>::from("DCDT-00000"),
        TokenId::<StaticApi>::from("DCDT-00000")
    );
    assert_ne!(
        TokenId::<StaticApi>::from("DCDT-00001"),
        TokenId::<StaticApi>::from("DCDT-00002")
    );

    assert_eq!(
        RewaOrDcdtTokenIdentifier::<StaticApi>::dcdt(TokenId::from("DCDT-00003")),
        TokenId::<StaticApi>::from("DCDT-00003").into_legacy()
    );
    assert_ne!(
        RewaOrDcdtTokenIdentifier::<StaticApi>::rewa(),
        TokenId::<StaticApi>::from("ANYTHING-1234").into_legacy()
    );
    assert_eq!(
        RewaOrDcdtTokenIdentifier::<StaticApi>::rewa(),
        TokenId::<StaticApi>::from("REWA").into_legacy()
    );
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
        token_id!(b"ALC-6258d2"),
        TokenId::<StaticApi>::from("ALC-6258d2")
    );
}

#[test]
fn test_token_id_to_string() {
    assert_eq!(
        TokenId::<StaticApi>::from("ALC-6258d2").to_string(),
        "ALC-6258d2"
    );
    assert_eq!(
        TokenId::<StaticApi>::from("REWA-00000").to_string(),
        "REWA-00000"
    );
    assert_eq!(
        RewaOrDcdtTokenIdentifier::<StaticApi>::rewa().to_string(),
        "REWA"
    );
    assert_eq!(
        RewaOrDcdtTokenIdentifier::<StaticApi>::dcdt(TokenId::from("REWAORDCDT-00001")).to_string(),
        "REWAORDCDT-00001"
    );
    assert_eq!(
        DcdtTokenIdentifier::<StaticApi>::from_dcdt_bytes("DCDT-00001").to_string(),
        "DCDT-00001"
    );
}
