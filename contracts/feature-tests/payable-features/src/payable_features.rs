#![no_std]
#![allow(clippy::type_complexity)]

dharitri_sc::imports!();

/// Contract that only tests the call value features,
/// i.e. the framework/Andes functionality for accepting REWA and DCDT payments.
#[dharitri_sc::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[view]
    #[payable("*")]
    fn echo_call_value(
        &self,
    ) -> MultiValue2<BigUint, ManagedVec<Self::Api, DcdtTokenPayment<Self::Api>>> {
        (
            self.call_value().rewa_value().clone_value(),
            self.call_value().all_dcdt_transfers().clone_value(),
        )
            .into()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_multiple(
        &self,
        #[payment_multi] payments: ManagedRef<'static, ManagedVec<DcdtTokenPayment<Self::Api>>>,
    ) -> ManagedVec<DcdtTokenPayment<Self::Api>> {
        payments.clone_value()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_3(&self) -> MultiValue3<DcdtTokenPayment, DcdtTokenPayment, DcdtTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_dcdt();
        (payment_a, payment_b, payment_c).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_1(
        &self,
        #[payment_amount] payment: BigUint,
        #[payment_token] token: RewaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let token = self.call_value().rewa_or_single_dcdt().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_3(
        &self,
        #[payment_token] token: RewaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().rewa_or_single_dcdt();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_4(&self) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().rewa_or_single_dcdt();
        (payment.amount, payment.token_identifier).into()
    }

    #[endpoint]
    #[payable("REWA")]
    fn payable_rewa_1(
        &self,
        #[payment_token] token: RewaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().rewa_value().clone_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("REWA")]
    fn payable_rewa_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let token = self.call_value().rewa_or_single_dcdt().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("REWA")]
    fn payable_rewa_3(
        &self,
        #[payment_token] token: RewaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().rewa_value().clone_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("REWA")]
    fn payable_rewa_4(&self) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().rewa_value();
        let token = self.call_value().rewa_or_single_dcdt().token_identifier;
        (payment.clone_value(), token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_1(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: RewaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let token = self.call_value().single_dcdt().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: RewaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().single_dcdt();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().single_dcdt().amount;
        let token = self.call_value().single_dcdt().token_identifier;
        (payment, token).into()
    }
}
