use super::{ErrorApi, ManagedTypeApi};
use crate::{
    err_msg,
    types::{BigUint, DcdtTokenPayment, DcdtTokenType, ManagedVec, TokenIdentifier},
};

pub trait CallValueApi: ManagedTypeApi + ErrorApi + Sized {
    fn check_not_payable(&self);

    /// Retrieves the REWA call value from the VM.
    /// Will return 0 in case of an DCDT transfer (cannot have both REWA and DCDT transfer simultaneously).
    fn rewa_value(&self) -> BigUint<Self>;

    /// Retrieves the DCDT call value from the VM.
    /// Will return 0 in case of an REWA transfer (cannot have both REWA and DCDT transfer simultaneously).
    fn dcdt_value(&self) -> BigUint<Self>;

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for REWA,
    /// but the REWA TokenIdentifier is serialized as `REWA`.
    fn token(&self) -> TokenIdentifier<Self>;

    /// Returns the nonce of the received DCDT token.
    /// Will return 0 in case of REWA or fungible DCDT transfer.
    fn dcdt_token_nonce(&self) -> u64;

    /// Returns the DCDT token type.
    /// Will return "Fungible" for REWA.
    fn dcdt_token_type(&self) -> DcdtTokenType;

    /// Will return the REWA call value,
    /// but also fail with an error if DCDT is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_rewa(&self) -> BigUint<Self> {
        if !self.token().is_rewa() {
            self.signal_error(err_msg::NON_PAYABLE_FUNC_DCDT);
        }
        self.rewa_value()
    }

    /// Will return the DCDT call value,
    /// but also fail with an error if REWA or the wrong DCDT token is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_dcdt(&self, token: &[u8]) -> BigUint<Self> {
        if self.token().as_managed_buffer() != token {
            self.signal_error(err_msg::BAD_TOKEN_PROVIDED);
        }
        self.dcdt_value()
    }

    /// Returns both the call value (either REWA or DCDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Andes
    /// that might be used in this scenario in the future.
    fn payment_token_pair(&self) -> (BigUint<Self>, TokenIdentifier<Self>) {
        let token = self.token();
        if token.is_rewa() {
            (self.rewa_value(), token)
        } else {
            (self.dcdt_value(), token)
        }
    }

    fn dcdt_num_transfers(&self) -> usize;

    fn dcdt_value_by_index(&self, index: usize) -> BigUint<Self>;

    fn token_by_index(&self, index: usize) -> TokenIdentifier<Self>;

    fn dcdt_token_nonce_by_index(&self, index: usize) -> u64;

    fn dcdt_token_type_by_index(&self, index: usize) -> DcdtTokenType;

    fn get_all_dcdt_transfers(&self) -> ManagedVec<Self, DcdtTokenPayment<Self>> {
        let num_transfers = self.dcdt_num_transfers();
        let mut transfers = ManagedVec::new(self.clone());

        for i in 0..num_transfers {
            let token_type = self.dcdt_token_type_by_index(i);
            let token_identifier = self.token_by_index(i);
            let token_nonce = self.dcdt_token_nonce_by_index(i);
            let amount = self.dcdt_value_by_index(i);

            transfers.push(DcdtTokenPayment::<Self> {
                token_type,
                token_identifier,
                token_nonce,
                amount,
            });
        }

        transfers
    }
}
