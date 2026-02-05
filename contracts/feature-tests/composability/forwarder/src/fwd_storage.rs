dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait ForwarderStorageModule {
    #[view(lastIssuedToken)]
    #[storage_mapper("lastIssuedToken")]
    fn last_issued_token(&self) -> SingleValueMapper<DcdtTokenIdentifier>;

    #[view(lastErrorMessage)]
    #[storage_mapper("lastErrorMessage")]
    fn last_error_message(&self) -> SingleValueMapper<ManagedBuffer>;
}
