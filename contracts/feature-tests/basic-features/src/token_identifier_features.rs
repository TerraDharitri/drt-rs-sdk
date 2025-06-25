numbat_wasm::imports!();

#[numbat_wasm::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_rewa(&self) -> TokenIdentifier {
        TokenIdentifier::rewa()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, bytes: &[u8]) -> bool {
        TokenIdentifier::from(bytes).is_valid_dcdt_identifier()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_dcdt_identifier()
    }
}
