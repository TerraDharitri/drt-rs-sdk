#![no_std]
#![allow(clippy::type_complexity)]

use dharitri_sc::imports::*;

pub mod payable_features_proxy;

/// Contract that only tests the call value features,
/// i.e. the framework/Andes functionality for accepting REWA and DCDT payments.
#[dharitri_sc::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[view]
    #[payable("*")]
    fn payable_legacy_rewa_dcdt(&self) -> MultiValue2<BigUint, ManagedVec<DcdtTokenPayment>> {
        (
            self.call_value().rewa_direct_non_strict().clone_value(),
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
        payments.clone()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_all_transfers(&self) -> ManagedVec<RewaOrDcdtTokenPayment> {
        self.call_value().all_transfers().clone()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_all(&self) -> ManagedVec<Payment> {
        self.call_value().all().clone()
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
    fn payment_array_rewa_or_dcdt_3(
        &self,
    ) -> MultiValue3<RewaOrDcdtTokenPayment, RewaOrDcdtTokenPayment, RewaOrDcdtTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_rewa_or_dcdt();
        (payment_a.clone(), payment_b.clone(), payment_c.clone()).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_3(&self) -> MultiValue3<Payment, Payment, Payment> {
        let [payment_a, payment_b, payment_c] = self.call_value().array();
        (payment_a.clone(), payment_b.clone(), payment_c.clone()).into()
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
    #[payable]
    fn payable_any_5(&self) -> OptionalValue<PaymentMultiValue> {
        optional_payment_to_multi_value(self.call_value().single_optional())
    }

    #[endpoint]
    #[payable("REWA")]
    fn payable_rewa_1(
        &self,
        #[payment_token] token: RewaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().rewa().clone();
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
        let payment = self.call_value().rewa().clone();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("REWA")]
    fn payable_rewa_4(&self) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().rewa();
        let token = self.call_value().rewa_or_single_dcdt().token_identifier;
        (payment.clone(), token).into()
    }

    #[endpoint]
    #[payable("REWA")]
    fn payable_rewa_5(&self) -> OptionalValue<PaymentMultiValue> {
        optional_payment_to_multi_value(self.call_value().single_optional())
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
    ) -> MultiValue2<BigUint, DcdtTokenIdentifier> {
        let token = self.call_value().single_dcdt().token_identifier.clone();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: RewaOrDcdtTokenIdentifier,
    ) -> MultiValue2<BigUint, RewaOrDcdtTokenIdentifier> {
        let payment = self.call_value().single_dcdt();
        (payment.amount.clone(), token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiValue2<BigUint, DcdtTokenIdentifier> {
        let payment = self.call_value().single_dcdt().amount.clone();
        let token = self.call_value().single_dcdt().token_identifier.clone();
        (payment, token).into()
    }
}

fn optional_payment_to_multi_value<M>(
    opt_payment: Option<Ref<'static, Payment<M>>>,
) -> OptionalValue<PaymentMultiValue<M>>
where
    M: ManagedTypeApi,
{
    if let Some(payment) = opt_payment {
        OptionalValue::Some(payment.clone().into_multi_value())
    } else {
        OptionalValue::None
    }
}
