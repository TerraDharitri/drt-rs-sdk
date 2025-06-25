use crate::{
    api::{CallValueApi, ErrorApi, ManagedTypeApi},
    types::{BigUint, DcdtTokenPayment, DcdtTokenType, ManagedVec, TokenIdentifier},
};

pub struct CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub(crate) api: A,
}

impl<A> CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub(crate) fn new(api: A) -> Self {
        CallValueWrapper { api }
    }

    /// Retrieves the REWA call value from the VM.
    /// Will return 0 in case of an DCDT transfer (cannot have both REWA and DCDT transfer simultaneously).
    pub fn rewa_value(&self) -> BigUint<A> {
        self.api.rewa_value()
    }

    /// Returns all DCDT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just REWA.
    /// Fully managed underlying types, very efficient.
    pub fn all_dcdt_transfers(&self) -> ManagedVec<A, DcdtTokenPayment<A>> {
        self.api.get_all_dcdt_transfers()
    }

    /// Retrieves the DCDT call value from the VM.
    /// Will return 0 in case of an REWA transfer (cannot have both REWA and DCDT transfer simultaneously).
    /// Warning, not tested with multi transfer, use `all_dcdt_transfers` instead!
    pub fn dcdt_value(&self) -> BigUint<A> {
        self.api.dcdt_value()
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for REWA,
    /// but the REWA TokenIdentifier is serialized as `REWA`.
    /// Warning, not tested with multi transfer, use `all_dcdt_transfers` instead!
    pub fn token(&self) -> TokenIdentifier<A> {
        self.api.token()
    }

    /// Returns the nonce of the received DCDT token.
    /// Will return 0 in case of REWA or fungible DCDT transfer.
    /// Warning, not tested with multi transfer, use `all_dcdt_transfers` instead!
    pub fn dcdt_token_nonce(&self) -> u64 {
        self.api.dcdt_token_nonce()
    }

    /// Returns the DCDT token type.
    /// Will return "Fungible" for REWA.
    /// Warning, not tested with multi transfer, use `all_dcdt_transfers` instead!
    pub fn dcdt_token_type(&self) -> DcdtTokenType {
        self.api.dcdt_token_type()
    }

    /// Returns both the call value (either REWA or DCDT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Andes
    /// that might be used in this scenario in the future.
    /// TODO: replace with multi transfer handling everywhere
    pub fn payment_token_pair(&self) -> (BigUint<A>, TokenIdentifier<A>) {
        self.api.payment_token_pair()
    }
}
