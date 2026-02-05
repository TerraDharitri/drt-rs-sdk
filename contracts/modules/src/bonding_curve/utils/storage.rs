dharitri_sc::imports!();
dharitri_sc::derive_imports!();

use super::structs::TokenOwnershipData;

#[dharitri_sc::module]
pub trait StorageModule {
    #[storage_mapper("token_details")]
    fn token_details(
        &self,
        token: &DcdtTokenIdentifier,
    ) -> SingleValueMapper<TokenOwnershipData<Self::Api>>;

    #[storage_mapper("bonding_curve")]
    fn bonding_curve(&self, token: &DcdtTokenIdentifier) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("owned_tokens")]
    fn owned_tokens(&self, owner: &ManagedAddress) -> SetMapper<DcdtTokenIdentifier>;

    #[storage_mapper("nonce_amount")]
    fn nonce_amount(
        &self,
        identifier: &DcdtTokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<BigUint>;
}
