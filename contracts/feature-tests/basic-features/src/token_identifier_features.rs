dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_moa(&self) -> MoaOrDcdtTokenIdentifier {
        MoaOrDcdtTokenIdentifier::moa()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, token_id: MoaOrDcdtTokenIdentifier) -> bool {
        token_id.is_valid()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_dcdt_identifier()
    }
}
