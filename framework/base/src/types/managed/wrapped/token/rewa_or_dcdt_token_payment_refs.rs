use crate::{
    api::ManagedTypeApi,
    types::{BigUint, RewaOrDcdtTokenIdentifier, RewaOrDcdtTokenPayment, DcdtTokenPaymentRefs},
};

/// Similar to `RewaOrDcdtTokenPayment`, but only contains references.
pub struct RewaOrDcdtTokenPaymentRefs<'a, M: ManagedTypeApi> {
    pub token_identifier: &'a RewaOrDcdtTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: &'a BigUint<M>,
}

impl<'a, M: ManagedTypeApi> RewaOrDcdtTokenPaymentRefs<'a, M> {
    pub fn new(
        token_identifier: &'a RewaOrDcdtTokenIdentifier<M>,
        token_nonce: u64,
        amount: &'a BigUint<M>,
    ) -> RewaOrDcdtTokenPaymentRefs<'a, M> {
        RewaOrDcdtTokenPaymentRefs {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn to_owned_payment(&self) -> RewaOrDcdtTokenPayment<M> {
        RewaOrDcdtTokenPayment {
            token_identifier: self.token_identifier.clone(),
            token_nonce: self.token_nonce,
            amount: self.amount.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.amount == &0u32
    }

    pub fn map_rewa_or_dcdt<Context, D, F, U>(self, context: Context, for_rewa: D, for_dcdt: F) -> U
    where
        D: FnOnce(Context, &BigUint<M>) -> U,
        F: FnOnce(Context, DcdtTokenPaymentRefs<M>) -> U,
    {
        self.token_identifier.map_ref_or_else(
            context,
            |context| for_rewa(context, self.amount),
            |context, token_identifier| {
                for_dcdt(
                    context,
                    DcdtTokenPaymentRefs::new(token_identifier, self.token_nonce, self.amount),
                )
            },
        )
    }
}
