use crate::{
    contract_base::TransferExecuteFailed,
    types::{BigUint, ManagedAddress, PaymentRefs, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

impl<Env> TxPayment<Env> for PaymentRefs<'_, Env::Api>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        // amount is NonZeroBigUint
        false
    }

    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.map_rewa_or_dcdt(
            fc,
            |fc, rewa_payment| {
                rewa_payment.perform_transfer_execute_fallible(env, to, gas_limit, fc)
            },
            |fc, dcdt_payment| {
                dcdt_payment.perform_transfer_execute_fallible(env, to, gas_limit, fc)
            },
        )
    }

    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.map_rewa_or_dcdt(
            fc,
            |fc, rewa_payment| rewa_payment.perform_transfer_execute_legacy(env, to, gas_limit, fc),
            |fc, dcdt_payment| dcdt_payment.perform_transfer_execute_legacy(env, to, gas_limit, fc),
        )
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        self.map_rewa_or_dcdt(
            (to, fc, f),
            |(to, fc, f), rewa_payment| rewa_payment.with_normalized(env, from, to, fc, f),
            |(to, fc, f), dcdt_payment| dcdt_payment.with_normalized(env, from, to, fc, f),
        )
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        self.map_rewa_or_dcdt(
            (),
            |(), rewa_payment| TxPayment::<Env>::into_full_payment_data(rewa_payment, env),
            |(), dcdt_payment| TxPayment::<Env>::into_full_payment_data(dcdt_payment, env),
        )
    }
}
