#![no_std]
#![allow(clippy::type_complexity)]

use dharitri_sc::imports::*;

pub mod payable_features_proxy;

/// Contract that only tests the call value features,
/// i.e. the framework/Andes functionality for accepting MOA and DCDT payments.
#[dharitri_sc::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[view]
    #[payable("*")]
    fn echo_call_value_legacy(&self) -> MultiValue2<BigUint, ManagedVec<DcdtTokenPayment>> {
        (
            self.call_value().moa_direct_non_strict().clone_value(),
            self.call_value().all_dcdt_transfers().clone_value(),
        )
            .into()
    }

    #[view]
    #[payable("*")]
    fn echo_call_value(&self) -> ManagedVec<MoaOrDcdtTokenPayment> {
        self.call_value().all_transfers().clone()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_multiple(
        &self,
        #[payment_multi] payments: ManagedRef<'static, ManagedVec<DcdtTokenPayment<Self::Api>>>,
    ) -> ManagedVec<DcdtTokenPayment<Self::Api>> {
        payments.clone()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_all_transfers(&self) -> ManagedVec<MoaOrDcdtTokenPayment> {
        self.call_value().all_transfers().clone()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_dcdt_3(
        &self,
    ) -> MultiValue3<DcdtTokenPayment, DcdtTokenPayment, DcdtTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_dcdt();
        (payment_a.clone(), payment_b.clone(), payment_c.clone()).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_moa_dcdt_3(
        &self,
    ) -> MultiValue3<MoaOrDcdtTokenPayment, MoaOrDcdtTokenPayment, MoaOrDcdtTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_moa_or_dcdt();
        (payment_a.clone(), payment_b.clone(), payment_c.clone()).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_1(
        &self,
        #[payment_amount] payment: BigUint,
        #[payment_token] token: MoaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        let token = self.call_value().moa_or_single_dcdt().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_3(
        &self,
        #[payment_token] token: MoaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        let payment = self.call_value().moa_or_single_dcdt();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_4(&self) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        let payment = self.call_value().moa_or_single_dcdt();
        (payment.amount, payment.token_identifier).into()
    }

    #[endpoint]
    #[payable("MOA")]
    fn payable_moa_1(
        &self,
        #[payment_token] token: MoaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        let payment = self.call_value().moa().clone();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOA")]
    fn payable_moa_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        let token = self.call_value().moa_or_single_dcdt().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOA")]
    fn payable_moa_3(
        &self,
        #[payment_token] token: MoaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        let payment = self.call_value().moa().clone();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOA")]
    fn payable_moa_4(&self) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        let payment = self.call_value().moa();
        let token = self.call_value().moa_or_single_dcdt().token_identifier;
        (payment.clone(), token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_1(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: MoaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let token = self.call_value().single_dcdt().token_identifier.clone();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: MoaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, MoaOrDcdtTokenIdentifier> {
        let payment = self.call_value().single_dcdt();
        (payment.amount.clone(), token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().single_dcdt().amount.clone();
        let token = self.call_value().single_dcdt().token_identifier.clone();
        (payment, token).into()
    }
}
